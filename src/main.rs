mod engine;

use anyhow::Ok;

fn main() -> anyhow::Result<()> {
    engine::Engine::new().run()?;

    Ok(())
}
