use wgpu::RenderPass;

pub trait Drawable {
    fn render(&self, render_pass: &mut RenderPass);
}