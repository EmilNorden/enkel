use crate::model::Mesh;

pub trait Renderer {
    fn draw_mesh(mesh: &Mesh);
}