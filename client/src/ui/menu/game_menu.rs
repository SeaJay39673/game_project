use std::sync::{Arc};

use server_lib::GameStartOption;
use shared::{
    AccountCredentials, AccountInfo, ClientControlStreamMessage, ServerControlStreamMessage
};
use tokio::sync::mpsc::error::TryRecvError;

use crate::{
    engine::{Drawable, Graphics}, server_state::ServerState, ui::{Menu, MenuEvent, UIComponentEvent, UIComponentID, UIRoot, UIRootItem}
};

pub struct LoadGameMenu {
    root: Box<dyn UIRoot + 'static>,
    window_size: (f32, f32),
    cursor_location: (f32, f32),
    server_state: Arc<ServerState>,
}

impl LoadGameMenu {
    pub fn new(
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

        let server_state = pollster::block_on(ServerState::new(GameStartOption::LoadGame("Blah".into())));

        Ok(Self {
            root: Box::new(root),
            window_size: size,
            cursor_location: mouse_position,
            server_state,
        })
    }
}

enum ProcessedMessage {
    None,
    Shutdown(String),
}

impl Menu for LoadGameMenu {
    fn handle_resize(&mut self, queue: &wgpu::Queue, width: f32, height: f32) {
        self.root.compute_layout((width, height), queue);
    }

    fn handle_input(
            &mut self,
            ui_event: &super::UIEvent,
            graphics: &Graphics
        ) -> anyhow::Result<MenuEvent> {
        let _ = self.root.handle_event(ui_event, &graphics.queue);

        Ok(MenuEvent::None)
    }

    fn update(
        &mut self,
        _graphics: &Graphics,
    ) -> anyhow::Result<MenuEvent> {
        use ServerControlStreamMessage::*;
        use ClientControlStreamMessage::*;

        if self.server_state.is_shutdown() {
            return Ok(MenuEvent::SwitchToStart);
        }
        for result in self.server_state.clone().receive_messages()? {
            let message: ProcessedMessage = match result {
                Ok(Connected) => {
                    let _ = self.server_state.clone().send_message(Login(AccountCredentials::new("Test".into(), "Test".into(), None)));
                    ProcessedMessage::None
                }
                Ok(Authenticated(account_info)) => {
                    let AccountInfo { characters, ..} = account_info;
                    if characters.len() > 0 {
                        let _ = self.server_state.clone().send_message(SelectCharacter(characters[0].character_id));
                        // let _ = self.server_state.clone().send_message(CreateCharacter("TestCharacter".into()));
                    }else {
                        let _ = self.server_state.clone().send_message(CreateCharacter("TestCharacter".into()));
                    }
                    ProcessedMessage::None
                }
                Ok(LoginDenied(reason)) => {
                    eprintln!("Client denied from logging in: {reason}");
                    ProcessedMessage::None},
                Ok(AccountCreateDenied(reason)) => {
                    eprintln!("Client denied from creating account: {reason}");
                    ProcessedMessage::None
                }
                Ok(CharacterSelected(character_id)) => {
                    println!("Character Selected: {character_id}");
                    ProcessedMessage::None
                },
                Ok(CharacterDenied(reason)) => {
                    eprintln!("Client denied from selecting character: {reason}");
                    ProcessedMessage::None
                }
                Ok(Disconnected(reason)) => {
                    ProcessedMessage::Shutdown(reason)
                }
                Err(TryRecvError::Disconnected) => ProcessedMessage::Shutdown("Client disconnected from server".into()),
                Err(_) => ProcessedMessage::None,
            };

            match message {
                ProcessedMessage::None => {},
                ProcessedMessage::Shutdown(reason) => {
                    eprintln!("{reason}");
                    pollster::block_on(self.server_state.clone().shutdown());
                }
            }

        }
    
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
    server_state: Arc<ServerState>,
}

impl GameMenu {
    pub async fn new(
        server_state: Arc<ServerState>,
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

    fn handle_input(
            &mut self,
            ui_event: &super::UIEvent,
            graphics: &Graphics
        ) -> anyhow::Result<MenuEvent> {
        Ok(MenuEvent::None)
    }

    fn update(
        &mut self,
        _graphics: &Graphics,
    ) -> anyhow::Result<MenuEvent> {
        Ok(MenuEvent::None)
    }
}

impl Drawable for GameMenu {
    fn render(&self, render_pass: &mut wgpu::RenderPass) {
        self.root.render(render_pass);
    }
}
