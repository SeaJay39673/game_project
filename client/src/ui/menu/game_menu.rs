use std::{net::SocketAddr, str::FromStr, sync::Arc};

use server_lib::{GameStartOption, start_single_player, thread_manager::ThreadManager};
use shared::{
    AccountCredentials, AccountInfo, ClientControlStreamMessage, ServerControlStreamMessage,
    receive_message, send_message,
};
use tokio::sync::{
    Notify, mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel}
};

use crate::{
    client_networking,
    engine::{Drawable, Graphics},
    ui::{Menu, MenuEvent, UIRoot, UIRootItem},
};

pub struct ServerState {
    thread_manager: Arc<ThreadManager>,
    rx: UnboundedReceiver<ServerControlStreamMessage>,
    tx: UnboundedSender<ClientControlStreamMessage>,
}

impl ServerState {
    pub async fn new() -> anyhow::Result<Self> {
        let thread_manager = ThreadManager::new();
        let ready = Arc::new(Notify::new());
        
        let endpoint = client_networking::get_single_player_endpoint()?;
        let (server_tx, server_rx) = unbounded_channel::<ServerControlStreamMessage>();
        let (client_tx, mut client_rx) = unbounded_channel::<ClientControlStreamMessage>();

        let child = thread_manager.child().await;
        thread_manager
            .spawn({
                let ready = ready.clone();
                move || 
                {
                async move {
                    let _ = ready.notified().await;
                    let addr = match SocketAddr::from_str("127.0.0.1:5250") {
                        Ok(addr) => addr,
                        Err(e) => {
                            eprintln!("Could not parse address: {e}");
                            return;
                        }
                    };
                    let connection = match endpoint.connect(addr, "localhost".into()) {
                        Ok(connecting) => match connecting.await {
                            Ok(connection) => connection,
                            Err(e) => {
                                eprintln!("Could not await connection to server: {e}");
                                return;
                            }
                        },
                        Err(e) => {
                            eprintln!("Failed connecting to server: {e}");
                            return;
                        }
                    };
                    let (mut send, mut recv) = match connection.open_bi().await {
                        Ok((mut send, recv)) => {
                            if let Err(e) = send_message(
                                &mut send,
                                ClientControlStreamMessage::Login(AccountCredentials {
                                    username: "Test".into(),
                                    user_password: "Test".into(),
                                    server_password: None,
                                }),
                            )
                            .await
                            {
                                eprintln!("Error sending client message to server: {e}");
                                return;
                            };
                            (send, recv)
                        }
                        Err(e) => {
                            eprintln!("Could not open bidirectional control stream to server: {e}");
                            return;
                        }
                    };

                    loop {
                        tokio::select! {
                            _ = child.await_cancel() => {
                                println!("Shutdown detected");
                                break;
                            }
                            response = receive_message(&mut recv) => {
                                match response {
                                    Ok(msg) => {
                                        if let Err(e) = server_tx.send(msg) {
                                            eprintln!("Could not forward server message to channel: {e}");
                                            return;
                                        }
                                    },
                                    Err(e) => {
                                        eprintln!("Error reading message from server: {e}");
                                        return;
                                    }
                                }
                            }
                            Some(msg) = client_rx.recv() => {
                                if let Err(e) = send_message(&mut send, msg).await {
                                    eprintln!("Error sending client message to server: {e}");
                                    return;
                                }
                            }
                        }
                    }
                    }
                }
            })
            .await;

        let child = thread_manager.child().await;
        thread_manager
            .spawn({
                let thread_manager = thread_manager.clone();
                let ready = ready.clone();
                move || async move {
                    if let Err(e) = start_single_player(
                        GameStartOption::LoadGame("Blah".into()),
                        ready,
                        child,
                    )
                    .await
                    {
                        eprintln!("Error with the server: {e}");
                        thread_manager.abort_async().await;
                    }
                }
            })
            .await;

        Ok(Self {
            thread_manager,
            rx: server_rx,
            tx: client_tx,
        })
    }
}

pub struct LoadGameMenu {
    root: Box<dyn UIRoot + 'static>,
    window_size: (f32, f32),
    cursor_location: (f32, f32),
    server_state: Arc<tokio::sync::Mutex<ServerState>>,
}

impl LoadGameMenu {
    pub async fn shutdown(&mut self) {
        self.server_state
            .lock()
            .await
            .thread_manager
            .shutdown()
            .await;
    }

