use crate::game::Game;

mod game;
mod engine;
mod asset_ingestion;
mod ui;
mod game_state;
mod client_networking;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Game::new()?.run()?;

    Ok(())
}