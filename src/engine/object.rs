use super::{model, renderer::RendererState};

pub struct Context<'a> {
    pub renderer_state: &'a mut RendererState,
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
    fn update(&self, ctx: &mut Context) {}
    fn destroy(&self, ctx: &mut Context) {}
}
