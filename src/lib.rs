mod graphics;
pub mod font;
mod content_loader;
mod font_loader;
mod text;

use sdl2::Sdl;
use sdl2::event::Event;
use std::time::{Instant, Duration};
use sdl2::video::{Window, GLContext};
use sdl2::ttf::{InitError, Sdl2TtfContext};
use std::io::Error;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use crate::graphics::GraphicsContext;
use crate::font_loader::FontLoader;
use std::path::Path;

#[derive(Copy, Clone)]
pub struct GameTime {
    game_duration: Duration,
    frame_duration: Duration,
    frame_millis: f32,
}

impl GameTime {
    pub fn new(game_duration: Duration, frame_duration: Duration) -> Self {
        let frame_millis: f32 = frame_duration.as_micros() as f32;
        GameTime {
            game_duration,
            frame_duration,
            frame_millis,
        }
    }
}

impl GameTime {
    pub fn game_duration(&self) -> Duration {
        self.game_duration
    }

    pub fn frame_duration(&self) -> Duration {
        self.frame_duration
    }

    pub fn frame_millis(&self) -> f32 {
        self.frame_millis
    }
}

pub trait Game {
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
    game: Box<dyn Game>
}


pub fn create<G: Game, P: AsRef<Path>>(name: &str, base_content_path: P, game: Box<dyn Game>) -> Result<GameHost, String> {
    let context = GameContext::create(name, base_content_path)?;

    Ok(GameHost {
        context,
        game
    })
}

impl GameHost {

    pub fn run(&mut self) {
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
            self.game.update(&mut self.context, game_time);
            self.game.draw(&mut self.context, game_time);
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
