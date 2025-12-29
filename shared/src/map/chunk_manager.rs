use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::{Chunk, ChunkPos, map::chunk, pos};

#[derive(Serialize, Deserialize, Clone)]
pub struct ChunkManager {
    pub chunks: HashMap<ChunkPos, Chunk>,
}

impl ChunkManager {
    pub fn new() -> anyhow::Result<Self> {
        let mut chunks: HashMap<ChunkPos, Chunk> = HashMap::new();

        let pos = (0, 0);
        let size: i64 = 2;

        let chunk_pos = ChunkPos::new(0, 0);

        for x in -(pos.0 + size)..=(pos.0 + size) {
            for y in -(pos.1 + size)..=(pos.1 + size) {
                let pos = ChunkPos::new(x, y);
                let chunk = Chunk::new(pos)?;
                chunks.insert(pos, chunk);
            }
        }

        Ok(Self { chunks })
    }

    pub fn get_chunks_radius(&self, pos: ChunkPos, size: usize) -> Vec<(ChunkPos, Chunk)> {
        let size = size as i64;

        let mut positions: HashSet<ChunkPos> = HashSet::new();

        for x in (pos.x - size)..=(pos.x + size) {
            for y in (pos.y - size)..=(pos.y + size) {
                let pos = ChunkPos::new(x, y);
                positions.insert(pos);
            }
        }

        self.get_chunks(positions)
    }

    pub fn get_chunks(&self, positions: HashSet<ChunkPos>) -> Vec<(ChunkPos, Chunk)> {
        let mut chunks: Vec<(ChunkPos, Chunk)> = vec![];

        for pos in positions {
            if let Some(chunk) = self.chunks.get(&pos) {
                chunks.push((pos, chunk.clone()));
            }
        }

        chunks
    }
}
