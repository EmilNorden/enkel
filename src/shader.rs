pub struct Shader {
    shader: wgpu::ShaderModule,
}

impl Shader {
    pub fn new(device: &wgpu::Device, source: &str) -> Self {
        Self {
            shader: device.create_shader_module(&wgpu::ShaderModuleDescriptor {
                source: wgpu::ShaderSource::Wgsl(source.into()),
                flags: wgpu::ShaderFlags::all(),
                label: None,
            })
        }
    }

    pub(crate) fn shader_module(&self) -> &wgpu::ShaderModule { &self.shader }
}