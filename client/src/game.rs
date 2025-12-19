use std::{
    collections::{HashSet},
    sync::Arc,
    time::{Duration, Instant},
};

use winit::{
    application::ApplicationHandler,
    event::{StartCause, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{Key, NamedKey, SmolStr},
    window::{Fullscreen, Window},
};

use crate::{
    asset_ingestion::load_assets,
    engine::Graphics,
    ui::{Menu, MenuManager, UIEvent},
};

struct GameManager {
    last_frame: Instant,
    target_frame_duration: Duration,

    window: Option<Arc<Window>>,
    fullscreen: bool,

    graphics: Option<Graphics>,

    pressed_named_keys: HashSet<NamedKey>,
    pressed_keys: HashSet<SmolStr>,
    cursor_location: (f32, f32),

    menu: Option<MenuManager>,
}

impl GameManager {
    pub async fn new() -> anyhow::Result<Self> {
        Ok(Self {
            last_frame: Instant::now(),
            target_frame_duration: Duration::from_secs_f64(1.0 / 120.0),

            window: None,
            fullscreen: false,

            graphics: None,

            pressed_named_keys: HashSet::new(),
            pressed_keys: HashSet::new(),
            cursor_location: (0.0, 0.0),

            menu: None,
        })
    }

    pub fn handle_named_key(&mut self, key: NamedKey, pressed: bool) {
        if pressed {
            self.pressed_named_keys.insert(key);
        } else {
            self.pressed_named_keys.remove(&key);
        }
    }

    pub fn handle_key(&mut self, key: SmolStr, pressed: bool) {
        if pressed {
            self.pressed_keys.insert(key);
        } else {
            self.pressed_keys.remove(&key);
        }
    }

    pub fn handle_cursor_location(&mut self, location: (f32, f32)) {
        self.cursor_location = location;
    }

    pub fn update_window(&mut self) {
        if self.pressed_named_keys.contains(&NamedKey::F11) {
            if let Some(ref window) = self.window {
                if self.fullscreen {
                    window.set_fullscreen(None);
                } else if let Some(monitor) = window.current_monitor() {
                    window.set_fullscreen(Some(Fullscreen::Borderless(Some(monitor))));
                }
                self.fullscreen = !self.fullscreen;
            }
        }
        if self.pressed_named_keys.contains(&NamedKey::Escape) {
            if let Some(ref window) = self.window {
                if self.fullscreen {
                    window.set_fullscreen(None);
                    self.fullscreen = false;
                } else {
                    if window.is_maximized() {
                        window.set_maximized(false);
                    } else {
                        window.set_maximized(true);
                    }
                }
            }
        }
    }
}

impl ApplicationHandler for GameManager {
    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        let now = Instant::now();
        let next_frame_time = self.last_frame + self.target_frame_duration;

        if let (Some(graphics), Some(menu)) = (&self.graphics, &mut self.menu) {
            if let Err(e) = menu.update(graphics) {
                eprintln!("Error updating menu: {e}");
            }
        } 

        if now >= next_frame_time || matches!(cause, StartCause::Init) {
            if let Some(window) = &self.window {
                window.request_redraw();
            }
            self.last_frame = now;
        }
        event_loop.set_control_flow(ControlFlow::WaitUntil(
            self.last_frame + self.target_frame_duration,
        ));
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            if let Ok(window) = event_loop.create_window(
                Window::default_attributes()
                    .with_title("Isometric Game!")
                    .with_maximized(true)
                    .with_visible(false),
            ) {
                self.window = Some(Arc::new(window));
            }
        }

        if self.graphics.is_none() {
            if let Some(ref window) = self.window {
                match pollster::block_on(Graphics::new(window)) {
                    Ok(graphics) => {
                        self.graphics = Some(graphics);
                    }
                    Err(e) => {
                        panic!("Could not create graphics: {e}");
                    }
                }
            }
        }

        if self.menu.is_none() {
            if let (Some(window), Some(graphics)) = (&self.window, &self.graphics) {
                let size = window.inner_size();
                match MenuManager::new(graphics, (size.width as f32, size.height as f32), self.cursor_location) {
                    Ok(menu) => self.menu = Some(menu),
                    Err(e) => panic!("Could not create MenuManager: {e}"),
                }
            }
        }

        if self.graphics.is_some()
            && self.menu.is_some()
            && let Some(ref window) = self.window
        {
            window.set_visible(true);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::KeyboardInput { event, .. } => {
                if let Key::Named(key) = event.logical_key {
                    self.handle_named_key(key, event.state.is_pressed());
                }
                if let Key::Character(ch) = event.logical_key {
                    self.handle_key(ch, event.state.is_pressed());
                }
                self.update_window();
            }
            WindowEvent::CursorMoved {
                device_id: _,
                position,
            } => {
                self.handle_cursor_location((position.x as f32, position.y as f32));
                if let (Some(graphics), Some(menu)) = (&mut self.graphics, &mut self.menu) {
                    let _ = menu.handle_input(&UIEvent::MouseMoved(self.cursor_location.0, self.cursor_location.1), graphics);
                }
            }
            WindowEvent::MouseInput {
                device_id: _,
                state,
                button,
            } => {
                if let (Some(graphics), Some(menu)) =
                    (&mut self.graphics, &mut self.menu)
                {
                    let _ = menu.handle_input(&UIEvent::MouseClicked { position: self.cursor_location, state, button }, graphics);
                }
            }
            WindowEvent::RedrawRequested => {
                if let (Some(graphics), Some(menu)) = (&mut self.graphics, &mut self.menu) {
                    if let Err(e) = graphics.render(&[menu]) {
                        eprintln!("Could not render menu: {e}")
                    }
                }
            }
            WindowEvent::Resized(size) => {
                if let Some(ref mut graphics) = self.graphics {
                    graphics.resize(size.width, size.height);
                    let Graphics { queue, .. } = graphics;
                    if let Some(ref mut menu) = self.menu {
                        menu.handle_resize(queue, size.width as f32, size.height as f32);
                    }
                }
            }
            _ => {}
        }
    }
}

pub struct Game {
    event_loop: EventLoop<()>,
    game_manager: GameManager,
}

impl Game {
    pub fn new() -> anyhow::Result<Self> {
        let event_loop = EventLoop::new()?;
        event_loop.set_control_flow(ControlFlow::Poll);
        let game_manager = pollster::block_on(GameManager::new())?;
        Ok(Self {
            event_loop,
            game_manager,
        })
    }

    pub fn run(mut self) -> anyhow::Result<()> {
        self.event_loop.run_app(&mut self.game_manager)?;
        Ok(())
    }
}
