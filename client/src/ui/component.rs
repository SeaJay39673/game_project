use std::sync::atomic::{AtomicU32, Ordering};

use wgpu::{BindGroupLayout, Device, Queue};

use crate::{
    engine::{Drawable, GraphicsData, TextureManager, TextureType},
    ui::UIEvent,
};

static NEXT_ID: AtomicU32 = AtomicU32::new(1);

pub fn new_id() -> UIComponentID {
    UIComponentID::Num(NEXT_ID.fetch_add(1, Ordering::Relaxed))
}

#[derive(Clone)]
pub enum UIComponentID {
    Num(u32),
    Name(String),
}

#[derive(Clone, Copy)]
pub enum UISize {
    PercentParent(f32, f32),
}

#[derive(Clone, Copy)]
pub enum UILayout {
    TC,
    TL,
    TR,
    MC,
    ML,
    MR,
    BC,
    BL,
    BR,
}

#[derive(Clone, Copy)]
pub enum UIAlign {
    Overlay,
    Vertical,
    Horizontal,
}

#[derive(Clone, Copy)]
pub struct UIRect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl UIRect {
    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.w && y >= self.y && y <= self.y + self.h
    }
}

#[derive(Clone)]
pub enum UIComponentEvent {
    Clicked(UIComponentID),
}

pub trait UIComponent: Drawable {
    fn get_size(&self) -> UISize;
    fn get_layout(&self) -> UILayout;
    fn compute_layout(
        &mut self,
        parent_rect: UIRect,
        window_size: (f32, f32),
        queue: &Queue,
    ) -> UIRect;
    fn layout_children(&mut self, parent_rect: UIRect, window_size: (f32, f32), queue: &Queue);
    fn handle_event(&mut self, event: UIEvent, queue: &Queue) -> Vec<UIComponentEvent>;
}

pub struct UIComponentItem {
    graphics_data: GraphicsData,
    id: UIComponentID,
    size: UISize,
    layout: UILayout,
    align: UIAlign,
    children: Vec<Box<dyn UIComponent + 'static>>,
}

pub struct UIComponentBuilder {
    id: Option<UIComponentID>,
    background: Option<TextureType>,
    size: UISize,
    layout: UILayout,
    align: UIAlign,
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
    pub fn layout(mut self, layout: UILayout) -> Self {
        self.layout = layout;
        self
    }
    pub fn align(mut self, align: UIAlign) -> Self {
        self.align = align;
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
    ) -> anyhow::Result<UIComponentItem> {
        let id = match self.id {
            Some(id) => id,
            None => new_id(),
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
            GraphicsData::new(device, layout, [0.0, 0.0, 0.0], background, 2.0);

        Ok(UIComponentItem {
            id,
            graphics_data,
            size: self.size,
            layout: self.layout,
            align: self.align,
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
            layout: UILayout::MC,
            align: UIAlign::Overlay,
            children: vec![],
        }
    }
}

pub fn compute_rect(parent_rect: UIRect, size: UISize, layout: UILayout) -> UIRect {
    let (w, h) = match size {
        UISize::PercentParent(px, py) => (parent_rect.w * px, parent_rect.h * py),
    };
    let (x, y) = match layout {
        UILayout::TL => (parent_rect.x, parent_rect.y),
        UILayout::TC => (parent_rect.x + (parent_rect.w - w) * 0.5, parent_rect.y),
        UILayout::TR => (parent_rect.x + parent_rect.w - w, parent_rect.y),
        UILayout::ML => (parent_rect.x, parent_rect.y + (parent_rect.h - h) * 0.5),
        UILayout::MC => (
            parent_rect.x + (parent_rect.w - w) * 0.5,
            parent_rect.y + (parent_rect.h - h) * 0.5,
        ),
        UILayout::MR => (
            parent_rect.x + parent_rect.w - w,
            parent_rect.y + (parent_rect.h - h) * 0.5,
        ),
        UILayout::BL => (parent_rect.x, parent_rect.y + parent_rect.h - h),
        UILayout::BC => (
            parent_rect.x + (parent_rect.w - w) * 0.5,
            parent_rect.y + parent_rect.h - h,
        ),
        UILayout::BR => (
            parent_rect.x + parent_rect.w - w,
            parent_rect.y + parent_rect.h - h,
        ),
    };
    UIRect { x, y, w, h }
}

impl UIComponent for UIComponentItem {
    fn get_size(&self) -> UISize {
        self.size
    }
    fn get_layout(&self) -> UILayout {
        self.layout
    }
    fn compute_layout(
        &mut self,
        parent_rect: UIRect,
        window_size: (f32, f32),
        queue: &Queue,
    ) -> UIRect {
        let rect = compute_rect(parent_rect, self.size, self.layout);
        let UIRect { x, y, w, h } = rect;
        self.graphics_data.set_rect(queue, x, y, w, h, window_size);
        self.layout_children(rect, window_size, queue);
        rect
    }

    fn layout_children(&mut self, parent_rect: UIRect, window_size: (f32, f32), queue: &Queue) {
        match self.align {
            UIAlign::Overlay => {
                for child in &mut self.children {
                    child.compute_layout(parent_rect, window_size, queue);
                }
            }
            UIAlign::Vertical => {
                if self.children.is_empty() {
                    return;
                }

                let mut cy = parent_rect.y;

                for child in &mut self.children {
                    let mut child_rect = compute_rect(parent_rect, child.get_size(), child.get_layout());
                    
                    child_rect.y = cy;

                    child.compute_layout(child_rect, window_size, queue);

                    cy += child_rect.h / 2.0;
                }
            }
            UIAlign::Horizontal => {
                if self.children.is_empty() {
                    return;
                }
                let count = self.children.len() as f32;
                let child_w = parent_rect.w / count;
                let mut cx = parent_rect.x;
                for child in &mut self.children {
                    child.compute_layout(
                        UIRect {
                            x: cx,
                            y: parent_rect.y,
                            w: child_w,
                            h: parent_rect.h,
                        },
                        window_size,
                        queue,
                    );
                    cx += child_w;
                }
            }
        }
    }

    fn handle_event(&mut self, event: UIEvent, queue: &Queue) -> Vec<UIComponentEvent> {
        let mut events: Vec<UIComponentEvent> = vec![];
        for child in &mut self.children {
            events.extend_from_slice(&child.handle_event(event.clone(), queue));
        }

        events
    }
}

impl Drawable for UIComponentItem {
    fn render(&self, render_pass: &mut wgpu::RenderPass) {
        self.graphics_data.render(render_pass);
        self.children
            .iter()
            .for_each(|child| child.render(render_pass));
    }
}
