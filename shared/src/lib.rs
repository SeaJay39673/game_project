use serde::{Deserialize, Serialize};

mod map;
pub use map::*;

mod pos;
pub use pos::*;

mod shared_networking;
pub use shared_networking::*;

mod entities;
pub use entities::*;

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

#[derive(Serialize, Deserialize, Clone)]
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
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ServerControlStreamMessage {
    Connected,
    Disconnected(String),
    Authenticated(AccountInfo),
    LoginDenied(String),
    AccountCreateDenied(String),
    CharacterSelected(i64),
    CharacterDenied(String),
}
