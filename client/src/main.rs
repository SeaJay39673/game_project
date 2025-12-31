use crate::game::Game;
use anyhow::anyhow;

mod client_networking;
mod game;
mod game_state;
mod graphics;
mod mesh;
mod server_state;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if let Err(e) = rustls::crypto::aws_lc_rs::default_provider().install_default() {
        return Err(anyhow!("Error installing default crypto provider: {:?}", e));
    }

    Game::new()?.run()?;
    Ok(())
}
