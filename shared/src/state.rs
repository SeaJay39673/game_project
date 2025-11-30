use std::sync::LazyLock;

use tokio::sync::RwLock;

use crate::{Chunk, ChunkPos};

pub static CHUNK: LazyLock<RwLock<Chunk>> = LazyLock::new(|| RwLock::new(Chunk::new(ChunkPos::new(0, 0))));
