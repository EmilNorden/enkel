pub struct Font<'ttf_module, 'rwops>  {
    ttf_font: sdl2::ttf::Font<'ttf_module, 'rwops>,
}

impl<'ttf_module, 'rwops> Font<'ttf_module, 'rwops> {
    pub(crate) fn new(ttf_font: sdl2::ttf::Font<'ttf_module, 'rwops>) -> Self {
        Font {
            ttf_font
        }
    }

    pub(crate) fn ttf_font(&self) -> &sdl2::ttf::Font {
        &self.ttf_font
    }
}