    pub async fn new(
        graphics: &Graphics,
        size: (f32, f32),
        mouse_position: (f32, f32),
    ) -> anyhow::Result<Self> {
        let Graphics {
            device,
            queue,
            tile_bind_group_layout,
            ..
        } = graphics;
        let layout = tile_bind_group_layout;
        let mut root: UIRootItem = UIRootItem::builder()
            .background(crate::engine::TextureType::Color([255, 0, 255, 255]))
            .build(device, queue, layout)?;
        let _ = root.handle_event(
            &super::UIEvent::MouseMoved(mouse_position.0, mouse_position.1),
            queue,
        );

        let server_state = Arc::new(tokio::sync::Mutex::new(ServerState::new().await?));

        Ok(Self {
            root: Box::new(root),
            window_size: size,
            cursor_location: mouse_position,
            server_state,
        })
    }
}

impl Menu for LoadGameMenu {
    fn handle_resize(&mut self, queue: &wgpu::Queue, width: f32, height: f32) {
        self.root.compute_layout((width, height), queue);
    }

    async fn update(
        &mut self,
        ui_event: &super::UIEvent,
        graphics: &Graphics,
    ) -> anyhow::Result<MenuEvent> {
        use ServerControlStreamMessage::*;
        match self.server_state.lock().await.rx.try_recv() {
            Ok(AccountCreateDenied(reason)) => {
                eprintln!("{reason}");
                self.server_state
                    .lock()
                    .await
                    .thread_manager
                    .shutdown()
                    .await;
                return Ok(MenuEvent::SwitchToStart);
            }
            Ok(Disconnected(reason)) => {
                eprintln!("{reason}");
                self.server_state
                    .lock()
                    .await
                    .thread_manager
                    .shutdown()
                    .await;
                return Ok(MenuEvent::SwitchToStart);
            }
            Ok(Authenticated(account_info)) => {
                println!("Client authenticated");
                let AccountInfo { characters, .. } = account_info;
                if characters.len() > 0 {
                    println!("Selecting character");
                    self.server_state.lock().await.tx.send(
                        ClientControlStreamMessage::SelectCharacter(characters[0].character_id),
                    )?;
                } else {
                    println!("Creating character");
                    self.server_state.lock().await.tx.send(
                        ClientControlStreamMessage::CreateCharacter("TestCharacter".into()),
                    )?;
                }
            }
            Ok(LoginDenied(reason)) => {
                eprintln!("{reason}");
                return Ok(MenuEvent::SwitchToStart);
            }
            Ok(CharacterSelected(character_id)) => {
                println!("Character selected/created! {character_id}");
                return Ok(MenuEvent::SwitchToGame(self.server_state.clone()));
            }
            Ok(CharacterDenied(reason)) => {
                eprintln!("{reason}");
                return Ok(MenuEvent::SwitchToStart);
            }
            Err(tokio::sync::mpsc::error::TryRecvError::Empty) => {}
            Err(tokio::sync::mpsc::error::TryRecvError::Disconnected) => {
                self.server_state
                    .lock()
                    .await
                    .thread_manager
                    .shutdown()
                    .await;
                println!("Connection to server closed!");
                return Ok(MenuEvent::SwitchToStart);
            }
        };

        Ok(MenuEvent::None)
    }
}

impl Drawable for LoadGameMenu {
    fn render(&self, render_pass: &mut wgpu::RenderPass) {
        self.root.render(render_pass);
    }
}

pub struct GameMenu {
    root: Box<dyn UIRoot + 'static>,
    window_size: (f32, f32),
    cursor_location: (f32, f32),
    server_state: Arc<tokio::sync::Mutex<ServerState>>,
}

impl GameMenu {
    pub async fn new(
        server_state: Arc<tokio::sync::Mutex<ServerState>>,
        graphics: &Graphics,
        size: (f32, f32),
        mouse_position: (f32, f32),
    ) -> anyhow::Result<Self> {
        let Graphics {
            device,
            queue,
            tile_bind_group_layout,
            ..
        } = graphics;
        let mut root: UIRootItem = UIRootItem::builder()
            .background(crate::engine::TextureType::Color([0, 0, 0, 0]))
            .build(device, queue, tile_bind_group_layout)?;
        root.compute_layout(size, queue);

        Ok(Self {
            root: Box::new(root),
            window_size: size,
            cursor_location: mouse_position,
            server_state,
        })
    }
}

impl Menu for GameMenu {
    fn handle_resize(&mut self, queue: &wgpu::Queue, width: f32, height: f32) {
        self.root.compute_layout((width, height), queue);
    }

    async fn update(
        &mut self,
        ui_event: &super::UIEvent,
        graphics: &Graphics,
    ) -> anyhow::Result<MenuEvent> {
        Ok(MenuEvent::None)
    }
}

impl Drawable for GameMenu {
    fn render(&self, render_pass: &mut wgpu::RenderPass) {
        self.root.render(render_pass);
    }
}
