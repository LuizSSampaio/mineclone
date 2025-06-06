use std::sync::Arc;

use winit::window::Window;

use super::{model, renderer::RendererState};

pub struct Context<'a> {
    pub(in crate::engine) renderer_state: &'a mut RendererState,
    pub(in crate::engine) window: &'a mut Arc<Window>,
}

impl<'a> Context<'a> {
    pub fn spawn_model(&mut self, model: &model::Model) -> anyhow::Result<()> {
        self.renderer_state.models.push(model.to_owned());

        Ok(())
    }
}

#[allow(unused)]
pub trait Object {
    fn start(&self, ctx: &mut Context) {}
    fn update(&self, ctx: &mut Context, delta: f32) {}
    fn destroy(&self, ctx: &mut Context) {}
}
