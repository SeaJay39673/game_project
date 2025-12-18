use std::{net::SocketAddr, sync::Arc};

use quinn::{Connection, Incoming, RecvStream, SendStream};
use shared::{
    AccountInfo, ClientControlStreamMessage, ServerControlStreamMessage, receive_message,
    send_message,
};

use crate::{
    state::{AuthState, GameManager, ServerSession},
    thread_manager::ThreadManager,
};

pub async fn handle_control_stream(
    send: &mut SendStream,
    recv: &mut RecvStream,
    session: Arc<tokio::sync::Mutex<ServerSession>>,
    game_manager: Arc<GameManager>,
    thread_manager: Arc<ThreadManager>,
) -> anyhow::Result<()> {
    use ClientControlStreamMessage::*;
    loop {
        tokio::select! {
            _ = thread_manager.await_cancel() => {
                println!("Server shutdown detected");
                break;
            },
            response = receive_message::<ClientControlStreamMessage>(recv) => {
                match response {
                    Ok(Login(credentials)) => {
                        let message = match game_manager.login(credentials).await {
                            ServerControlStreamMessage::Authenticated(account_info) => {
                                let AccountInfo { username, .. } = &account_info;
                                session.lock().await.auth = AuthState::Authenticated {
                                    username: username.clone(),
                                };
                                println!("Server logged in account");
                                ServerControlStreamMessage::Authenticated(account_info)
                            }
                            message => {
                                session.lock().await.auth = AuthState::Rejected("Could not login".into());
                                message
                            }
                        };
                        send_message(send, message).await?;
                    }
                    Ok(CreateAccount(credentials)) => {
                        let message = match game_manager.create(credentials).await {
                            ServerControlStreamMessage::Authenticated(account_info) => {
                                let AccountInfo { username, .. } = &account_info;
                                session.lock().await.auth = AuthState::Authenticated {
                                    username: username.clone(),
                                };
                                println!("Server created accout");
                                ServerControlStreamMessage::Authenticated(account_info)
                            }
                            msg => {
                                session.lock().await.auth =
                                    AuthState::Rejected("Could not create account".into());
                                msg
                            }
                        };
                        send_message(send, message).await?;
                    }
                    Ok(SelectCharacter(id)) => {
                        let session_guard = session.lock().await;
                        match session_guard.auth.clone() {
                            AuthState::Authenticated{ username } => {
                                let msg = game_manager.select_character(username, id).await;
                                println!("Character selected");
                                if let Err(e) = send_message(send, msg).await {
                                    eprintln!("Error sending selected character to client: {e}");
                                }
                            },
                            _ => {
                                if let Err(e) = send_message(send, ServerControlStreamMessage::CharacterDenied("User not authenticated".into())).await {
                                    eprintln!("Could not select character: {e}");
                                }
                            }
                        }
                    }
                    Ok(CreateCharacter(character_name)) => {
                        let session_guard = session.lock().await;
                        match session_guard.auth.clone() {
                            AuthState::Authenticated{ username } => {
                                let msg = game_manager.create_character(username, character_name).await;
                                println!("Character created");
                                if let Err(e) = send_message(send, msg).await {
                                    eprintln!("Error sending created character to client: {e}");
                                }
                            },
                            _ => {
                                if let Err(e) = send_message(send, ServerControlStreamMessage::CharacterDenied("User not authenticated".into())).await {
                                    eprintln!("Could not create character: {e}");
                                }
                            }
                        }
                    }
                    Err(e) => {
                        session.lock().await.auth =
                            AuthState::Rejected(format!("Error receiving message from client: {e}"));
                        println!("Client rejected: {e}");
                        send_message(send, ServerControlStreamMessage::Disconnected(format!(
                            "Error receiving message from client: {e}"
                        ))).await?;
                        break;
                    }
                    _ => {
                        session.lock().await.auth = AuthState::Rejected(
                            "Improper handshake flow initiated. Must login/create account first".into(),
                        );
                        println!("Client rejected");
                        send_message(send, ServerControlStreamMessage::Disconnected(
                            "Improper handshake flow initiated. Must login/create account first".into(),
                        )).await?;
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}

pub async fn handle_connection(
    conn: Incoming,
    game_manager: Arc<GameManager>,
    thread_manager: Arc<ThreadManager>,
) -> anyhow::Result<()> {
    let connection: Connection = conn.await?;
    let addr: SocketAddr = connection.remote_address();

    let session = Arc::new(tokio::sync::Mutex::new(ServerSession::new(addr)));

    game_manager
        .session_manager
        .sessions
        .insert(addr, session.clone());

    let child = thread_manager.child().await;

    thread_manager
        .spawn_loop({
            move || {
                let connection = connection.clone();
                let session = session.clone();
                let game_manager = game_manager.clone();
                let child = child.clone();
                async move {
                    match connection.accept_bi().await {
                        Ok((mut send, mut recv)) => {
                            let session = session.clone();
                            let mut session_guard = session.lock().await;
                            if session_guard.control_stream_opened {
                                drop(session_guard);
                                let _ = send_message(
                                    &mut send,
                                    ServerControlStreamMessage::Disconnected(
                                        "Control stream already opened".into(),
                                    ),
                                );
                            } else {
                                session_guard.control_stream_opened = true;
                                drop(session_guard);
                                let game_manager = game_manager.clone();
                                let child_2 = child.child().await;
                                child
                                    .spawn(|| async move {
                                        if let Err(e) = handle_control_stream(
                                            &mut send,
                                            &mut recv,
                                            session.clone(),
                                            game_manager,
                                            child_2,
                                        )
                                        .await
                                        {
                                            println!("Could not handle stream correctly: {e}");
                                        }
                                    })
                                    .await;
                            }
                        }
                        Err(e) => {
                            return;
                        }
                    };
                }
            }
        })
        .await;

    Ok(())
}
