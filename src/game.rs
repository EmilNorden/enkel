use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::window::Window;
use crate::content_loader::ContentLoader;
use crate::game_time::GameTime;
use crate::input::input_system::InputSystem;
use crate::model::assimp_loader::AssimpModelLoader;
use crate::model::ModelLoader;
use crate::renderer::Renderer;
use crate::State;

pub trait Game {
    fn new(context: &mut GameContext) -> Self;
    fn load_content(&mut self, context: &mut GameContext);
    fn update(&mut self, context: &mut GameContext, time: GameTime);
    fn draw<'a, 'b>(&'a self, renderer: &'b mut (dyn Renderer<'a> + 'b));
}

pub struct GameContext {
    name: String,
    base_path: Box<Path>,
    input_system: InputSystem,
    device_state: State,
    model_loaders: Vec<Box<dyn ModelLoader>>,
}

impl GameContext {
    pub(crate) fn create(
        name: &str,
        base_path: Box<Path>,
        device_state: State)
        -> Result<Self, String> {
        let input_system = InputSystem::new();
        let model_loaders: Vec<Box<dyn ModelLoader>> = vec![
            Box::from(AssimpModelLoader::new())
        ];

        Ok(GameContext {
            name: name.to_owned(),
            base_path,
            input_system,
            device_state,
            model_loaders,
        })
    }

    pub fn input(&mut self) -> &mut InputSystem { &mut self.input_system }

    pub fn content(&self) -> ContentLoader {
        ContentLoader::new(
            self.base_path.clone(),
            &self.model_loaders,
            &self.device_state.device,
            &self.device_state.queue,
            &self.device_state.texture_bind_group_layout)
    }

    pub fn update(&self) {}
}

pub struct GameHost {
    event_loop: EventLoop<()>,
    window: Window,
    state: State,
    base_path: Box<Path>,
}

pub struct GameHostBuilder {
    base_content_path: PathBuf,
    game_name: String,
}


impl GameHostBuilder {
    pub fn new() -> Self {
        GameHostBuilder
        {
            base_content_path: "/".parse().unwrap(),
            game_name: "My Game!".to_string(),
        }
    }

    pub fn with_name(&mut self, name: &str) -> &mut Self {
        self.game_name = name.to_owned();
        self
    }

    pub fn with_content_path<P: AsRef<Path>>(&mut self, path: P) -> &mut Self {
        self.base_content_path = if path.as_ref().is_relative() {
            std::env::current_dir().unwrap().join(path)
        } else {
            path.as_ref().into()
        };
        self
    }

    pub fn build(&self) -> Result<GameHost, String> {
        let event_loop = EventLoop::new();
        let window = Window::new(&event_loop).unwrap();
        let state = pollster::block_on(State::new(&window));

        Ok(GameHost {
            base_path: Box::from(self.base_content_path.as_path()),
            event_loop,
            window,
            state,
        })
    }
}


impl GameHost {
    pub fn run<G: Game + 'static>(mut self) {
        let window = self.window;
        let mut state = self.state;

        let mut context = GameContext::create(
            "My_name",
            self.base_path,
            state
        ).unwrap();
        let mut game = G::new(&mut context);

        let game_timer = Instant::now();
        let mut frame_timer = Instant::now();


        self.event_loop.run_return(|event, _, control_flow| {
            *control_flow = ControlFlow::Poll;


            match event {
                Event::RedrawRequested(_) => {
                    context.device_state.update();

                    let mut encoder = context.device_state.create_encoder();
                    let frame = context.device_state.swap_chain
                        .get_current_frame().unwrap().output;
                    let mut render_pass = context.device_state.begin_render(&mut encoder, &frame).unwrap(); // TODO: Use below match here

                    game.draw(&mut render_pass);
                    drop(render_pass);
                    context.device_state.end_render(encoder);

                    // game.draw(&render_pass, &mut context);
                    /*match state.render() {
                        Ok(_) => {}
                        Err(wgpu::SwapChainError::Lost) => state.resize(state.size),
                        Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        Err(e) => eprintln!("{:?}", e),
                    }*/
                }
                Event::MainEventsCleared => {
                    game.update(&mut context, GameTime::new(
                        Duration::from_secs(0),
                        Duration::from_secs(0)));
                    window.request_redraw();
                }
                Event::WindowEvent {
                    ref event,
                    window_id
                } if window_id == window.id() => if !context.device_state.input(event) {
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            context.device_state.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            context.device_state.resize(**new_inner_size);
                        }
                        WindowEvent::KeyboardInput {
                            input, ..
                        } => {
                            match input {
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                } => *control_flow = ControlFlow::Exit,
                                _ => {
                                    if let Some(keycode) = input.virtual_keycode {
                                        if keycode == VirtualKeyCode::U {
                                            println!("U is pressed!");
                                        }

                                        match input.state {
                                            ElementState::Pressed => context.input().on_key_down(keycode.into()),
                                            ElementState::Released => context.input().on_key_up(keycode.into()),
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                },
                _ => {}
            }
        });
        let game_time = GameTime::new(game_timer.elapsed(), frame_timer.elapsed());
        frame_timer = Instant::now();
    }
}