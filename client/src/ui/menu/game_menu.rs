use std::sync::Arc;

use server_lib::GameStartOption;
use shared::{
    AccountCredentials, AccountInfo, ClientControlStreamMessage, ServerControlStreamMessage,
};
use tokio::sync::mpsc::error::TryRecvError;

use crate::{
    engine::{Drawable, Graphics},
    server_state::ServerState,
    ui::{Menu, MenuEvent, UIComponentEvent, UIComponentID, UIRoot, UIRootItem},
};

pub struct GameMenu {
    root: Box<dyn UIRoot + 'static>,
    window_size: (f32, f32),
    cursor_location: (f32, f32),
    server_state: Arc<ServerState>,
}

impl GameMenu {
    pub fn new(
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

        if let Err(e) = server_state.clone().send_message(ClientControlStreamMessage::JoinWorldRequest) {
            eprintln!("Error requesting to join the world: {e}\nShutting down.");
            pollster::block_on(server_state.clone().shutdown());
        }

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
        graphics: &Graphics,
    ) -> anyhow::Result<MenuEvent> {
        let _: Vec<UIComponentEvent> = self.root.handle_event(ui_event, &graphics.queue);
        
        Ok(MenuEvent::None)
    }

    fn update(&mut self, _graphics: &Graphics) -> anyhow::Result<MenuEvent> {
        Ok(MenuEvent::None)
    }
}

impl Drawable for GameMenu {
    fn render(&self, render_pass: &mut wgpu::RenderPass) {
        self.root.render(render_pass);
    }
}
