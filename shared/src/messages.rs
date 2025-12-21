use serde::{Deserialize, Serialize};

use crate::{Chunk, ChunkPos, characters};

#[derive(Serialize, Deserialize, Clone)]
pub struct AccountCredentials {
    pub username: String,
    pub user_password: String,
    pub server_password: Option<String>,
}

impl AccountCredentials {
    pub fn new(username: String, user_password: String, server_password: Option<String>) -> Self {
        Self {
            username,
            user_password,
            server_password,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountInfo {
    pub username: String,
    pub characters: Vec<characters::Model>,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ClientControlStreamMessage {
    ConnectionRequest,
    CreateAccount(AccountCredentials),
    Login(AccountCredentials),
    CreateCharacter(String),
    SelectCharacter(i64),
    JoinWorldRequest,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ServerControlStreamMessage {
    Connected,
    Disconnected(String),
    Authenticated(AccountInfo),
    LoginDenied(String),
    AccountCreateDenied(String),
    CharacterSelected,
    CharacterDenied(String),
    InitialWorld {
        chunks: Vec<(ChunkPos, Chunk)>,
    }
}
