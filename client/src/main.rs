use crate::game::Game;

mod game;
mod engine;
mod asset_ingestion;
mod ui;

fn main() -> anyhow::Result<()> {
    Game::new()?.run()?;

    Ok(())
}