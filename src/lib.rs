pub mod game_time;
pub mod font;
mod content_loader;
mod font_loader;
mod graphics;
mod text;

use sdl2::Sdl;
use sdl2::event::Event;
use sdl2::video::{Window, GLContext};
use sdl2::ttf::{InitError, Sdl2TtfContext};
use std::io::Error;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use crate::graphics::GraphicsContext;
use crate::font_loader::FontLoader;
use std::path::{Path, PathBuf};
use crate::game_time::GameTime;
use std::time::Instant;

pub trait Newable {
    fn new() -> Self;
}

pub trait Game {
    fn load_content(&mut self, context: &mut GameContext);
    fn update(&mut self, context: &mut GameContext, time: GameTime);
    fn draw(&self, context: &mut GameContext, time: GameTime);
}

pub struct GameContext {
    name: String,
    sdl: Sdl,
    graphics: GraphicsContext,
    fonts: FontLoader,
}

impl GameContext {
    pub(crate) fn create<P: AsRef<Path>>(name: &str, base_content_path: P) -> Result<Self, String> {
        let sdl = sdl2::init()?;
        let video = sdl.video().unwrap();
        let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

        let window = video.window(name, 800, 600)
            .position_centered()
            .opengl()
            .build().unwrap();

        let canvas = window
            .into_canvas()
            .index(GameContext::find_sdl_gl_driver().unwrap())
            .build()
            .unwrap();

        Ok(GameContext {
            sdl,
            name: name.to_owned(),
            graphics: GraphicsContext::new(canvas),
            fonts: FontLoader::new(base_content_path, ttf_context)
        })
    }

    pub fn graphics(&self) -> &GraphicsContext {
        &self.graphics
    }

    pub fn graphics_mut(&mut self) -> &mut GraphicsContext {
        &mut self.graphics
    }

    pub fn fonts(&self) -> &FontLoader {
        &self.fonts
    }

    fn find_sdl_gl_driver() -> Option<u32> {
        for (index, item) in sdl2::render::drivers().enumerate() {
            if item.name == "opengl" {
                return Some(index as u32);
            }
        }
        None
    }
}

pub struct GameHost {
    context: GameContext,
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
            game_name: "My Game!".to_string()
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
        let context = GameContext::create(&self.game_name, self.base_content_path.as_path())?;

        Ok(GameHost {
            context,
        })
    }
}

impl GameHost {

    pub fn run<G: 'static + Game + Newable>(&mut self) {

        let mut game = G::new();
        game.load_content(&mut self.context);

        let mut events = self.context.sdl.event_pump().unwrap();
        let game_timer = Instant::now();
        let mut frame_timer = Instant::now();
        'game_loop: loop {
            for event in events.poll_iter() {
                match event {
                    Event::Quit {..} => {
                        break 'game_loop;
                    },
                    _ => {}
                }
            }

            let game_time = GameTime::new(game_timer.elapsed(), frame_timer.elapsed());
            frame_timer = Instant::now();
            game.update(&mut self.context, game_time);
            game.draw(&mut self.context, game_time);
            self.context.graphics.present();
        }
    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
