mod engine;

use anyhow::Ok;

fn main() -> anyhow::Result<()> {
    let mut game = engine::Game::new();

    game.run()?;
    Ok(())
}
