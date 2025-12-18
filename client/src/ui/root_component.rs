use wgpu::{BindGroupLayout, Device, Queue};

use crate::{
    engine::{Drawable, GraphicsData, TextureAssetData, TextureManager, TextureType},
    ui::{UIComponent, UIComponentEvent, UIComponentID, UIEvent, UIRect},
};

pub trait UIRoot: Drawable {
    fn compute_layout(&mut self, window_size: (f32, f32), queue: &Queue);
    fn handle_event(&mut self, event: &UIEvent, queue: &Queue) -> Vec<UIComponentEvent>;
}

pub struct UIRootItem {
    id: UIComponentID,
    graphics_data: GraphicsData,
    children: Vec<Box<dyn UIComponent + 'static>>,
}

pub struct UIRootBuilder {
    background: Option<TextureType>,
    children: Vec<Box<dyn UIComponent + 'static>>,
}

impl UIRootBuilder {
    pub fn background(mut self, background: TextureType) -> Self {
        self.background = Some(background);
        self
    }
    pub fn add_child(mut self, child: Box<dyn UIComponent + 'static>) -> Self {
        self.children.push(child);
        self
    }
    pub fn build(
        self,
        device: &Device,
        queue: &Queue,
        layout: &BindGroupLayout,
    ) -> anyhow::Result<UIRootItem> {
        let texture: TextureAssetData = match self.background {
            Some(background) => TextureManager::get_texture_asset_data(device, queue, background)?,
            None => TextureManager::get_texture_asset_data(
                device,
                queue,
                TextureType::Color([0, 0, 0, 0]),
            )?,
        };
        let graphics_data: GraphicsData =
            GraphicsData::new(device, layout, [0.0, 0.0, 0.0], texture, 2.0);

        let id = UIComponentID::Name("root".into());

        Ok(UIRootItem {
            id,
            graphics_data,
            children: self.children,
        })
    }
}

impl UIRootItem {
    pub fn builder() -> UIRootBuilder {
        UIRootBuilder {
            background: None,
            children: vec![],
        }
    }
}

impl UIRoot for UIRootItem {
    fn compute_layout(&mut self, window_size: (f32, f32), queue: &Queue) {
        let rect: UIRect = UIRect::new((0.0, 0.0), window_size);
        let UIRect { x, y, w, h } = rect;
        self.graphics_data.set_rect(queue, x, y, w, h, window_size);

        for child in &mut self.children {
            let child_rect = rect.compute_rect(child.get_size(), super::UILayout::MC);
            child.compute_layout(child_rect, window_size, queue);
        }
    }

    fn handle_event(&mut self, event: &UIEvent, queue: &Queue) -> Vec<UIComponentEvent> {
        let mut events: Vec<UIComponentEvent> = vec![];
        for child in &mut self.children {
            events.extend_from_slice(&child.handle_event(event, queue));
        }

        events
    }
}

impl Drawable for UIRootItem {
    fn render(&self, render_pass: &mut wgpu::RenderPass) {
        self.graphics_data.render(render_pass);

        for child in &self.children {
            child.render(render_pass);
        }
    }
}