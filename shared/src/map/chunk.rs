use std::collections::{HashMap};

use serde::{Deserialize, Serialize};

use crate::{ChunkPos, Tile, TilePos, generate_heightmap};

#[derive(Serialize, Deserialize, Clone)]
pub struct Chunk {
    pub size: usize,
    pub tiles: HashMap<TilePos, Tile>,
}

impl Chunk {
    pub fn new(pos: ChunkPos) -> Self {
        let size: usize = 4;
        let mut tiles: HashMap<TilePos, Tile> = HashMap::new();

        if let Ok(height_map) = generate_heightmap(&pos, size) {
            height_map.iter().for_each(|((x, y), z)| {
                let position: TilePos = TilePos::new(*x, *y, *z);
                let tile: Tile = Tile {
                    position,
                    tile_kind: super::TileKind::Grass,
                    tile_type: super::TileType::Block,
                };
                tiles.insert(position, tile);
            });
        } else {
            for y in -(size as i64)..=size as i64 {
                for x in -(size as i64)..=size as i64 {
                    let position: TilePos = TilePos::new(x, y, 0);
                    let tile: Tile = Tile {
                        position,
                        tile_kind: super::TileKind::Grass,
                        tile_type: super::TileType::Block,
                    };
                    tiles.insert(position, tile);
                }
            }
        }

        Self {
            size,
            tiles,
        }
    }
}
