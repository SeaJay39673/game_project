use std::collections::HashSet;

use wgpu::{BindGroupLayout, Device, Queue};
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, MouseButton},
    keyboard::{NamedKey, SmolStr},
};

use crate::{
    engine::{Drawable, TextureType},
    ui::{
        UIAlign, UIButton, UIComponent, UIComponentEvent, UIComponentID, UIComponentItem, UIRect, UISize, component
    },
};

#[derive(Clone)]
pub enum UIEvent<'a> {
    MouseMoved(f32, f32),
    MouseClicked {
        position: (f32, f32),
        state: ElementState,
        button: MouseButton,
    },
    KeyEntered {
        pressed_named_keys: &'a HashSet<NamedKey>,
        pressed_keys: &'a HashSet<SmolStr>,
    },
}

pub trait Menu: Drawable {
    fn handle_ui_event(&mut self, event: UIEvent, queue: &Queue);
    fn handle_resize(&mut self, queue: &Queue, width: f32, height: f32);
}

pub struct StartMenu {
    root: Box<dyn UIComponent>,
}

impl StartMenu {
    pub fn new(device: &Device, queue: &Queue, layout: &BindGroupLayout) -> anyhow::Result<Self> {
        let submit_button = UIButton::builder()
            .id(UIComponentID::Name("Submit_Button".into()))
            .default(TextureType::Color([150, 255, 255, 255]))
            .hovered(TextureType::Color([255, 255, 255, 255]))
            .pressed(TextureType::Color([0, 255, 0, 255]))
            .build(device, queue, layout)?;
        let button2 = UIButton::builder()
            .default(TextureType::Color([255, 0, 0, 255]))
            .hovered(TextureType::Color([0, 255, 0, 255]))
            .pressed(TextureType::Color([150, 255, 255, 255]))
            .build(device, queue, layout)?;
        let child = UIComponentItem::builder()
            .align(UIAlign::Vertical)
            .background(TextureType::Color([255, 0, 255, 255]))
            .size(UISize::PercentParent(0.5, 0.5))
            .add_child(Box::new(submit_button))
            .add_child(Box::new(button2))
            .build(device, queue, layout)?;
        let component = UIComponentItem::builder()
            .background(TextureType::Color([0, 0, 0, 255]))
            .add_child(Box::new(child))
            .build(device, queue, layout)?;
        Ok(Self {
            root: Box::new(component),
        })
    }
}

impl Menu for StartMenu {
    fn handle_ui_event(&mut self, event: UIEvent, queue: &Queue) {
        let events = self.root.handle_event(event, queue);
        use UIComponentEvent::*;
        for event in events {
            match event {
                Clicked(id) => {
                    if let UIComponentID::Name(name) = id && name == "Submit_Button" {
                        println!("Submit button clicked!")
                    }
                }
            }
        }
    }

    fn handle_resize(&mut self, queue: &Queue, width: f32, height: f32) {
        self.root.compute_layout(
            UIRect {
                x: 0.0,
                y: 0.0,
                w: width,
                h: height,
            },
            (width, height),
            queue,
        );
    }
}

impl Drawable for StartMenu {
    fn render(&self, render_pass: &mut wgpu::RenderPass) {
        self.root.render(render_pass);
    }
}
