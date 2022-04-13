use winit::{event_loop::EventLoop, window::WindowBuilder};

pub struct Window(winit::window::Window);

impl Window {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let window = WindowBuilder::new()
            .with_title("Fornjot")
            .with_maximized(true)
            .with_decorations(true)
            .with_transparent(false)
            .build(event_loop)
            .unwrap();

        Self(window)
    }

    pub fn inner(&self) -> &winit::window::Window {
        &self.0
    }

    pub fn width(&self) -> u32 {
        self.0.inner_size().width
    }

    pub fn height(&self) -> u32 {
        self.0.inner_size().height
    }
}
