use app::App;
use winit::event_loop::{self, EventLoop};

pub mod app;
pub mod model;
pub mod renderer;
pub mod texture;

pub struct Engine {}

impl Engine {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        let event_loop = EventLoop::new().unwrap();
        let mut app = App::default();

        event_loop.set_control_flow(event_loop::ControlFlow::Poll);
        event_loop.run_app(&mut app)?;

        Ok(())
    }
}
