use serde::{Deserialize, Serialize};

mod map;
pub use map::*;

mod pos;
pub use pos::*;

mod shared_networking;
pub use shared_networking::*;

mod state;
pub use state::*;

#[derive(Serialize, Deserialize)]
pub enum StreamKind {
    Gameplay,
    Chat,
    Inventory,
    Crafting,
    ChunkStreaming,
}

#[derive(Serialize, Deserialize)]
pub enum ClientMessage {
    ConnectionRequest{ server_pass: String },
    PlayerCreateRequest { username: String, password: String },
    PlayerLoginRequest { username: String, password: String },
    ChatMessage{ session: String, message: String },
    ChunkRequest{ session: String, pos: ChunkPos }
}

#[derive(Serialize, Deserialize)]
pub enum ServerMessage {
    ConnectionAuthenticated,
    ConnectionDenied{ reason: Option<String> },
    PlayerCreateLoginDenied{ reason: Option<String>},
    PlayerAuthResponse{ username: String, session: String },
    ChunkResponse { pos: ChunkPos, chunk: Chunk }
}