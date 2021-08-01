use sdl2::ttf::Sdl2TtfContext;
use crate::font::Font;
use std::path::{Path, PathBuf};
use crate::text::Text;

pub struct FontLoader {
    base_path: PathBuf,
    ttf_context: Sdl2TtfContext,
}

impl FontLoader {
    pub fn new<P: AsRef<Path>>(base_path: P, ttf_context: Sdl2TtfContext) -> Self {
        FontLoader {
            base_path: base_path.as_ref().into(),
            ttf_context,
        }
    }

    pub fn load(&self, name: &str, size: u16) -> Result<Font, String> {
        let font = self.ttf_context.load_font(self.base_path.join(name), size)?;

        Ok(Font::new(font))
    }

}