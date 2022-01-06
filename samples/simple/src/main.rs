use enkel::game::{Game, GameContext, GameHostBuilder};
use std::mem::size_of;
use std::time::{Duration, Instant};
use std::ops::Rem;
use std::rc::Rc;
use enkel::game_time::GameTime;
use enkel::input::input_system::{AxisBinding, AxisBindingDefinition, AxisDefinition};
use enkel::model::Material;
use enkel::renderer::Renderer;
use enkel::VirtualKeyCode;
macro_rules! axis_property {
    ($t:ident) => {
        pub fn $t(&self) -> f32 {
            42.9
        }
    }
}

struct FpsMovement {}

impl FpsMovement {
    axis_property!(hmovement);
}


pub struct MyGame {
    apple: enkel::model::Model,
}

impl Game for MyGame {
    fn new(context: &mut GameContext) -> Self {
        let apple = context.content().load_model("apple/apple.obj").unwrap();


        context.input().create_axis(AxisDefinition {
            name: "forward_movement",
            bindings: vec![
                AxisBindingDefinition {
                    scale: 1.0,
                    binding: AxisBinding::Keyboard(VirtualKeyCode::U),
                }
            ],
        });
        //context.input().register_axis("hmov");
        MyGame {
            apple,
        }
    }

    fn load_content(&mut self, _: &mut GameContext) {}

    fn update(&mut self, ctx: &mut GameContext, time: GameTime) {
        let c = ctx.input().query_axis("forward_movement").unwrap();
        println!("forward_movement: {}", c);
        if time.game_duration().as_secs().rem(2) == 1 {
            // context.graphics_mut().set_clear_color(glm::vec3(1.0, 0.0, 0.0));
        } else {
            // context.graphics_mut().set_clear_color(glm::vec3(0.0, 0.0, 1.0));
        }
    }

    fn draw<'a, 'b>(&'a self, renderer: &'b mut (dyn Renderer<'a> + 'b)) {
        renderer.draw_model_instanced(&self.apple, 0..1);
    }
}

fn main() {
    env_logger::init();
    GameHostBuilder::new()
        .with_content_path("res")
        .with_name("Test game")
        .build()
        .unwrap()
        .run::<MyGame>();
}
