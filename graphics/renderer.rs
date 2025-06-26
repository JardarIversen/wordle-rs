pub mod Renderer {
    window: winit::window::Window,
    surface: wgpu:Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<i32>,
}

impl Renderer {
    pub async fn new() -> Self {
        Self {
            window: 
        }
    }

    pub async fn window(&self) -> &winit::window::Window {
        &self.window;
    }
}