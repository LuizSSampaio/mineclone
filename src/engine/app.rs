use std::{sync::Arc, time::Instant};

use winit::{
    application::ApplicationHandler,
    event::{DeviceEvent, WindowEvent},
    event_loop::{self, EventLoop},
    window::{Window, WindowAttributes},
};

use tokio::runtime::Runtime;

use super::{
    input::Input,
    object::{Context, Object},
    renderer::RendererState,
};

#[derive(Default)]
pub struct App {
    renderer_state: Option<RendererState>,
    window: Option<Arc<Window>>,
    input: Input,

    objects: Vec<Box<dyn Object>>,

    last_render_time: Option<Instant>,
}

impl App {
    pub fn run(&mut self) -> anyhow::Result<()> {
        let event_loop = EventLoop::new().unwrap();

        self.last_render_time = Some(Instant::now());

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
            window: self.window.as_mut().unwrap(),
            input: &mut self.input,
        };

        for i in 0..self.objects.len() {
            self.objects[i].start(&mut ctx);
        }

        if ctx
            .window
            .set_cursor_grab(winit::window::CursorGrabMode::Locked)
            .is_err()
            && ctx
                .window
                .set_cursor_grab(winit::window::CursorGrabMode::Confined)
                .is_err()
        {
            let _ = ctx
                .window
                .set_cursor_grab(winit::window::CursorGrabMode::None);
        }
        self.window.as_ref().unwrap().set_cursor_visible(false);
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let mut ctx = Context {
            renderer_state: self.renderer_state.as_mut().unwrap(),
            window: self.window.as_mut().unwrap(),
            input: &mut self.input,
        };

        let now = Instant::now();
        let delta = (now - self.last_render_time.unwrap()).as_secs_f32();
        self.last_render_time = Some(now);

        for i in 0..self.objects.len() {
            self.objects[i].update(&mut ctx, delta);
        }

        ctx.renderer_state.update();
        if window_id == ctx.window.id() && !ctx.input.handle_input(&event) {
            match event {
                WindowEvent::CloseRequested => {
                    event_loop.exit();
                }
                WindowEvent::Resized(physical_size) => {
                    ctx.renderer_state.resize(physical_size);
                }
                WindowEvent::RedrawRequested => {
                    ctx.window.request_redraw();

                    match ctx.renderer_state.render() {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                            let new_size = ctx.renderer_state.size;
                            ctx.renderer_state.resize(new_size);
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

    fn device_event(
        &mut self,
        _event_loop: &event_loop::ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        if let DeviceEvent::MouseMotion { delta } = event {
            self.input.mouse_delta.0 += delta.0;
            self.input.mouse_delta.1 += delta.1;
        }
    }
}
