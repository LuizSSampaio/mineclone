use std::sync::Arc;

use winit::window::Window;

pub struct RendererState {}

impl RendererState {
    pub async fn new(window: &Arc<Window>) -> Self {
        Self {}
    }
}
