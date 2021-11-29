use crate::types::Sentinel;

#[derive(PartialEq)]
pub enum WindowMode {
    Windowed,
    Fullscreen,
}

pub struct GraphicsContext {
    resolution: Sentinel<(u32, u32)>,
    window_mode: Sentinel<WindowMode>,
}

impl GraphicsContext {
    pub fn set_resolution(&mut self, width: u32, height: u32) {
        self.resolution.set((width, height));
    }

    pub fn set_window_mode(&mut self, mode: WindowMode) {
        self.window_mode.set(mode);
    }
}