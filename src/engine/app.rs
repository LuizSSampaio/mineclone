use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{self, EventLoop},
    window::{Window, WindowAttributes},
};

use tokio::runtime::Runtime;

use super::{
    object::{Context, Object},
    renderer::RendererState,
};

#[derive(Default)]
pub struct App {
    pub renderer_state: Option<RendererState>,
    window: Option<Arc<Window>>,

    objects: Vec<Box<dyn Object>>,
}

impl App {
    pub fn run(&mut self) -> anyhow::Result<()> {
        let event_loop = EventLoop::new().unwrap();

        event_loop.set_control_flow(event_loop::ControlFlow::Poll);
        event_loop.run_app(self)?;

        Ok(())
    }

    pub fn add_object(mut self, object: impl Object + 'static) -> Self {
        self.objects.push(Box::new(object));

        self
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let runtime = Runtime::new().unwrap();

        self.window = Some(Arc::new(
            event_loop
                .create_window(WindowAttributes::default())
                .unwrap(),
        ));
        self.renderer_state =
            Some(runtime.block_on(RendererState::new(self.window.as_ref().unwrap())));

        let mut ctx = Context {
            renderer_state: self.renderer_state.as_mut().unwrap(),
        };

        for i in 0..self.objects.len() {
            self.objects[i].start(&mut ctx);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let mut ctx = Context {
            renderer_state: self.renderer_state.as_mut().unwrap(),
        };

        for i in 0..self.objects.len() {
            self.objects[i].update(&mut ctx);
        }

        let state = self.renderer_state.as_mut().unwrap();
        let window = self.window.as_ref().unwrap();

        if window_id == window.id() && !state.input(&event) {
            match event {
                WindowEvent::CloseRequested => {
                    event_loop.exit();
                }
                WindowEvent::Resized(physical_size) => {
                    state.resize(physical_size);
                }
                WindowEvent::RedrawRequested => {
                    window.request_redraw();

                    match state.render() {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                            let new_size = state.size;
                            state.resize(new_size);
                        }
                        Err(wgpu::SurfaceError::OutOfMemory | wgpu::SurfaceError::Other) => {
                            log::error!("OutOfMemory");
                            event_loop.exit();
                        }
                        Err(wgpu::SurfaceError::Timeout) => {
                            log::warn!("Surface timeout");
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
