use std::{mem, sync::Arc};

use anyhow::anyhow;

use wgpu::{
    BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BufferAddress,
    Color, CommandEncoderDescriptor, Device, DeviceDescriptor, Instance, LoadOp, Operations,
    PipelineLayoutDescriptor, Queue, RenderPass, RenderPassColorAttachment, RenderPassDescriptor,
    RenderPipeline, RequestAdapterOptions, SamplerBindingType, ShaderModuleDescriptor,
    ShaderStages, StoreOp, Surface, SurfaceConfiguration, SurfaceError, TextureSampleType,
    TextureViewDescriptor, TextureViewDimension, VertexBufferLayout, VertexStepMode,
    vertex_attr_array,
};
use winit::{dpi::PhysicalSize, window::Window};

use crate::mesh::{InstanceData, VertexData};

pub struct Graphics {
    pub surface: Surface<'static>,
    pub device: Device,
    pub queue: Queue,
    pub config: SurfaceConfiguration,
    pub pipeline: RenderPipeline,
    pub texture_layout: BindGroupLayout,
}

impl Graphics {
    pub async fn new(window: &Arc<Window>) -> anyhow::Result<Self> {
        let PhysicalSize { width, height } = window.inner_size();
        let instance = Instance::default();
        let surface = instance.create_surface(window.clone())?;

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await?;

        let (device, queue) = adapter.request_device(&DeviceDescriptor::default()).await?;

        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Game Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/shader.wgsl").into()),
        });

        let config = if let Some(config) = surface.get_default_config(&adapter, width, height) {
            config
        } else {
            return Err(anyhow!("Could not get surface configuration"));
        };

        surface.configure(&device, &config);

        let texture_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("Texture Bind Group Layout"),
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        count: None,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Texture {
                            sample_type: TextureSampleType::Float { filterable: true },
                            view_dimension: TextureViewDimension::D2,
                            multisampled: false,
                        },
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        count: None,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    },
                ],
            });

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[&texture_bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[
                    VertexBufferLayout {
                        array_stride: mem::size_of::<VertexData>() as BufferAddress,
                        step_mode: VertexStepMode::Vertex,
                        attributes: &vertex_attr_array![
                            0 => Float32x3,
                            1 => Float32x2,
                            2 => Unorm8x4,
                        ],
                    },
                    VertexBufferLayout {
                        array_stride: mem::size_of::<InstanceData>() as BufferAddress,
                        step_mode: VertexStepMode::Instance,
                        attributes: &vertex_attr_array![
                            3 => Float32x4,
                            4 => Float32x4,
                            5 => Float32x4,
                            6 => Float32x4,
                            7 => Unorm8x4,
                        ],
                    },
                ],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        Ok(Self {
            surface,
            device,
            queue,
            config,
            pipeline,
            texture_layout: texture_bind_group_layout,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
    }

    pub fn render(&mut self, renderable: Option<&dyn Renderable>) -> anyhow::Result<()> {
        let frame = match self.surface.get_current_texture() {
            Ok(frame) => frame,
            Err(SurfaceError::Lost) => {
                self.resize(self.config.width, self.config.height);
                return Ok(());
            }
            Err(SurfaceError::OutOfMemory) => return Err(anyhow!("Out of GPU memory!")),
            Err(e) => return Err(anyhow!("Graphics SurfaceError: {e}")),
        };

        let view = frame.texture.create_view(&TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Clear Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(&self.pipeline);
            if let Some(renderable) = renderable {
                renderable.render(&mut render_pass);
            }
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        frame.present();

        Ok(())
    }
}

pub trait Renderable {
    fn render(&self, render_pass: &mut RenderPass);
}
