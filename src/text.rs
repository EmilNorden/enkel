use sdl2::render::Texture;

pub struct Text<'r> {
    texture: Texture<'r>,
    width: u32,
    height: u32,
}