pub(crate) mod assimp_loader;

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
use crate::content_loader::LoadError;
use crate::texture::Texture;

pub(crate) trait Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub(crate) struct ModelVertex {
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

impl Model {
    pub fn new(meshes: Vec<Mesh>, materials: Vec<Material>) -> Self {
        Self {
            meshes,
            materials
        }
    }
}

pub struct Material {
    pub name: String,
    pub diffuse_texture: texture::Texture,
    pub bind_group: wgpu::BindGroup,
}

impl Material {
    pub fn new(name: String, diffuse_texture: texture::Texture, bind_group: wgpu::BindGroup) -> Self {
        Self {
            name,
            diffuse_texture,
            bind_group
        }
    }
}

pub struct Mesh {
    pub name: String,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_elements: u32,
    pub material: usize,
}

impl Mesh {
    pub fn new(name: String, vertex_buffer: wgpu::Buffer, index_buffer: wgpu::Buffer, num_elements: u32, material: usize) -> Self {
        Mesh {
            name,
            vertex_buffer,
            index_buffer,
            num_elements,
            material
        }
    }
}
/*
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
*/
pub trait ModelLoader {
    fn load(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        layout: &wgpu::BindGroupLayout,
        path: &Path,
    ) -> Result<Model, LoadError>;

    fn can_handle_extension(&self, path: &Path) -> bool;
}

