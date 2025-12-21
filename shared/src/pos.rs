use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PlayerPos {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl PlayerPos {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Clone, Copy)]
pub struct TilePos {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl TilePos {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Clone, Copy)]
pub struct ChunkPos {
    pub x: i64,
    pub y: i64,
}

impl ChunkPos {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn to_tile_pos(&self, size: usize) -> TilePos {
        TilePos { x: self.x * size as i64, y: self.y * size as i64, z: 0 }
    }
}