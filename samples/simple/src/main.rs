use enkel::{GameContext, GameHostBuilder};
use enkel::game::Game;
use std::mem::size_of;
use std::time::{Duration, Instant};
use std::ops::Rem;
use std::rc::Rc;
use enkel::game_time::GameTime;

pub struct MyGame{
    apple: Option<enkel::model::Model>,
}

impl MyGame {
    pub fn new() -> Self {
        MyGame {
            apple: None,
        }
    }
}

impl Game for MyGame {
    fn load_content(&mut self, context: &mut GameContext) {
    }

    fn update(&mut self, context: &mut GameContext, time: GameTime) {
        if time.game_duration().as_secs().rem(2) == 1 {
            // context.graphics_mut().set_clear_color(glm::vec3(1.0, 0.0, 0.0));
        } else {
            // context.graphics_mut().set_clear_color(glm::vec3(0.0, 0.0, 1.0));
        }
    }

    fn draw(&self, context: &mut GameContext, time: GameTime) {

        // context.graphics_mut().clear();
    }
}



fn main() {
    env_logger::init();
    let game = MyGame::new();
    GameHostBuilder::new()
        .with_content_path("/Users/emilnorden")
        .with_name("Test game")
        .build()
        .unwrap()
        .run::<MyGame>(game);
}
