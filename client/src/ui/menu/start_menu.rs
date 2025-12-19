use std::process::exit;

use wgpu::Queue;

use crate::{
    engine::{Drawable, Graphics, TextureType},
    ui::{
        Menu, MenuEvent, UIAlign, UIButton, UIComponentEvent, UIComponentID, UIComponentItem,
        UIEvent, UILayout, UIRoot, UIRootItem, UISize,
    },
};

pub struct StartMenu {
    root: Box<dyn UIRoot + 'static>,
    window_size: (f32, f32),
    cursor_location: (f32, f32),
}

impl StartMenu {
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

        let play_button: UIButton = UIButton::builder()
            .id(UIComponentID::Name("play_button".into()))
            .default(TextureType::Color([255, 0, 0, 255]))
            .hovered(TextureType::Color([255, 255, 255, 255]))
            .pressed(TextureType::Color([0, 255, 255, 255]))
            .size(UISize::PercentParent(0.75, 0.15))
            .margin(0.0125)
            .build(device, queue, layout)?;

        let host_button: UIButton = UIButton::builder()
            .id(UIComponentID::Name("host_button".into()))
            .default(TextureType::Color([0, 255, 0, 255]))
            .hovered(TextureType::Color([255, 255, 255, 255]))
            .pressed(TextureType::Color([0, 255, 255, 255]))
            .size(UISize::PercentParent(0.75, 0.15))
            .margin(0.0125)
            .build(device, queue, layout)?;

        let credits_button: UIButton = UIButton::builder()
            .id(UIComponentID::Name("credits_button".into()))
            .default(TextureType::Color([255, 255, 0, 255]))
            .hovered(TextureType::Color([255, 255, 255, 255]))
            .pressed(TextureType::Color([0, 255, 255, 255]))
            .size(UISize::PercentParent(0.725, 1.0))
            .margin(0.0125)
            .build(device, queue, layout)?;

        let settings_button: UIButton = UIButton::builder()
            .id(UIComponentID::Name("settings_button".into()))
            .default(TextureType::Color([0, 255, 255, 255]))
            .hovered(TextureType::Color([255, 255, 255, 255]))
            .pressed(TextureType::Color([0, 255, 255, 255]))
            .size(UISize::PercentParent(0.25, 1.0))
            .margin(0.0125)
            .build(device, queue, layout)?;

        let settings_credits_group: UIComponentItem = UIComponentItem::builder()
            .background(TextureType::Color([0, 0, 0, 0]))
            .size(UISize::PercentParent(0.75, 0.15))
            .align_children(UIAlign::Horizontal)
            .add_child(Box::new(credits_button))
            .add_child(Box::new(settings_button))
            .margin(0.0125)
            .build(device, queue, layout)?;

        let exit_button: UIButton = UIButton::builder()
            .id(UIComponentID::Name("exit_button".into()))
            .default(TextureType::Color([75, 255, 150, 255]))
            .hovered(TextureType::Color([255, 255, 255, 255]))
            .pressed(TextureType::Color([0, 255, 255, 255]))
            .size(UISize::PercentParent(0.75, 0.15))
            .margin(0.0125)
            .build(device, queue, layout)?;

        let child: UIComponentItem = UIComponentItem::builder()
            .background(TextureType::Color([255, 0, 255, 255]))
            .align_children(UIAlign::Vertical)
            .layout_children(UILayout::MC)
            .size(UISize::PercentParent(0.5, 0.5))
            .add_child(Box::new(play_button))
            .add_child(Box::new(host_button))
            .add_child(Box::new(settings_credits_group))
            .add_child(Box::new(exit_button))
            .build(device, queue, layout)?;

        let mut root: UIRootItem = UIRootItem::builder()
            .background(TextureType::Color([0, 0, 0, 255]))
            .add_child(Box::new(child))
            .build(device, queue, layout)?;

        root.compute_layout(size, queue);

        let _ = root.handle_event(
            &UIEvent::MouseMoved(mouse_position.0, mouse_position.1),
            queue,
        );

        Ok(Self {
            root: Box::new(root),
            window_size: size,
            cursor_location: mouse_position,
        })
    }
}

impl Menu for StartMenu {
    fn handle_input(
        &mut self,
        ui_event: &UIEvent,
        graphics: &Graphics,
    ) -> anyhow::Result<MenuEvent> {
        let Graphics { queue, .. } = graphics;
        let events = self.root.handle_event(ui_event, queue);
        for event in events {
            match event {
                UIComponentEvent::Clicked(id) => {
                    if let UIComponentID::Name(id) = id {
                        if id == "play_button" {
                            return Ok(MenuEvent::SwitchToLoadGame);
                        } else if id == "exit_button" {
                            exit(0);
                        }
                    }
                }
            }
        }
        Ok(MenuEvent::None)
    }

    fn update(&mut self, graphics: &Graphics) -> anyhow::Result<MenuEvent> {
        Ok(MenuEvent::None)
    }

    fn handle_resize(&mut self, queue: &Queue, width: f32, height: f32) {
        self.root.compute_layout((width, height), queue);
    }
}

impl Drawable for StartMenu {
    fn render(&self, render_pass: &mut wgpu::RenderPass) {
        self.root.render(render_pass);
    }
}
