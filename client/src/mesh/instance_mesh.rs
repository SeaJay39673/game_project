use wgpu::{
    Buffer, BufferUsages, Device, IndexFormat,
    util::{BufferInitDescriptor, DeviceExt},
};

use crate::{
    graphics::Renderable,
    mesh::{InstanceData, VertexData},
};

pub struct InstanceMesh {
    pub vertex_buffer: Buffer,
    pub vertex_count: u32,
    pub index_buffer: Option<Buffer>,
    pub index_count: u32,
    pub instance_buffer: Buffer,
    pub instance_count: u32,
}

impl InstanceMesh {
    pub fn new(
        device: &Device,
        vertices: &[VertexData],
        indices: Option<Vec<u32>>,
        instances: &[InstanceData],
    ) -> Self {
        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Instance Mesh Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: BufferUsages::VERTEX,
        });

        let (index_buffer, index_count) = if let Some(indices) = indices {
            (
                Some(device.create_buffer_init(&BufferInitDescriptor {
                    label: Some("Instance Mesh Index Buffer"),
                    contents: bytemuck::cast_slice(indices.as_ref()),
                    usage: BufferUsages::INDEX,
                })),
                indices.len() as u32,
            )
        } else {
            (None, 0)
        };

        let instance_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Instance Mesh Instance Buffer"),
            contents: bytemuck::cast_slice(instances),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });

        Self {
            vertex_buffer,
            vertex_count: vertices.len() as u32,
            index_buffer,
            index_count,
            instance_buffer,
            instance_count: instances.len() as u32,
        }
    }
}

impl Renderable for InstanceMesh {
    fn render(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));

        if let Some(index_buffer) = &self.index_buffer {
            render_pass.set_index_buffer(index_buffer.slice(..), IndexFormat::Uint32);
            render_pass.draw_indexed(0..self.index_count, 0, 0..self.instance_count);
        } else {
            render_pass.draw(0..self.vertex_count, 0..self.instance_count);
        }
    }
}
