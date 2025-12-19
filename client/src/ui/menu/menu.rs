use std::sync::Arc;

use wgpu::Queue;
use winit::event::{ElementState, MouseButton};

use crate::{
    engine::{Drawable, Graphics}, server_state::ServerState, ui::{GameMenu, LoadGameMenu, StartMenu}
};

pub enum UIEvent {
    None,
    MouseMoved(f32, f32),
    MouseClicked {
        position: (f32, f32),
        state: ElementState,
        button: MouseButton,
    },
}

pub enum MenuEvent {
    None,
    SwitchToStart,
    SwitchToLoadGame,
    // SwitchToGame(Arc<ServerState>),
}

pub enum ActiveMenu {
    Start(StartMenu),
    LoadGame(LoadGameMenu),
    Game(GameMenu),
}

pub trait Menu: Drawable {
    fn update(
        &mut self,
        graphics: &Graphics,
    ) -> anyhow::Result<MenuEvent>;
    fn handle_input(
        &mut self,
        ui_event: &UIEvent,
        graphics: &Graphics
    ) -> anyhow::Result<MenuEvent>;
    fn handle_resize(&mut self, queue: &Queue, width: f32, height: f32);
}

pub struct MenuManager {
    menu: ActiveMenu,
    size: (f32, f32),
    mouse_position: (f32, f32),
}

impl MenuManager {
    pub fn new(
        graphics: &Graphics,
        size: (f32, f32),
        mouse_position: (f32, f32),
    ) -> anyhow::Result<Self> {
        Ok(Self {
            menu: ActiveMenu::Start(StartMenu::new(graphics, size, mouse_position)?),
            size,
            mouse_position,
        })
    }
}

impl Menu for MenuManager {
    fn handle_resize(&mut self, queue: &Queue, width: f32, height: f32) {
        match &mut self.menu {
            ActiveMenu::Start(m) => m.handle_resize(queue, width, height),
            ActiveMenu::LoadGame(m) => m.handle_resize(queue, width, height),
            ActiveMenu::Game(m) => m.handle_resize(queue, width, height),
        }
    }

    fn handle_input(
            &mut self,
            ui_event: &UIEvent,
            graphics: &Graphics
        ) -> anyhow::Result<MenuEvent> {
        let event = match &mut self.menu {
            ActiveMenu::Start(m) => m.handle_input(ui_event, graphics)?,
            ActiveMenu::LoadGame(m) => m.handle_input(ui_event, graphics)?,
            ActiveMenu::Game(m) => m.handle_input(ui_event, graphics)?
        };

        match event {
            MenuEvent::None => {}
            MenuEvent::SwitchToStart => self.menu = ActiveMenu::Start(StartMenu::new(graphics, self.size, self.mouse_position)?),
            MenuEvent::SwitchToLoadGame => self.menu = ActiveMenu::LoadGame(LoadGameMenu::new(graphics, self.size, self.mouse_position)?),
        };

        Ok(MenuEvent::None)
    }

    fn update(
        &mut self,
        graphics: &Graphics,
    ) -> anyhow::Result<MenuEvent> {
        let event = match &mut self.menu {
            ActiveMenu::Start(m) => m.update(graphics)?,
            ActiveMenu::LoadGame(m) => m.update(graphics)?,
            ActiveMenu::Game(m) => m.update(graphics)?,
        };

        match event {
            MenuEvent::None => {}
            MenuEvent::SwitchToStart => {
                self.menu = ActiveMenu::Start(StartMenu::new(graphics, self.size, self.mouse_position)?);
            }
            MenuEvent::SwitchToLoadGame => {
                self.menu = ActiveMenu::LoadGame(LoadGameMenu::new(
                    graphics,
                    self.size,
                    self.mouse_position,
                )?);
            }
        }

        Ok(MenuEvent::None)
    }
}

impl Drawable for MenuManager {
    fn render(&self, render_pass: &mut wgpu::RenderPass) {
        match &self.menu {
            ActiveMenu::Start(m) => m.render(render_pass),
            ActiveMenu::LoadGame(m) => m.render(render_pass),
            ActiveMenu::Game(m) => m.render(render_pass),
        }
    }
}
