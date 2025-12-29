use std::{
    collections::HashSet,
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

use crate::{graphics::Graphics, mesh::ChunkMeshes};

struct GameManager {
    last_frame: Instant,
    target_frame_duration: Duration,

    window: Option<Arc<Window>>,
    fullscreen: bool,

    graphics: Option<Graphics>,

    pressed_named_keys: HashSet<NamedKey>,
    pressed_keys: HashSet<SmolStr>,
    cursor_location: (f32, f32),

    chunks: Option<ChunkMeshes>,
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
            chunks: None,
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

        if self.chunks.is_none()
            && let Some(graphics) = &self.graphics
        {
            match ChunkMeshes::new(graphics, 1, 8, 0.1) {
                Ok(meshes) => self.chunks = Some(meshes),
                Err(e) => {
                    eprintln!("Could not create chunk meshes: {e}")
                }
            };
        }

        if self.graphics.is_some()
            && self.chunks.is_some()
            && let Some(window) = &self.window
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
            }
            WindowEvent::MouseInput {
                device_id: _,
                state,
                button,
            } => {}
            WindowEvent::RedrawRequested => {
                if let (Some(graphics), Some(chunks)) = (&mut self.graphics, &self.chunks) {
                    if let Err(e) = graphics.render(Some(chunks)) {
                        eprintln!("Error rendering chunks: {e}");
                    };
                }
            }
            WindowEvent::Resized(size) => {
                if let Some(ref mut graphics) = self.graphics {
                    graphics.resize(size.width, size.height);
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
