use serde::{Deserialize, Serialize};

use crate::{TilePos};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq)]
pub enum TileKind {
    Grass,
    Stone,
    Sand,
    Water,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq)]
pub enum TileType {
    Block,
    SlopeL,
    SlopeR,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Tile {
    pub tile_kind: TileKind,
    pub tile_type: TileType,
    pub position: TilePos,
}   