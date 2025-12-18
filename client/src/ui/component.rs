use wgpu::{BindGroupLayout, Device, Queue};

use crate::{
    engine::{Drawable, GraphicsData, TextureManager, TextureType},
    ui::{
        UIAlign, UIComponent, UIComponentEvent, UIComponentID, UIEvent, UILayout, UIMargin, UIRect,
        UISize,
    },
};

pub struct UIComponentItem {
    graphics_data: GraphicsData,
    id: UIComponentID,
    size: UISize,
    margin: UIMargin,
    layout_children: UILayout,
    align_children: UIAlign,
    children: Vec<Box<dyn UIComponent + 'static>>,
}

pub struct UIComponentBuilder {
    id: Option<UIComponentID>,
    background: Option<TextureType>,
    size: UISize,
    margin: UIMargin,
    layout_children: UILayout,
    align_children: UIAlign,
    children: Vec<Box<dyn UIComponent + 'static>>,
}

impl UIComponentBuilder {
    pub fn id(mut self, id: UIComponentID) -> Self {
        self.id = Some(id);
        self
    }
    pub fn background(mut self, background: TextureType) -> Self {
        self.background = Some(background);
        self
    }
    pub fn size(mut self, size: UISize) -> Self {
        self.size = size;
        self
    }
    pub fn layout_children(mut self, layout_children: UILayout) -> Self {
        self.layout_children = layout_children;
        self
    }
    pub fn align_children(mut self, align_children: UIAlign) -> Self {
        self.align_children = align_children;
        self
    }
    pub fn add_child(mut self, child: Box<dyn UIComponent + 'static>) -> Self {
        self.children.push(child);
        self
    }

    pub fn margin(mut self, margin: f32) -> Self {
        self.margin = UIMargin::new(margin, margin, margin, margin);
        self
    }

    pub fn build(
        self,
        device: &Device,
        queue: &Queue,
        layout: &BindGroupLayout,
    ) -> anyhow::Result<UIComponentItem> {
        let id = match self.id {
            Some(id) => id,
            None => UIComponentID::new_id(),
        };

        let background = match self.background {
            Some(background) => TextureManager::get_texture_asset_data(device, queue, background)?,
            None => TextureManager::get_texture_asset_data(
                device,
                queue,
                TextureType::Color([0, 0, 0, 255]),
            )?,
        };

        let graphics_data: GraphicsData =
            GraphicsData::new(device, layout, [0.0, 0.0, 0.0], background, 0.0);

        Ok(UIComponentItem {
            id,
            graphics_data,
            size: self.size,
            margin: self.margin,
            layout_children: self.layout_children,
            align_children: self.align_children,
            children: self.children,
        })
    }
}

impl UIComponentItem {
    pub fn builder() -> UIComponentBuilder {
        UIComponentBuilder {
            id: None,
            background: None,
            size: UISize::PercentParent(1.0, 1.0),
            margin: UIMargin::ZERO,
            layout_children: UILayout::MC,
            align_children: UIAlign::Overlay,
            children: vec![],
        }
    }
}

impl UIComponent for UIComponentItem {
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
        self.graphics_data.set_rect(queue, x, y, w, h, window_size);

        self.layout_children(rect, window_size, queue);
    }

    fn layout_children(&mut self, rect: UIRect, window_size: (f32, f32), queue: &Queue) {
        match self.align_children {
            UIAlign::Overlay => {
                for child in &mut self.children {
                    let child_rect = rect.compute_rect(child.get_size(), self.layout_children);
                    child.compute_layout(child_rect, window_size, queue);
                }
            }
            UIAlign::Vertical => {
                let mut rects: Vec<(UIRect, UIRect)> = vec![];
                let mut total_size: f32 = 0.0;

                for child in &mut self.children {
                    let child_rect = rect.compute_rect(child.get_size(), self.layout_children);

                    let margin_rect = child_rect.apply_margin(rect, child.get_margin());

                    total_size += margin_rect.h;
                    rects.push((child_rect, margin_rect));
                }

                let mut current_y: f32 = rect.y + (rect.h - total_size) / 2.0;

                for (child, (child_rect, margin_rect)) in
                    self.children.iter_mut().zip(rects.iter_mut())
                {
                    margin_rect.y = current_y;
                    current_y += margin_rect.h;
                    let final_rect = UIRect {
                        x: child_rect.x + rect.w * child.get_margin().left,
                        y: margin_rect.y,
                        w: child_rect.w,
                        h: child_rect.h,
                    };
                    child.compute_layout(final_rect, window_size, queue);
                }
            }
            UIAlign::Horizontal => {
                let mut rects: Vec<(UIRect, UIRect)> = vec![];
                let mut total_size: f32 = 0.0;

                for child in &mut self.children {
                    let child_rect = rect.compute_rect(child.get_size(), self.layout_children);
                    let margin_rect = child_rect.apply_margin(rect, child.get_margin());
                    total_size += margin_rect.w;
                    rects.push((child_rect, margin_rect));
                }

                let mut current_x: f32 = rect.x + (rect.w - total_size) / 2.0;

                self.children.iter_mut().zip(rects.iter_mut()).for_each(
                    |(child, (child_rect, margin_rect))| {
                        margin_rect.x = current_x;
                        current_x += margin_rect.w;

                        let final_rect = UIRect {
                            x: margin_rect.x + rect.w * child.get_margin().left,
                            y: child_rect.y + rect.h * child.get_margin().top,
                            w: child_rect.w,
                            h: child_rect.h,
                        };

                        child.compute_layout(final_rect, window_size, queue);
                    },
                );
            }
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

impl Drawable for UIComponentItem {
    fn render(&self, render_pass: &mut wgpu::RenderPass) {
        self.graphics_data.render(render_pass);

        for child in &self.children {
            child.render(render_pass);
        }
    }
}
