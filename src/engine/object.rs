use std::sync::Arc;

use cgmath::Point3;
use winit::window::Window;

use super::{camera::Camera, input::Input, model, renderer::RendererState};

pub struct Context<'a> {
    pub(in crate::engine) renderer_state: &'a mut RendererState,
    pub(in crate::engine) window: &'a mut Arc<Window>,
    pub input: &'a mut Input,
}

impl<'a> Context<'a> {
    pub fn spawn_model(&mut self, model: &model::Model) {
        self.renderer_state.models.insert(model.to_owned());
    }

    pub fn despawn_model(&mut self, model: &model::Model) {
        self.renderer_state.models.remove(model);
    }

    pub fn update_camera(&mut self, camera: &Camera) {
        self.renderer_state.camera = camera.to_owned();
    }

    pub fn get_camera_position(&mut self) -> Point3<f32> {
        self.renderer_state.camera.position.clone()
    }
}

#[allow(unused)]
pub trait Object {
    fn start(&mut self, ctx: &mut Context) {}
    fn update(&mut self, ctx: &mut Context, delta: f32) {}
    fn destroy(&mut self, ctx: &mut Context) {}
}
