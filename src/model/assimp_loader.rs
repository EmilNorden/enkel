use std::path::Path;
use anyhow::{Context, Result};
use russimp::scene::PostProcess;
use russimp::texture::TextureType;
use wgpu::{BindGroupLayout, Device, Queue};
use wgpu::util::DeviceExt;
use crate::model::{Material, Mesh, Model, ModelLoader, ModelVertex};
use crate::texture::Texture;

pub struct AssimpModelLoader {

}

impl AssimpModelLoader {
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

        Ok(Mesh::new(ai_mesh.name, vertex_buffer, index_buffer, ai_mesh.faces.len() as u32 * 3, ai_mesh.material_index as usize))
    }

    fn create_material(device: &Device, queue: &Queue, layout: &wgpu::BindGroupLayout, mut ai_material: russimp::material::Material, folder: &Path) -> Result<Material> {
        let diffuse_textures = ai_material.textures.remove(&TextureType::Diffuse).unwrap();
        let first_diffuse_texture = diffuse_textures.first().unwrap();

        let texture = Self::create_texture(device, queue, &first_diffuse_texture, folder)?;

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

        Ok(Material::new("New material".to_string(), texture, bind_group))
    }

    fn create_texture(device: &Device, queue: &Queue, ai_texture: &russimp::texture::Texture, folder: &Path) -> Result<Texture> {
        Texture::load(device, queue, folder.join(&ai_texture.path))
    }
}

impl ModelLoader for AssimpModelLoader {
    fn load(&self, device: &Device, queue: &Queue, layout: &BindGroupLayout, path: &Path) -> anyhow::Result<Model> {
        /*let scene = russimp::scene::Scene::from_file(path.as_ref().to_str().unwrap(), vec![

            PostProcess::CalculateTangentSpace, PostProcess::Triangulate,
            PostProcess::GenerateSmoothNormals, PostProcess::ImproveCacheLocality,
            PostProcess::RemoveRedundantMaterials, PostProcess::OptimizeMeshes,
            PostProcess::FixInfacingNormals, PostProcess::FixOrRemoveInvalidData,
            PostProcess::OptimizeGraph, PostProcess::JoinIdenticalVertices,
            PostProcess::FindInstances, PostProcess::GenerateUVCoords, PostProcess::SortByPrimitiveType,
        ]).expect("Unable to read file");*/

        let scene = russimp::scene::Scene::from_file(path.to_str().unwrap(), vec![
            PostProcess::RemoveRedundantMaterials,
            PostProcess::CalculateTangentSpace, PostProcess::Triangulate,
            PostProcess::FindInstances, PostProcess::GenerateUVCoords, PostProcess::SortByPrimitiveType,
        ]).expect("Unable to read file");

        let parent_folder = path
            .parent()
            .context("Directory has no parent")?;

        let mut materials = Vec::new();
        for (i, mat) in scene.materials.into_iter().enumerate(){
            materials.push(Self::create_material(device, queue, layout,mat, parent_folder)?);
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

    fn can_handle_extension(&self, path: &Path) -> bool {
        true
    }
}