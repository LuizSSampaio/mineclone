use app::App;
use winit::event_loop::{self, EventLoop};

mod app;
pub mod model;
mod renderer;
pub mod resources;
pub mod texture;

pub struct Engine {
    app: app::App,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            app: App::default(),
        }
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        let event_loop = EventLoop::new().unwrap();

        event_loop.set_control_flow(event_loop::ControlFlow::Poll);
        event_loop.run_app(&mut self.app)?;

        Ok(())
    }
}
