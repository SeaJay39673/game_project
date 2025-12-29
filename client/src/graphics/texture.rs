use wgpu::{
    AddressMode, BindGroup, BindGroupDescriptor, BindGroupEntry, BindingResource, Extent3d,
    FilterMode, Origin3d, Sampler, SamplerDescriptor, TexelCopyBufferLayout, TexelCopyTextureInfo,
    TextureAspect, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureView,
    TextureViewDescriptor,
};

use crate::graphics::Graphics;

pub struct Texture {
    pub view: TextureView,
    pub sampler: Sampler,
    pub bind_group: BindGroup,
}

impl Texture {
    pub fn from_file(graphics: &Graphics, path: &str) -> anyhow::Result<Self> {
        let Graphics {
            device,
            queue,
            texture_layout,
            ..
        } = graphics;

        let img = image::open(path)?.to_rgba8();
        let (width, height) = img.dimensions();

        let texture_size = Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&TextureDescriptor {
            label: Some("Texture"),
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
                aspect: TextureAspect::All,
            },
            &img,
            TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * width),
                rows_per_image: Some(height),
            },
            texture_size,
        );

        let texture_view = texture.create_view(&TextureViewDescriptor::default());
        let sampler = device.create_sampler(&SamplerDescriptor {
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            mipmap_filter: FilterMode::Nearest,
            ..Default::default()
        });

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Texture Bind Group"),
            layout: texture_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&texture_view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&sampler),
                },
            ],
        });

        Ok(Self {
            view: texture_view,
            sampler,
            bind_group,
        })
    }

    pub fn from_color(graphics: &Graphics, color: &[u8; 4], width: u32, height: u32) -> Self {
        let Graphics {
            device,
            queue,
            texture_layout,
            ..
        } = graphics;

        let size = Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&TextureDescriptor {
            label: Some("Fallback Texture"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
                aspect: TextureAspect::All,
            },
            color,
            TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4),
                rows_per_image: Some(1),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor::default());

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Texture Bind Group"),
            layout: texture_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&sampler),
                },
            ],
        });

        Self {
            view,
            sampler,
            bind_group,
        }
    }
}
