use std::collections::HashMap;
use std::ops::Range;
use std::path::Path;
use crate::texture;
use anyhow::*;
use bytemuck::Pod;
use russimp::scene::{PostProcess, PostProcessSteps};
use russimp::texture::{DataContent, TextureType};
use russimp::Vector3D;
use wgpu::{Device, Queue};
use wgpu::util::DeviceExt;
use crate::texture::Texture;

pub trait Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelVertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
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
    pub bind_group: wgpu::BindGroup,
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
    ) -> Result<Model> {
        /*let scene = russimp::scene::Scene::from_file(path.as_ref().to_str().unwrap(), vec![

            PostProcess::CalculateTangentSpace, PostProcess::Triangulate,
            PostProcess::GenerateSmoothNormals, PostProcess::ImproveCacheLocality,
            PostProcess::RemoveRedundantMaterials, PostProcess::OptimizeMeshes,
            PostProcess::FixInfacingNormals, PostProcess::FixOrRemoveInvalidData,
            PostProcess::OptimizeGraph, PostProcess::JoinIdenticalVertices,
            PostProcess::FindInstances, PostProcess::GenerateUVCoords, PostProcess::SortByPrimitiveType,
        ]).expect("Unable to read file");*/

        let scene = russimp::scene::Scene::from_file(path.as_ref().to_str().unwrap(), vec![
            PostProcess::RemoveRedundantMaterials,
            PostProcess::CalculateTangentSpace, PostProcess::Triangulate,
            PostProcess::FindInstances, PostProcess::GenerateUVCoords, PostProcess::SortByPrimitiveType,
        ]).expect("Unable to read file");

        let parent_folder = path.as_ref()
            .parent()
            .context("Directory has no parent")?;

        let mut materials = Vec::new();
        for (i, mat) in scene.materials.into_iter().enumerate(){
            materials.push(Model::create_material(device, queue, layout,mat, parent_folder)?);
        }

        let mut meshes = Vec::new();
        for mesh in scene.meshes {
            meshes.push(Self::create_mesh(device, mesh)?);
        }

        Ok(Model {
            meshes,
            materials
        })
    }

    fn create_mesh(device: &Device, ai_mesh: russimp::mesh::Mesh) -> Result<Mesh> {

        let texture_coords = ai_mesh.texture_coords[0].as_ref().unwrap();

        let mut vertices = Vec::with_capacity(ai_mesh.vertices.len());
        for i in 0..ai_mesh.vertices.len() {
            vertices.push(ModelVertex {
                position: [ai_mesh.vertices[i].x, ai_mesh.vertices[i].y, ai_mesh.vertices[i].z],
                tex_coords: [texture_coords[i].x, texture_coords[i].y],
                normal: [ai_mesh.normals[i].x, ai_mesh.normals[i].y, ai_mesh.normals[i].z],
            });
        }

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{:?} Vertex Buffer", ai_mesh.name)),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsage::VERTEX,
            }
        );

        // Since we specify PostProcess::Triangulate when loading the model, we can safely assume 3 indices per face
        let mut indices = Vec::with_capacity(ai_mesh.faces.len() * 3);
        for face in &ai_mesh.faces {
            for i in &face.0 {
                indices.push(*i);
            }
        }

        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some(&format!("{:?} Index Buffer", ai_mesh.name)),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsage::INDEX,
            }
        );

        Ok(Mesh {
            name: ai_mesh.name,
            vertex_buffer,
            index_buffer,

            num_elements: ai_mesh.faces.len() as u32 * 3,
            material: ai_mesh.material_index as usize,
        })
    }

    fn create_material(device: &Device, queue: &Queue, layout: &wgpu::BindGroupLayout, mut ai_material: russimp::material::Material, folder: &Path) -> Result<Material> {
        let diffuse_textures = ai_material.textures.remove(&TextureType::Diffuse).unwrap();
        let first_diffuse_texture = diffuse_textures.first().unwrap();

        let texture = Model::create_texture(device, queue, &first_diffuse_texture, folder)?;

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
            ],
            label: None,
        });

        Ok(Material {
            diffuse_texture: texture,
            name: "New material".to_string(),
            bind_group,
        })
    }

    fn create_texture(device: &Device, queue: &Queue, ai_texture: &russimp::texture::Texture, folder: &Path) -> Result<Texture> {
        /*let data = ai_texture.data.as_ref().expect("Texture holds no data!");
        let bytes = match data {
            DataContent::Texel(texels) =>
                texels.iter().flat_map(|t| [t.r, t.g, t.b, t.a])
                    .collect::<Vec<_>>(),
            DataContent::Bytes(bytes) => bytes.to_vec() // TODO: This will copy the vec. Rewrite to avoid
        };

        Texture::from_pixels(device, queue, &bytes, ai_texture.width, ai_texture.height, None)*/
        Texture::load(device, queue, folder.join(&ai_texture.path))
    }
}

pub trait DrawModel<'a> {
    fn draw_mesh(&mut self, mesh: &'a Mesh, material: &'a Material);
    fn draw_mesh_instanced(
        &mut self,
        mesh: &'a  Mesh,
        material: &'a Material,
        instances: Range<u32>);
}

impl<'a, 'b> DrawModel<'b> for wgpu::RenderPass<'a>
where 'b: 'a{
    fn draw_mesh(&mut self, mesh: &'b Mesh, material: &'b Material) {
        self.draw_mesh_instanced(mesh,  material, 0..1);
    }

    fn draw_mesh_instanced(&mut self, mesh: &'b Mesh, material: &'b Material, instances: Range<u32>) {
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        self.set_bind_group(0, &material.bind_group, &[]);
        self.draw_indexed(0..mesh.num_elements, 0, instances);
    }
}