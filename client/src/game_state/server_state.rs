use std::{net::SocketAddr, str::FromStr, sync::Arc};

use server_lib::thread_manager::ThreadManager;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel};

use crate::client_networking;

pub struct ServerState {
    pub thread_manager: Arc<ThreadManager>,
    pub client_tx: UnboundedSender<shared::ClientControlStreamMessage>,
    pub server_rx: UnboundedReceiver<shared::ServerControlStreamMessage>,
}

impl ServerState {
    pub async fn new() -> Self {
        let thread_manager = ThreadManager::new();
        let (ready_tx, mut ready_rx) = tokio::sync::watch::channel(false);

        let child = thread_manager.child().await;
        thread_manager
            .spawn({
                let thread_manager = thread_manager.clone();
                move || async move {
                    if let Err(e) = server_lib::start_single_player(
                        server_lib::GameStartOption::LoadGame("blah".into()),
                        ready_tx,
                        child,
                    )
                    .await
                    {
                        eprintln!("Error with single player server: {e}");
                        thread_manager.abort_async().await;
                    }
                }
            })
            .await;
        let (mut client_tx, mut client_rx) =
            unbounded_channel::<shared::ClientControlStreamMessage>();
        let (mut server_tx, mut server_rx) =
            unbounded_channel::<shared::ServerControlStreamMessage>();

        thread_manager
            .spawn({
                let thread_manager = thread_manager.clone();
                move || async move {
                    while ! *ready_rx.borrow() {
                        if let Err(e) = ready_rx.changed().await {
                            eprintln!("Error waiting for server: {e}");
                            thread_manager.shutdown().await;
                            return;
                        }
                    }
                    let endpoint = match client_networking::get_single_player_endpoint() {
                        Ok(endpoint) => endpoint,
                        Err(e) => {
                            eprintln!("Could not get single player endpoint: {e}");
                            thread_manager.shutdown().await;
                            return;
                        }
                    };
                    let addr = match SocketAddr::from_str("127.0.0.1:5250") {
                        Ok(addr) => addr,
                        Err(e) => {
                            eprintln!("Failed to parse socket address for server: {e}");
                            thread_manager.shutdown().await;
                            return;
                        }
                    };
                    let connecting = match endpoint.connect(addr, "localhost".into()) {
                        Ok(connecting) => connecting,
                        Err(e) => {
                            eprintln!("Failed to establish connection to server: {e}");
                            thread_manager.shutdown().await;
                            return;
                        }
                    };
                    let connection = match connecting.await {
                        Ok(connection) => connection,
                        Err(e) => {
                            eprintln!("Connection to server error: {e}");
                            thread_manager.shutdown().await;
                            return;
                        }
                    };
                    let (mut send, mut recv) = match connection.open_bi().await {
                        Ok((send, recv)) => (send, recv),
                        Err(e) => {
                            eprintln!("Error opening bi-directional stream to server: {e}");
                            thread_manager.shutdown().await;
                            return;
                        }
                    };

                    if let Err(e) = shared::send_message(&mut send, shared::ClientControlStreamMessage::ConnectionRequest).await {
                        eprintln!("Failed to send connection request to server: {e}");
                        thread_manager.shutdown().await;
                        return;
                    }

                    loop {
                        tokio::select! {
                            _ = thread_manager.await_cancel() => {
                                println!("Shutting down connection loop to game server");
                                break;
                            }
                            result = shared::receive_message::<shared::ServerControlStreamMessage>(&mut recv) => {
                                match result {
                                    Ok(msg) => {
                                        if let Err(e) = server_tx.send(msg) {
                                            eprintln!("Error forwarding message from server to client: {e}");
                                        }
                                    },
                                    Err(e) => {
                                        eprintln!("Error receiving message from server: {e}");
                                        thread_manager.shutdown().await;
                                        break;
                                    }
                                }
                            }
                            Some(msg) = client_rx.recv() => {
                                if let Err(e) = shared::send_message::<shared::ClientControlStreamMessage>(&mut send, msg).await {
                                    eprintln!("Error sending message to the server: {e}");
                                    return;
                                }
                            }
                        }
                    }
                }
            })
            .await;

        Self {
            thread_manager: thread_manager.clone(),
            client_tx,
            server_rx,
        }
    }
}
