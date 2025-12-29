use serde::{Deserialize, Serialize};

use crate::TilePos;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq)]
pub enum TileKind {
    Grass,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Tile {
    pub tile_kind: TileKind,
    pub position: TilePos,
}
