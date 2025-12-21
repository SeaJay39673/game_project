use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{ChunkPos, Tile, TilePos, generate_heightmap};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Chunk {
    pub pos: ChunkPos,
    pub size: usize,
    pub tiles: HashMap<TilePos, Tile>,
}

impl Chunk {
    pub fn new(pos: ChunkPos) -> anyhow::Result<Self> {
        let size: usize = 4;
        let mut tiles: HashMap<TilePos, Tile> = HashMap::new();

        let height_map = generate_heightmap(&pos.to_tile_pos(size), size)?;
        height_map.iter().for_each(|((x, y), z)| {
            let position: TilePos = TilePos::new(*x, *y, *z);
            let tile: Tile = Tile {
                position,
                tile_kind: super::TileKind::Grass,
                tile_type: super::TileType::Block,
            };
            tiles.insert(position, tile);
        });
        Ok(Self { pos, size, tiles })
    }
}
