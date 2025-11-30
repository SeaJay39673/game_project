use std::sync::Arc;

use crate::engine::TextureInfo;

pub struct ColorAssetInfo {
    pub color: [u8; 4],
    pub uv: [[f32; 2]; 4],
}

impl ColorAssetInfo {
    pub fn new(color: [u8; 4]) -> Self {
        Self {
            color,
            uv: [
                map_uv(&[0.0, 1.0], 1, 1, 0, 0),
                map_uv(&[1.0, 1.0], 1, 1, 0, 0),
                map_uv(&[1.0, 0.0], 1, 1, 0, 0),
                map_uv(&[0.0, 0.0], 1, 1, 0, 0),
            ],
        }
    }
}

pub struct AssetInfo {
    pub texture_info: Arc<TextureInfo>,
    pub uv: [[f32; 2]; 4],
}

pub enum AssetInfoType {
    Color(ColorAssetInfo),
    Asset(AssetInfo),
}

fn map_uv(coords: &[f32; 2], x_count: u32, y_count: u32, x: u32, y: u32) -> [f32; 2] {
    let x_map: f32 = (coords[0] / x_count as f32) + (x as f32 / x_count as f32);
    let y_map: f32 = (coords[1] / y_count as f32) + (y as f32 / y_count as f32);
    [x_map, y_map]
}

impl AssetInfo {
    pub fn new(texture_info: Arc<TextureInfo>, index: [u32; 2]) -> Self {
        let TextureInfo {
            x_count, y_count, ..
        } = *texture_info;
        Self {
            texture_info,
            uv: [
                map_uv(&[0.0, 1.0], x_count, y_count, index[0], index[1]),
                map_uv(&[1.0, 1.0], x_count, y_count, index[0], index[1]),
                map_uv(&[1.0, 0.0], x_count, y_count, index[0], index[1]),
                map_uv(&[0.0, 0.0], x_count, y_count, index[0], index[1]),
            ],
        }
    }

    pub fn update(&mut self, texture_info: Arc<TextureInfo>, index: [u32; 2]) {
        self.texture_info = texture_info;
        let TextureInfo {
            x_count, y_count, ..
        } = *self.texture_info;
        self.uv = [
            map_uv(&[0.0, 1.0], x_count, y_count, index[0], index[1]),
            map_uv(&[1.0, 1.0], x_count, y_count, index[0], index[1]),
            map_uv(&[1.0, 0.0], x_count, y_count, index[0], index[1]),
            map_uv(&[0.0, 0.0], x_count, y_count, index[0], index[1]),
        ];
    }
}
