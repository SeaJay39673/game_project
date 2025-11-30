use glam::{Mat4, Vec3};
use wgpu::{
    BindGroup, BindGroupLayout, Buffer, Queue,
    util::{BufferInitDescriptor, DeviceExt},
    Device, RenderPass,
};
use crate::engine::{Drawable, TextureAssetData, VertexFloat32};

pub struct GraphicsData {
    pub world_position: [f32; 3],
    pub scale: f32,

    pub width: f32,
    pub height: f32,

    rect_x: f32,
    rect_y: f32,

    vertex_buffer: Buffer,
    index_buffer: Buffer,
    transform: Mat4,
    transform_buffer: Buffer,
    bind_group: BindGroup,
}

impl GraphicsData {
    pub fn new(
        device: &Device,
        layout: &BindGroupLayout,
        world_position: [f32; 3],
        texture: TextureAssetData,
        scale: f32,
    ) -> Self {
        let half_size = scale / 2.0;
        let indices: &[u16] = &[0, 1, 2, 0, 2, 3];

        let vertices = [
            VertexFloat32 { position: [-half_size, -half_size], uv: texture.uv[0] },
            VertexFloat32 { position: [half_size, -half_size], uv: texture.uv[1] },
            VertexFloat32 { position: [half_size, half_size], uv: texture.uv[2] },
            VertexFloat32 { position: [-half_size, half_size], uv: texture.uv[3] },
        ];

        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let transform = Mat4::IDENTITY;

        let transform_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Transform Buffer"),
            contents: bytemuck::cast_slice(transform.as_ref()),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Bind Group"),
            layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: transform_buffer.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(&texture.texture.view) },
                wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::Sampler(&texture.texture.sampler) },
            ],
        });

        Self {
            world_position,
            scale,
            width: scale,
            height: scale,
            rect_x: world_position[0] - scale * 0.5,
            rect_y: world_position[1] - scale * 0.5,
            vertex_buffer,
            index_buffer,
            transform,
            transform_buffer,
            bind_group,
        }
    }

    pub fn set_rect(&mut self, queue: &Queue, x: f32, y: f32, w: f32, h: f32, window_size: (f32, f32)) {
        self.width = w;
        self.height = h;
        self.rect_x = x;
        self.rect_y = y;

        let cx = x + w * 0.5;
        let cy = y + h * 0.5;

        let ndc_x = (cx / window_size.0) * 2.0 - 1.0;
        let ndc_y = 1.0 - (cy / window_size.1) * 2.0;

        self.world_position = [ndc_x, ndc_y, 0.0];

        self.transform =
            Mat4::from_translation(Vec3::new(ndc_x, ndc_y, 0.0)) *
            Mat4::from_scale(Vec3::new(w / window_size.0, h / window_size.1, 1.0));

        queue.write_buffer(
            &self.transform_buffer,
            0,
            bytemuck::cast_slice(self.transform.as_ref()),
        );
    }

    pub fn set_position(&mut self, queue: &Queue, x: f32, y: f32, window_size: (f32, f32)) {
        self.set_rect(queue, x, y, self.width, self.height, window_size);
    }

    pub fn set_size(&mut self, queue: &Queue, w: f32, h: f32, window_size: (f32, f32)) {
        self.set_rect(queue, self.rect_x, self.rect_y, w, h, window_size);
    }

    pub fn translate(&mut self, queue: &Queue, delta: Vec3, window_size: (f32,f32)) {
        self.rect_x += delta.x;
        self.rect_y += delta.y;
        self.set_rect(queue, self.rect_x, self.rect_y, self.width, self.height, window_size);
    }
}

impl Drawable for GraphicsData {
    fn render(&self, render_pass: &mut RenderPass) {
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..6, 0, 0..1);
    }
}
