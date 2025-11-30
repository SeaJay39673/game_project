use serde::{Deserialize, Serialize};

use crate::{TilePos};

#[derive(Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq)]
pub enum TileKind {
    Grass,
    Stone,
    Sand,
    Water,
}

#[derive(Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq)]
pub enum TileType {
    Block,
    SlopeL,
    SlopeR,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Tile {
    pub tile_kind: TileKind,
    pub tile_type: TileType,
    pub position: TilePos,
}   