use std::path::{Path, PathBuf};
use std::time::Instant;
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;
use crate::content_loader::ContentLoader;
use crate::game_time::GameTime;
use crate::renderer::Renderer;
use crate::State;

pub trait Game {
    fn new(context: &GameContext) -> Self;
    fn load_content(&mut self, context: &mut GameContext);
    fn update(&mut self, context: &mut GameContext, time: GameTime);
    fn draw<'a, 'b>(&'a self, renderer: &'b mut (dyn Renderer<'a> + 'b));
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
        self.base_content_path = path.as_ref().into();
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
            &state.device,
            &state.queue,
            &state.texture_bind_group_layout,
        ).unwrap();

        let mut game = G::new(&context);

        let game_timer = Instant::now();
        let mut frame_timer = Instant::now();

        'game_loop: loop {
            self.event_loop.run(move |event, _, control_flow| {
                *control_flow = ControlFlow::Poll;


                match event {
                    Event::RedrawRequested(_) => {
                        state.update();

                        let mut encoder = state.create_encoder();
                        let frame = state.swap_chain
                            .get_current_frame().unwrap().output;
                        let mut render_pass = state.begin_render(&mut encoder, &frame).unwrap(); // TODO: Use below match here

                        game.draw(&mut render_pass);
                        drop(render_pass);
                        state.end_render(encoder);

                        // game.draw(&render_pass, &mut context);
                        /*match state.render() {
                            Ok(_) => {}
                            Err(wgpu::SwapChainError::Lost) => state.resize(state.size),
                            Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                            Err(e) => eprintln!("{:?}", e),
                        }*/
                    }
                    Event::MainEventsCleared => {
                        window.request_redraw();
                    }
                    Event::WindowEvent {
                        ref event,
                        window_id
                    } if window_id == window.id() => if !state.input(event) {
                        match event {
                            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                            WindowEvent::Resized(physical_size) => {
                                state.resize(*physical_size);
                            }
                            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                                state.resize(**new_inner_size);
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
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            });

            let game_time = GameTime::new(game_timer.elapsed(), frame_timer.elapsed());
            frame_timer = Instant::now();
        }
    }
}