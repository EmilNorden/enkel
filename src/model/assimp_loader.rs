use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use russimp::scene::PostProcess;
use russimp::texture::TextureType;
use wgpu::{BindGroupLayout, Device, Queue};
use wgpu::util::DeviceExt;
use crate::content_loader::LoadError;
use crate::model::{Material, Mesh, Model, ModelLoader, ModelVertex};
use crate::texture::Texture;

pub struct AssimpModelLoader {}

impl AssimpModelLoader {
    pub fn new() -> Self { Self {} }
    fn create_mesh(device: &Device, ai_mesh: russimp::mesh::Mesh) -> Result<Mesh, LoadError> {
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

    fn load_diffuse_texture(
        device: &Device,
        queue: &Queue,
        folder: &Path,
        mut ai_material: russimp::material::Material) -> Result<Texture, LoadError> {

        let path = ai_material.textures.remove(&TextureType::Diffuse)
            .unwrap_or(Vec::new())
            .first()
            .map(|x| folder.join(&x.path))
            .unwrap_or(PathBuf::from("/Users/emilnorden/models/walnut/dioDiffuseMap.png".to_string()));

        let texture = Texture::load(device, queue, folder.join(&path))
            .map_err(|e| LoadError::UnableToReadFile)?;

        Ok(texture)
    }

    fn create_material(
        device: &Device,
        queue: &Queue,
        layout: &wgpu::BindGroupLayout,
        mut ai_material: russimp::material::Material,
        folder: &Path) -> Result<Material, LoadError> {
        let texture = Self::load_diffuse_texture(
            device,
            queue,
            folder,
            ai_material)?;

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
}

impl ModelLoader for AssimpModelLoader {
    fn load(&self, device: &Device, queue: &Queue, layout: &BindGroupLayout, path: &Path) -> Result<Model, LoadError> {
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
            PostProcess::FindInstances, PostProcess::SortByPrimitiveType,
        ]).expect("Unable to read file");
        // , PostProcess::GenerateUVCoords,

        let parent_folder = path
            .parent()
            .ok_or(LoadError::OtherError("Could not find parent folder for file.".to_string()))?;

        let mut materials = Vec::new();
        for (i, mat) in scene.materials.into_iter().enumerate() {
            materials.push(Self::create_material(device, queue, layout, mat, parent_folder)?);
        }

        let mut meshes = Vec::new();
        for mesh in scene.meshes {
            meshes.push(Self::create_mesh(device, mesh)?);
        }

        Ok(Model {
            meshes,
            materials,
        })
    }

    fn can_handle_extension(&self, path: &Path) -> bool {
        true
    }
}