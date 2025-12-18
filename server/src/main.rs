use std::sync::{Arc, atomic::AtomicBool};

use server_lib::{GameStartOption, start_single_player};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // let running = Arc::new(AtomicBool::new(true));
    // start_single_player(running, GameStartOption::NewGame("seed".into())).await?;
    Ok(())
}