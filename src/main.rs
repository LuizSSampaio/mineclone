mod blocks;
mod engine;

use anyhow::Ok;
use cgmath::Deg;
use engine::{
    app::App,
    camera::Camera,
    object::{Context, Object},
};

fn main() -> anyhow::Result<()> {
    App::default()
        .add_object(GrassBlock::default())
        .add_object(Camera::new((0.0, 1.0, 2.0), Deg(-90.0), Deg(-20.0)))
        .run()?;
    Ok(())
}

#[derive(Default)]
struct GrassBlock {}

impl Object for GrassBlock {
    fn start(&self, ctx: &mut Context) {
        let grass = ctx.create_block("grass_block_side.png", "grass").unwrap();
        let _ = ctx.spawn_model(&grass);
    }
}

impl Object for Camera {}
