use anyhow::anyhow;
use bcrypt::{hash, verify};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, ModelTrait,
    QueryFilter, sqlx::types::chrono,
};
use shared::{AccountCredentials, AccountInfo, ChunkManager, ServerControlStreamMessage, accounts, characters};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::{GameStartOption, state::{AuthState, ServerSession, SessionManager}};

pub struct GameManager {
    pub db: DatabaseConnection,
    pub session_manager: Arc<SessionManager>,
    pub game_dir: PathBuf,
    pub chunk_manager: ChunkManager,
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
            chunk_manager: ChunkManager::new()?,
        }))
    }

    pub async fn create_character(
        &self,
        username: String,
        character_name: String,
    ) -> ServerControlStreamMessage {
        match accounts::Entity::find_by_id(username.clone())
            .one(&self.db)
            .await
        {
            Ok(Some(_)) => {}
            Ok(None) => {
                return ServerControlStreamMessage::CharacterDenied(
                    "Account does not exist".into(),
                );
            }
            Err(e) => {
                return ServerControlStreamMessage::AccountCreateDenied(format!(
                    "DB error checking account: {e}"
                ));
            }
        }

        let character = characters::ActiveModel {
            account_username: Set(username),
            name: Set(character_name),
            created_at: Set(chrono::Utc::now().into()),
            ..Default::default()
        };

        match character.insert(&self.db).await {
            Ok(model) => ServerControlStreamMessage::CharacterSelected,
            Err(e) => ServerControlStreamMessage::CharacterDenied(format!(
                "Failed to create character: {e}"
            )),
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
            .await
        {
            Ok(Some(model)) => {
                return ServerControlStreamMessage::CharacterSelected;
            }
            Ok(None) => {
                return ServerControlStreamMessage::CharacterDenied(format!(
                    "Could not find character"
                ));
            }
            Err(e) => ServerControlStreamMessage::CharacterDenied(format!(
                "DB error when trying to find character: {e}"
            )),
        };

        ServerControlStreamMessage::CharacterDenied("An unexpected error occured".into())
    }

    pub async fn create(
        &self,
        credentials: AccountCredentials,
        session: Arc<tokio::sync::Mutex<ServerSession>>,
    ) -> ServerControlStreamMessage {
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
                let msg = "Account already exists".to_string();
                session.lock().await.auth = AuthState::Rejected(msg.clone());
                return ServerControlStreamMessage::AccountCreateDenied(msg);
            }
            Ok(None) => {}
            Err(e) => {
                let msg = format!("Database error: {e}");
                session.lock().await.auth = AuthState::Rejected(msg.clone());
                return ServerControlStreamMessage::AccountCreateDenied(msg);
            }
        }

        let password_hash = match hash(&user_password, bcrypt::DEFAULT_COST) {
            Ok(h) => h,
            Err(e) => {
                let msg = format!("Password hash failed: {e}");
                session.lock().await.auth = AuthState::Rejected(msg.clone());
                return ServerControlStreamMessage::AccountCreateDenied(msg);
            }
        };

        let account = accounts::ActiveModel {
            username: Set(username.clone()),
            password_hash: Set(password_hash),
            ..Default::default()
        };

        match account.insert(&self.db).await {
            Ok(_) => {
                session.lock().await.auth = AuthState::Authenticated {
                    username: username.clone(),
                };
                ServerControlStreamMessage::Authenticated(AccountInfo {
                    username,
                    characters: vec![],
                })
            }
            Err(e) => {
                let msg = format!("Failed to create account: {e}");
                session.lock().await.auth = AuthState::Rejected(msg.clone());
                ServerControlStreamMessage::AccountCreateDenied(msg)
            }
        }
    }

    pub async fn login(
        &self,
        credentials: AccountCredentials,
        session: Arc<tokio::sync::Mutex<ServerSession>>,
    ) -> ServerControlStreamMessage {
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
                let msg = "Account does not exist".to_string();
                session.lock().await.auth = AuthState::Rejected(msg.clone());
                return ServerControlStreamMessage::LoginDenied(msg);
            }
            Err(e) => {
                let msg = format!("Database error: {e}");
                session.lock().await.auth = AuthState::Rejected(msg.clone());
                return ServerControlStreamMessage::LoginDenied(msg);
            }
        };

        match verify(&user_password, &account.password_hash) {
            Ok(true) => match account.find_related(characters::Entity).all(&self.db).await {
                Ok(characters) => {
                    session.lock().await.auth = AuthState::Authenticated {
                        username: username.clone(),
                    };
                    ServerControlStreamMessage::Authenticated(AccountInfo {
                        username,
                        characters: characters,
                    })
                }
                Err(e) => {
                    let msg = format!("Could not load account characters: {e}");
                    session.lock().await.auth = AuthState::Rejected(msg.clone());
                    ServerControlStreamMessage::LoginDenied(msg)
                }
            },
            Ok(false) => {
                let msg = "Invalid Password".to_string();
                session.lock().await.auth = AuthState::Rejected(msg.clone());
                ServerControlStreamMessage::LoginDenied(msg)
            }
            Err(e) => {
                let msg = format!("Password verification failed: {e}");
                session.lock().await.auth = AuthState::Rejected(msg.clone());
                ServerControlStreamMessage::LoginDenied(msg)
            }
        }
    }
}
