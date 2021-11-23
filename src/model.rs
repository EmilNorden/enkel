use std::collections::HashMap;
use std::path::Path;
use crate::texture;
use anyhow::*;
use russimp::scene::{PostProcess, PostProcessSteps};
use russimp::texture::{DataContent, TextureType};
use wgpu::{Device, Queue};
use crate::texture::Texture;

pub trait Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelVertex {
    position: [f32; 3],
    tex_coords: [f32; 3],
    normal: [f32; 3],
}

impl Vertex for ModelVertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<ModelVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }
}

pub struct Model {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
}

pub struct Material {
    pub name: String,
    pub diffuse_texture: texture::Texture,
}

pub struct Mesh {
    pub name: String,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_elements: u32,
    pub material: usize,
}

impl Model {
    pub fn load<P: AsRef<Path>>(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        layout: &wgpu::BindGroupLayout,
        path: P,
    ) -> Result<i32> {
        let scene = russimp::scene::Scene::from_file(path.as_ref().to_str().unwrap(), vec![
            PostProcess::CalculateTangentSpace, PostProcess::Triangulate,
            PostProcess::GenerateSmoothNormals, PostProcess::ImproveCacheLocality,
            PostProcess::RemoveRedundantMaterials, PostProcess::OptimizeMeshes,
            PostProcess::FixInfacingNormals, PostProcess::FixOrRemoveInvalidData,
            PostProcess::OptimizeGraph, PostProcess::JoinIdenticalVertices,
            PostProcess::FindInstances, PostProcess::GenerateUVCoords, PostProcess::SortByPrimitiveType,
        ]).expect("Unable to read file");

        let parent_folder = path.as_ref()
            .parent()
            .context("Directory has no parent");

        let mut materials = HashMap::new();
        for (i, mat) in scene.materials.into_iter().enumerate(){
            materials.insert(i, Model::create_material(device, queue,mat));
        }

        Ok(32)
    }

    fn create_material(device: &Device, queue: &Queue, mut ai_material: russimp::material::Material) -> Result<Material> {
        let diffuse_textures = ai_material.textures.remove(&TextureType::Diffuse).unwrap();
        let first_diffuse_texture = diffuse_textures.first().unwrap();


        Ok(Material {
            diffuse_texture: Model::create_texture(device, queue, &first_diffuse_texture)?,
            name: "New material".to_string(),
        })
    }

    fn create_texture(device: &Device, queue: &Queue, ai_texture: &russimp::texture::Texture) -> Result<Texture> {
        let data = ai_texture.data.as_ref().expect("Texture holds no data!");
        let bytes = match data {
            DataContent::Texel(texels) =>
                texels.iter().flat_map(|t| [t.r, t.g, t.b, t.a])
                    .collect::<Vec<_>>(),
            DataContent::Bytes(bytes) => bytes.to_vec() // TODO: This will copy the vec. Rewrite to avoid
        };

        Texture::from_pixels(device, queue, &bytes, ai_texture.width, ai_texture.height, None)
    }
}