use shared::AccountCredentials;
use tokio::sync::mpsc::error::TryRecvError;

use crate::{
    game_state::ServerState,
    graphics::{Graphics, Renderable},
    mesh::ChunkMeshes,
};

pub struct GameState {
    server_state: ServerState,
    render_chunks: ChunkMeshes,
}

impl GameState {
    pub async fn new(graphics: &Graphics) -> anyhow::Result<Self> {
        let server_state = ServerState::new().await;

        let render_chunks = ChunkMeshes::new(graphics)?;

        Ok(Self {
            server_state,
            render_chunks,
        })
    }
    pub fn update(&mut self, graphics: &Graphics) {
        if self.server_state.thread_manager.is_cancelled() {
            return;
        }

        loop {
            let msg = match self.server_state.server_rx.try_recv() {
                Ok(msg) => msg,
                Err(TryRecvError::Disconnected) => {
                    eprintln!("Disconnected from forwarder");
                    pollster::block_on(self.server_state.thread_manager.shutdown());
                    break;
                }
                Err(TryRecvError::Empty) => {
                    break;
                }
            };

            use shared::ServerControlStreamMessage::*;

            match msg {
                Connected => {
                    if let Err(e) =
                        self.server_state
                            .client_tx
                            .send(shared::ClientControlStreamMessage::Login(
                                AccountCredentials {
                                    username: "Test".into(),
                                    user_password: "Test".into(),
                                    server_password: None,
                                },
                            ))
                    {
                        eprintln!("Error sending account message to server: {e}");
                    }
                }
                Authenticated(account_info) => {
                    if account_info.characters.len() > 1 {
                        if let Err(e) = self.server_state.client_tx.send(
                            shared::ClientControlStreamMessage::SelectCharacter(
                                account_info.characters[0].character_id,
                            ),
                        ) {
                            eprintln!("Error requesting character from server: {e}");
                        }
                    } else {
                        if let Err(e) = self.server_state.client_tx.send(
                            shared::ClientControlStreamMessage::CreateCharacter(
                                "TestCharacter".into(),
                            ),
                        ) {
                            eprintln!("Error requesting create character from server: {e}");
                        }
                    }
                }
                CharacterSelected => {
                    if let Err(e) = self
                        .server_state
                        .client_tx
                        .send(shared::ClientControlStreamMessage::JoinWorldRequest)
                    {
                        eprintln!("Error requesting to join the world: {e}");
                    }
                }
                InitialWorld { chunks } => {
                    for (chunk_pos, chunk) in chunks {
                        self.render_chunks
                            .insert(&graphics.device, chunk_pos, chunk);
                    }
                }
                AccountCreateDenied(reason) => {
                    eprintln!("Could not create account: {reason}")
                }
                LoginDenied(reason) => {
                    eprintln!("Could not login: {reason}")
                }
                CharacterDenied(reason) => {
                    eprintln!("Could not create/select character: {reason}");
                }
                Disconnected(reason) => {
                    eprintln!("Disconnected from server: {reason}")
                }
            }
        }
    }
}

impl Renderable for GameState {
    fn render(&self, render_pass: &mut wgpu::RenderPass) {
        self.render_chunks.render(render_pass);
    }
}
