use std::collections::HashMap;

use quinn::Chunk;

use crate::ChunkPos;



pub struct ChunkManager {
    chunks: HashMap<ChunkPos, Chunk>
}

impl ChunkManager {
    
}