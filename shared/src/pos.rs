use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PlayerPos {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Serialize, Deserialize, Hash, PartialEq, Eq, Clone, Copy)]
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

#[derive(Serialize, Deserialize)]
pub struct ChunkPos {
    pub x: i64,
    pub y: i64,
}

impl ChunkPos {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}