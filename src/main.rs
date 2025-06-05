mod blocks;
mod engine;

use anyhow::Ok;
use engine::{
    app::App,
    object::{Context, Object},
};

fn main() -> anyhow::Result<()> {
    App::default().add_object(GrassBlock::default()).run()?;
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
