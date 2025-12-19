use std::{
    any,
    fs::read_to_string,
    net::SocketAddr,
    str::FromStr,
    sync::{Arc, Mutex},
};

use anyhow::anyhow;
use server_lib::{GameStartOption, start_single_player, thread_manager::ThreadManager};
use shared::{
    ClientControlStreamMessage, ServerControlStreamMessage, receive_message, send_message,
};
use tokio::sync::{
    Notify,
    mpsc::{UnboundedReceiver, UnboundedSender, error::TryRecvError, unbounded_channel},
    watch,
};

use crate::{client_networking::get_single_player_endpoint, server_state};

pub struct ServerState {
    thread_manager: Arc<ThreadManager>,
    shutdown: bool,
    client_tx: UnboundedSender<ClientControlStreamMessage>,
    server_rx: Mutex<UnboundedReceiver<ServerControlStreamMessage>>,
}

impl ServerState {
    async fn start_server(
        thread_manager: Arc<ThreadManager>,
        ready: watch::Sender<bool>,
        option: GameStartOption,
    ) {
        let child = thread_manager.child().await;
        thread_manager
            .spawn({
                let thread_manager = thread_manager.clone();
                move || async move {
                    if let Err(e) = start_single_player(option, ready, child).await {
                        eprintln!("Error with the server: {e}");
                        thread_manager.abort_async().await;
                    }
                }
            })
            .await;
    }

    async fn start_forwarder(
        thread_manager: Arc<ThreadManager>,
        mut server_ready: watch::Receiver<bool>,
        forwarder_ready: Arc<Notify>,
        server_tx: UnboundedSender<ServerControlStreamMessage>,
        mut client_rx: UnboundedReceiver<ClientControlStreamMessage>,
    ) {
        thread_manager
            .spawn({
                let thread_manager = thread_manager.clone();
                move || async move {
                    while !*server_ready.borrow() {
                        if let Err(e) = server_ready.changed().await {
                            println!("Error awaiting server to be ready: {e}");
                            thread_manager.shutdown().await;
                        }
                    }
                    let endpoint = match get_single_player_endpoint() {
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
                            eprintln!("Could not parse address: {e}");
                            thread_manager.shutdown().await;
                            return;
                        }
                    };
                    let connection = match endpoint.connect(addr, "localhost".into()) {
                        Ok(connecting) => match connecting.await {
                            Ok(connection) => connection,
                            Err(e) => {
                                eprintln!("Could not await connection to server: {e}");
                                thread_manager.shutdown().await;
                                return;
                            }
                        },
                        Err(e) => {
                            eprintln!("Failed connecting to server: {e}");
                            thread_manager.shutdown().await;
                            return;
                        }
                    };
                    let (send, recv) = &mut match connection.open_bi().await {
                        Ok((send, recv)) => (send, recv),
                        Err(e) => {
                            eprintln!("Failed openeing bi-directional stream to server: {e}");
                            thread_manager.shutdown().await;
                            return;
                        }
                    };

                    if let Err(e) =
                        send_message(send, ClientControlStreamMessage::ConnectionRequest).await
                    {
                        eprintln!("Failed to send initial connection request to server: {e}");
                        thread_manager.shutdown().await;
                        return;
                    }

                    loop {
                        tokio::select! {
                            _ = thread_manager.await_cancel() => {
                                println!("Closing Forwarding Loop");
                                break;
                            }

                            result = receive_message(recv) => {
                                match result {
                                    Ok(msg) => {
                                        if let Err(e) = server_tx.send(msg) {
                                            eprintln!("Could not forward message from server: {e}");
                                        }
                                    }
                                    Err(e) => {
                                        eprintln!("Could not receive message from server: {e}");
                                        thread_manager.shutdown().await;
                                    }
                                }
                            }

                            Some(msg) = client_rx.recv() => {
                                if let Err(e) = send_message(send, msg).await {
                                    eprintln!("Could not forward message to server: {e}");
                                    thread_manager.shutdown().await;
                                }
                            }
                        }
                    }
                }
            })
            .await;
    }

    pub fn is_shutdown(self: &Arc<Self>) -> bool {
        self.thread_manager.is_cancelled()
    }

    pub async fn new(option: GameStartOption) -> Arc<Self> {
        let thread_manager = ThreadManager::new();
        let (server_ready_tx, mut server_ready_rx) = watch::channel(false);
        Self::start_server(thread_manager.clone(), server_ready_tx, option).await;

        let (server_tx, server_rx) = unbounded_channel::<ServerControlStreamMessage>();
        let (client_tx, client_rx) = unbounded_channel::<ClientControlStreamMessage>();
        let forwarder_ready = Arc::new(Notify::new());
        Self::start_forwarder(
            thread_manager.clone(),
            server_ready_rx,
            forwarder_ready,
            server_tx,
            client_rx,
        )
        .await;

        Arc::new(Self {
            thread_manager,
            shutdown: false,
            client_tx,
            server_rx: Mutex::new(server_rx),
        })
    }

    pub async fn shutdown(self: Arc<Self>) {
        self.thread_manager.shutdown().await;
    }

    pub async fn abort(self: Arc<Self>) {
        self.thread_manager.abort_async().await;
    }

    pub fn send_message(
        self: Arc<Self>,
        message: ClientControlStreamMessage,
    ) -> anyhow::Result<()> {
        self.client_tx.send(message)?;
        Ok(())
    }

    pub fn receive_messages(
        self: Arc<Self>,
    ) -> anyhow::Result<
        Vec<Result<ServerControlStreamMessage, tokio::sync::mpsc::error::TryRecvError>>,
    > {
        let mut lock = self.server_rx.lock().map_err(|e| anyhow!("{e}"))?;
        let mut messages: Vec<Result<ServerControlStreamMessage, TryRecvError>> = vec![];
        loop {
            let result = lock.try_recv();
            if let Err(TryRecvError::Empty) = result {
                break;
            }
            messages.push(result);
        }

        Ok(messages)
    }
}
