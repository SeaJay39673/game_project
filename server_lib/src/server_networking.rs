use std::{net::SocketAddr, sync::Arc};

use quinn::{Connection, Incoming, RecvStream, SendStream};
use shared::{
    ClientControlStreamMessage, ServerControlStreamMessage, receive_message, send_message,
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
                println!("Server exiting control stream");
                break;
            },
            response = receive_message::<ClientControlStreamMessage>(recv) => {
                match response {
                    Ok(ConnectionRequest) => {
                        send_message(send, ServerControlStreamMessage::Connected).await?;
                    }
                    Ok(Login(credentials)) => {
                        let message = game_manager.login(credentials, session.clone()).await;
                        send_message(send, message).await?;
                    }
                    Ok(CreateAccount(credentials)) => {
                        let message = game_manager.create(credentials, session.clone()).await;
                        send_message(send, message).await?;
                    }
                    Ok(SelectCharacter(id)) => {
                        let session_guard = session.lock().await;
                        match session_guard.auth.clone() {
                            AuthState::Authenticated{ username } => {
                                let msg = game_manager.select_character(username, id).await;
                                if let Err(e) = send_message(send, msg).await {
                                    eprintln!("Error sending selected character to client: {e}");
                                }
                            },
                            _ => {
                                if let Err(e) = send_message(send, ServerControlStreamMessage::CharacterDenied("User not authenticated".into())).await {
                                    eprintln!("Could not deny client character selection: {e}");
                                }
                            }
                        }
                    }
                    Ok(CreateCharacter(character_name)) => {
                        let session_guard = session.lock().await;
                        match session_guard.auth.clone() {
                            AuthState::Authenticated{ username } => {
                                let msg = game_manager.create_character(username, character_name).await;
                                if let Err(e) = send_message(send, msg).await {
                                    eprintln!("Error sending created character to client: {e}");
                                }
                            },
                            _ => {
                                if let Err(e) = send_message(send, ServerControlStreamMessage::CharacterDenied("User not authenticated".into())).await {
                                    eprintln!("Could not deny client character creation: {e}");
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error receiving message from client: {e}");
                        thread_manager.shutdown().await;
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

    loop {
        tokio::select! {
            _ = thread_manager.await_cancel() => {
                println!("Server exiting accept loop");
                break;
            }
            response = connection.accept_bi() => {
                match response {
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
                    },
                    Err(e) => {
                        eprintln!("Error accepting bi-directional stream from client: {e}");
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}
