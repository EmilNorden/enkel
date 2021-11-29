use std::path::Path;
use crate::content_loader::ContentLoader;
use crate::game_time::GameTime;

pub trait Game {
    fn new(context: &GameContext) -> Self;
    fn load_content(&mut self, context: &mut GameContext);
    fn update(&mut self, context: &mut GameContext, time: GameTime);
    fn draw(&self, context: &mut GameContext, time: GameTime);
}

pub struct GameContext<'a> {
    name: String,
    content_loader: ContentLoader<'a>,
}

impl<'a> GameContext<'a> {
    pub(crate) fn create(
        name: &str,
        base_content_path: Box<Path>,
        device: &'a wgpu::Device,
        queue: &'a wgpu::Queue,
        texture_bind_group_layout: &'a wgpu::BindGroupLayout)
        -> Result<Self, String> {

        let content_loader = ContentLoader::new_with_defaults(
            base_content_path,
            device,
            queue,
            texture_bind_group_layout,
        );

        Ok(GameContext {
            name: name.to_owned(),
            content_loader,
        })
    }

    pub fn content(&self) -> &ContentLoader { &self.content_loader }
}