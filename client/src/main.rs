use crate::game::Game;
use anyhow::anyhow;

mod asset_ingestion;
mod client_networking;
mod engine;
mod game;
mod game_state;
mod server_state;
mod ui;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if let Err(e) = rustls::crypto::aws_lc_rs::default_provider().install_default() {
        return Err(anyhow!("Error installing default crypto provider: {:?}", e));
    }

    Game::new()?.run()?;
    Ok(())
}
