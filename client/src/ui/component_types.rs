use std::sync::atomic::{AtomicU32, Ordering};

use wgpu::Queue;

use crate::{engine::Drawable, ui::UIEvent};

static NEXT_ID: AtomicU32 = AtomicU32::new(1);

#[derive(Clone)]
pub enum UIComponentID {
    Num(u32),
    Name(String),
}

impl UIComponentID {
    pub fn new_id() -> UIComponentID {
        UIComponentID::Num(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
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
pub struct UIMargin {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl UIMargin {
    pub const ZERO: Self = Self {
        left: 0.0,
        right: 0.0,
        top: 0.0,
        bottom: 0.0,
    };
    pub fn new(left: f32, right: f32, top: f32, bottom: f32) -> Self {
        Self {
            left,
            right,
            top,
            bottom,
        }
    }
}

#[derive(Clone, Copy)]
pub struct UIRect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl UIRect {
    pub fn new(position: (f32, f32), size: (f32, f32)) -> Self {
        Self {
            x: position.0,
            y: position.1,
            w: size.0,
            h: size.1,
        }
    }

    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.w && y >= self.y && y <= self.y + self.h
    }

    pub fn compute_rect(&self, size: UISize, layout: UILayout) -> UIRect {
        let (w, h) = match size {
            UISize::PercentParent(px, py) => (self.w * px, self.h * py),
        };
        let (x, y) = match layout {
            UILayout::TL => (self.x, self.y),
            UILayout::TC => (self.x + (self.w - w) * 0.5, self.y),
            UILayout::TR => (self.x + self.w - w, self.y),
            UILayout::ML => (self.x, self.y + (self.h - h) * 0.5),
            UILayout::MC => (self.x + (self.w - w) * 0.5, self.y + (self.h - h) * 0.5),
            UILayout::MR => (self.x + self.w - w, self.y + (self.h - h) * 0.5),
            UILayout::BL => (self.x, self.y + self.h - h),
            UILayout::BC => (self.x + (self.w - w) * 0.5, self.y + self.h - h),
            UILayout::BR => (self.x + self.w - w, self.y + self.h - h),
        };
        UIRect { x, y, w, h }
    }

    pub fn apply_margin(&self, parent_rect: UIRect, margin: UIMargin) -> UIRect {
        UIRect {
            x: self.x - parent_rect.w * margin.left,
            y: self.y - parent_rect.h * margin.top,
            w: self.w + parent_rect.w * (margin.left + margin.right),
            h: self.h + parent_rect.h * (margin.top + margin.bottom),
        }
    }
}

#[derive(Clone)]
pub enum UIComponentEvent {
    Clicked(UIComponentID),
}

pub trait UIComponent: Drawable {
    fn get_id(&self) -> UIComponentID;
    fn get_size(&self) -> UISize;
    fn get_margin(&self) -> UIMargin;
    fn compute_layout(&mut self, rect: UIRect, window_size: (f32, f32), queue: &Queue);
    fn layout_children(&mut self, rect: UIRect, window_size: (f32, f32), queue: &Queue);
    fn handle_event(&mut self, event: &UIEvent, queue: &Queue) -> Vec<UIComponentEvent>;
}
