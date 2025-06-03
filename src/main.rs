mod engine;

use anyhow::Ok;

pub use crate::engine::app;

fn main() -> anyhow::Result<()> {
    engine::Engine::new().run()?;

    Ok(())
}
