use anyhow::anyhow;
use bcrypt::{hash, verify};
use dashmap::DashMap;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait, QueryFilter, sqlx::types::chrono
};
use shared::{AccountCredentials, AccountInfo, ServerControlStreamMessage, accounts, characters};
use std::{
    fs,
    net::SocketAddr,
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::GameStartOption;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionRole {
    LocalMaster,
    RemoteClient,
}

#[derive(Debug, Clone)]
pub enum AuthState {
    WaitingForCredentials,
    Authenticated { username: String },
    Rejected(String),
}

#[derive(Debug, Clone)]
pub struct ServerSession {
    pub role: ConnectionRole,
    pub auth: AuthState,
    pub addr: SocketAddr,
    pub control_stream_opened: bool,
}

impl ServerSession {
    pub fn new(addr: SocketAddr) -> Self {
        let role = if addr.ip().is_loopback() {
            ConnectionRole::LocalMaster
        } else {
            ConnectionRole::RemoteClient
        };

        Self {
            role,
            auth: AuthState::WaitingForCredentials,
            addr,
            control_stream_opened: false,
        }
    }
}

pub struct SessionManager {
    pub sessions: DashMap<SocketAddr, Arc<tokio::sync::Mutex<ServerSession>>>,
}

impl SessionManager {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            sessions: DashMap::new(),
        })
    }
}

pub struct GameManager {
    pub db: DatabaseConnection,
    pub session_manager: Arc<SessionManager>,
    pub game_dir: PathBuf,
}

impl GameManager {
    pub fn new(db: DatabaseConnection, option: GameStartOption) -> anyhow::Result<Arc<Self>> {
        let data_path = Path::new("src/data");
        if !data_path.exists() {
            fs::create_dir(data_path)?;
        }

        let game_dir = match option {
            GameStartOption::LoadGame(name) => {
                let path = data_path.join(name);
                if !path.exists() {
                    return Err(anyhow!(
                        "Error loading game, path '{:?}' does not exist",
                        path
                    ));
                }
                path
            }
            GameStartOption::NewGame(name) => {
                let path = data_path.join(name);
                if path.exists() {
                    return Err(anyhow!(
                        "Error creating game, path '{:?}' already exists",
                        path
                    ));
                }
                fs::create_dir(&path)?;
                path
            }
        };

        Ok(Arc::new(Self {
            db: db,
            session_manager: SessionManager::new(),
            game_dir,
        }))
    }

    pub async fn create_character(
        &self,
        username: String,
        character_name: String,
    ) -> ServerControlStreamMessage {
        match accounts::Entity::find_by_id(username.clone()).one(&self.db).await {
            Ok(Some(_)) => {},
            Ok(None) => {
                return ServerControlStreamMessage::CharacterDenied("Account does not exist".into());
            },
            Err(e) => {
                return ServerControlStreamMessage::AccountCreateDenied(format!("DB error checking account: {e}"));
            }
        }

        let character = characters::ActiveModel {
            account_username: Set(username),
            name: Set(character_name),
            created_at: Set(chrono::Utc::now().into()),
            ..Default::default()
        };

        match character.insert(&self.db).await {
            Ok(model) => ServerControlStreamMessage::CharacterSelected(model.character_id),
            Err(e) => ServerControlStreamMessage::CharacterDenied(format!("Failed to create character: {e}"))
        }
    }

    pub async fn select_character(
        &self,
        username: String,
        character_id: i64,
    ) -> ServerControlStreamMessage {
        match characters::Entity::find()
            .filter(characters::Column::AccountUsername.eq(username.clone()))
            .filter(characters::Column::CharacterId.eq(character_id.clone()))
            .one(&self.db)
            .await {
                Ok(Some(model)) => {
                    ServerControlStreamMessage::CharacterSelected(model.character_id)
                },
                Ok(None) => {
                    ServerControlStreamMessage::CharacterDenied(format!("Could not find character"))
                }
                Err(e) => {
                    ServerControlStreamMessage::CharacterDenied(format!("DB error when trying to find character: {e}"))
                }
            };

        ServerControlStreamMessage::CharacterDenied("An unexpected error occured".into())
    }

    pub async fn create(&self, credentials: AccountCredentials) -> ServerControlStreamMessage {
        let AccountCredentials {
            username,
            user_password,
            ..
        } = credentials;

        match accounts::Entity::find_by_id(username.clone())
            .one(&self.db)
            .await
        {
            Ok(Some(_)) => {
                return ServerControlStreamMessage::AccountCreateDenied(
                    "Account already exists".into(),
                );
            }
            Ok(None) => {} // good, continue
            Err(e) => {
                return ServerControlStreamMessage::AccountCreateDenied(format!(
                    "Database error: {e}"
                ));
            }
        }

        let password_hash = match hash(&user_password, bcrypt::DEFAULT_COST) {
            Ok(h) => h,
            Err(e) => {
                return ServerControlStreamMessage::AccountCreateDenied(format!(
                    "Password hash failed: {e}"
                ));
            }
        };

        let account = accounts::ActiveModel {
            username: Set(username.clone()),
            password_hash: Set(password_hash),
            ..Default::default()
        };

        match account.insert(&self.db).await {
            Ok(_) => ServerControlStreamMessage::Authenticated(AccountInfo {
                username,
                characters: vec![],
            }),
            Err(e) => ServerControlStreamMessage::AccountCreateDenied(format!(
                "Failed to create account: {e}"
            )),
        }
    }

    pub async fn login(&self, credentials: AccountCredentials) -> ServerControlStreamMessage {
        let AccountCredentials {
            username,
            user_password,
            ..
        } = credentials;
        let account = match accounts::Entity::find_by_id(username.clone())
            .one(&self.db)
            .await
        {
            Ok(Some(acc)) => acc,
            Ok(None) => {
                return ServerControlStreamMessage::LoginDenied("Account does not exists".into());
            }
            Err(e) => {
                return ServerControlStreamMessage::LoginDenied(format!("Database error: {e}"));
            }
        };

        match verify(&user_password, &account.password_hash) {
            Ok(true) => match account.find_related(characters::Entity).all(&self.db).await {
                Ok(characters) => ServerControlStreamMessage::Authenticated(AccountInfo {
                    username,
                    characters: characters,
                }),
                Err(e) => ServerControlStreamMessage::LoginDenied(format!(
                    "Could not load account characters: {e}"
                )),
            },
            Ok(false) => ServerControlStreamMessage::LoginDenied("Invalid Password".into()),
            Err(e) => ServerControlStreamMessage::LoginDenied(format!(
                "Password verification failed: {e}"
            )),
        }
    }
}
