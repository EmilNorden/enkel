use enkel::game::{Game, GameContext, GameHostBuilder};
use std::mem::size_of;
use std::time::{Duration, Instant};
use std::ops::Rem;
use std::rc::Rc;
use enkel::game_time::GameTime;
use enkel::model::Material;
use enkel::renderer::Renderer;

pub struct MyGame {
    apple: enkel::model::Model,
}

impl Game for MyGame {
    fn new(context: &GameContext) -> Self {
        let apple = context.content().load_model("apple/apple.obj").unwrap();
        MyGame {
            apple,
        }
    }

    fn load_content(&mut self, context: &mut GameContext) {}

    fn update(&mut self, context: &mut GameContext, time: GameTime) {
        if time.game_duration().as_secs().rem(2) == 1 {
            // context.graphics_mut().set_clear_color(glm::vec3(1.0, 0.0, 0.0));
        } else {
            // context.graphics_mut().set_clear_color(glm::vec3(0.0, 0.0, 1.0));
        }
    }

    fn draw<'a, 'b>(&'a self, renderer: &'b mut (dyn Renderer<'a> + 'b)){
        let mesh = &self.apple.meshes[0];
        let material = &self.apple.materials[mesh.material];
        renderer.draw_mesh(mesh, material);
    }
}

fn main() {
    env_logger::init();
    GameHostBuilder::new()
        .with_content_path("/Users/emilnorden/models")
        .with_name("Test game")
        .build()
        .unwrap()
        .run::<MyGame>();
}
