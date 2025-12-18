use wgpu::{BindGroupLayout, Device, Queue};
use winit::event::{ElementState, MouseButton};

use crate::{
    engine::{Drawable, GraphicsData, TextureManager, TextureType},
    ui::{UIComponent, UIComponentEvent, UIComponentID, UIEvent, UIMargin, UIRect, UISize},
};

#[derive(PartialEq, Eq)]
pub enum ButtonState {
    Default,
    Hovered,
    Pressed,
}

pub struct UIButton {
    id: UIComponentID,
    rect: UIRect,
    state: ButtonState,
    size: UISize,
    margin: UIMargin,

    gfx_default: GraphicsData,
    gfx_hover: Option<GraphicsData>,
    gfx_pressed: Option<GraphicsData>,
}

pub struct UIButtonBuilder {
    id: Option<UIComponentID>,
    rect: UIRect,
    state: ButtonState,
    size: UISize,
    margin: UIMargin,

    default_texture: Option<TextureType>,
    hovered_texture: Option<TextureType>,
    pressed_texture: Option<TextureType>,
}

impl UIButtonBuilder {
    pub fn id(mut self, id: UIComponentID) -> Self {
        self.id = Some(id);
        self
    }

    pub fn size(mut self, size: UISize) -> Self {
        self.size = size;
        self
    }

    pub fn margin(mut self, margin: f32) -> Self {
        self.margin = UIMargin::new(margin, margin, margin, margin);
        self
    }

    pub fn default(mut self, default: TextureType) -> Self {
        self.default_texture = Some(default);
        self
    }

    pub fn hovered(mut self, hovered: TextureType) -> Self {
        self.hovered_texture = Some(hovered);
        self
    }

    pub fn pressed(mut self, pressed: TextureType) -> Self {
        self.pressed_texture = Some(pressed);
        self
    }

    pub fn build(
        self,
        device: &Device,
        queue: &Queue,
        layout: &BindGroupLayout,
    ) -> anyhow::Result<UIButton> {
        let id = match self.id {
            Some(id) => id,
            None => UIComponentID::new_id(),
        };
        let default = match self.default_texture {
            Some(default_texture) => default_texture,
            None => TextureType::Color([255, 0, 255, 255]),
        };
        let default_tex_asset_data =
            TextureManager::get_texture_asset_data(device, queue, default)?;
        let gfx_default: GraphicsData =
            GraphicsData::new(device, layout, [0.0, 0.0, 0.0], default_tex_asset_data, 0.0);

        let gfx_hover = match self.hovered_texture {
            Some(hovered) => {
                let hovered_tex_asset_data =
                    TextureManager::get_texture_asset_data(device, queue, hovered)?;
                let gfx_hovered: GraphicsData =
                    GraphicsData::new(device, layout, [0.0, 0.0, 0.0], hovered_tex_asset_data, 0.0);
                Some(gfx_hovered)
            }
            None => None,
        };

        let gfx_pressed = match self.pressed_texture {
            Some(pressed) => {
                let pressed_tex_data =
                    TextureManager::get_texture_asset_data(device, queue, pressed)?;
                Some(GraphicsData::new(
                    device,
                    layout,
                    [0.0, 0.0, 0.0],
                    pressed_tex_data,
                    0.0,
                ))
            }
            None => None,
        };

        Ok(UIButton {
            id,
            rect: self.rect,
            state: self.state,
            size: self.size,
            margin: self.margin,
            gfx_default,
            gfx_hover,
            gfx_pressed,
        })
    }
}

impl UIButton {
    pub fn builder() -> UIButtonBuilder {
        UIButtonBuilder {
            id: None,
            rect: UIRect {
                x: 0.0,
                y: 0.0,
                w: 0.0,
                h: 0.0,
            },
            state: ButtonState::Default,
            size: UISize::PercentParent(0.5, 0.5),
            margin: UIMargin::ZERO,
            default_texture: None,
            hovered_texture: None,
            pressed_texture: None,
        }
    }
}

impl UIComponent for UIButton {
    fn get_id(&self) -> UIComponentID {
        self.id.clone()
    }

    fn get_size(&self) -> UISize {
        self.size
    }

    fn get_margin(&self) -> UIMargin {
        self.margin
    }

    fn compute_layout(&mut self, rect: UIRect, window_size: (f32, f32), queue: &Queue) {
        let UIRect { x, y, w, h } = rect;

        self.rect = rect;

        self.gfx_default.set_rect(queue, x, y, w, h, window_size);

        if let Some(ref mut hovered) = self.gfx_hover {
            hovered.set_rect(queue, x, y, w, h, window_size);
        }

        if let Some(ref mut pressed) = self.gfx_pressed {
            pressed.set_rect(queue, x, y, w, h, window_size);
        }
    }

    fn layout_children(&mut self, _rect: UIRect, _window_size: (f32, f32), _queue: &Queue) {}

    fn handle_event(&mut self, event: &UIEvent, _queue: &Queue) -> Vec<super::UIComponentEvent> {
        match *event {
            UIEvent::MouseMoved(x, y) => {
                if self.rect.contains(x as f32, y as f32) {
                    if self.state != ButtonState::Pressed {
                        self.state = ButtonState::Hovered;
                    }
                } else {
                    self.state = ButtonState::Default;
                }
            }
            UIEvent::MouseClicked {
                position,
                state,
                button,
            } => {
                if self.rect.contains(position.0, position.1)
                    && button == MouseButton::Left
                    && state == ElementState::Pressed
                {
                    self.state = ButtonState::Pressed;
                } else if self.state == ButtonState::Pressed
                    && button == MouseButton::Left
                    && state == ElementState::Released
                {
                    self.state = ButtonState::Hovered;
                    return vec![UIComponentEvent::Clicked(self.id.clone())];
                }
            }
            _ => {}
        }
        vec![]
    }
}

impl Drawable for UIButton {
    fn render(&self, render_pass: &mut wgpu::RenderPass) {
        match self.state {
            ButtonState::Default => self.gfx_default.render(render_pass),
            ButtonState::Hovered => {
                if let Some(ref hovered) = self.gfx_hover {
                    hovered.render(render_pass);
                } else {
                    self.gfx_default.render(render_pass);
                }
            }
            ButtonState::Pressed => {
                if let Some(ref pressed) = self.gfx_pressed {
                    pressed.render(render_pass);
                } else {
                    self.gfx_default.render(render_pass);
                }
            }
        }
    }
}
