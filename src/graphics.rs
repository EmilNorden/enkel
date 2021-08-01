use sdl2::pixels::Color;
use sdl2::render::{WindowCanvas, TextureQuery};
use crate::font::Font;
use sdl2::rect::Rect;

static SCREEN_WIDTH: u32 = 800;
static SCREEN_HEIGHT: u32 = 600;

// handle the annoying Rect i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
    )
);

pub struct GraphicsContext {
    canvas: WindowCanvas,
}

impl GraphicsContext {
    pub fn new(canvas: WindowCanvas) -> Self {
        GraphicsContext {
            canvas
        }
    }

    pub fn set_clear_color(&mut self, color: glm::Vec3) {
        self.canvas.set_draw_color(
            Color::RGBA((color.x * 255.0) as u8, (color.y * 255.0) as u8, (color.z * 255.0) as u8, 255));
    }

    pub fn clear(&mut self) {
        self.canvas.clear();
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

    // Scale fonts to a reasonable size when they're too big (though they might look less smooth)
    fn get_centered_rect(rect_width: u32, rect_height: u32, cons_width: u32, cons_height: u32) -> Rect {
        let wr = rect_width as f32 / cons_width as f32;
        let hr = rect_height as f32 / cons_height as f32;

        let (w, h) = if wr > 1f32 || hr > 1f32 {
            if wr > hr {
                println!("Scaling down! The text will look worse!");
                let h = (rect_height as f32 / wr) as i32;
                (cons_width as i32, h)
            } else {
                println!("Scaling down! The text will look worse!");
                let w = (rect_width as f32 / hr) as i32;
                (w, cons_height as i32)
            }
        } else {
            (rect_width as i32, rect_height as i32)
        };

        let cx = (SCREEN_WIDTH as i32 - w) / 2;
        let cy = (SCREEN_HEIGHT as i32 - h) / 2;
        rect!(cx, cy, w, h)
    }

    pub fn draw_string(&mut self, string: &str, x: f32, y: f32, font: &Font) {
        let texture_creator = self.canvas.texture_creator();
        let surface = font.ttf_font().render(string)
            .solid(Color::RGB(255, 0, 0))
            .unwrap();

        let texture = texture_creator.create_texture_from_surface(&surface).unwrap();

        let TextureQuery { width, height, .. } = texture.query();

        // If the example text is too big for the screen, downscale it (and center irregardless)
        let padding = 64;
        let target = GraphicsContext::get_centered_rect(
            width,
            height,
            SCREEN_WIDTH - padding,
            SCREEN_HEIGHT - padding,
        );

        self.canvas.copy(&texture, None, Some(target));
    }
}