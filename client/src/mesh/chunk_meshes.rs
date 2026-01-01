use std::{collections::BTreeMap, sync::Arc};

use shared::{Chunk, ChunkPos};
use wgpu::Device;

use crate::{
    graphics::{Graphics, Renderable, Texture},
    mesh::ChunkMesh,
};

pub struct ChunkMeshes {
    meshes: BTreeMap<(i64, i64), ChunkMesh>,
    texture: Arc<Texture>,
}

impl ChunkMeshes {
    pub fn new(graphics: &Graphics) -> anyhow::Result<Self> {
        let texture: Arc<Texture> =
            Arc::new(Texture::from_file(graphics, "src/assets/grass_block.png")?);
        Ok(Self {
            meshes: BTreeMap::new(),
            texture,
        })
    }
    pub fn insert(&mut self, device: &Device, chunk_pos: ChunkPos, chunk: Chunk) {
        self.meshes.insert(
            (-chunk_pos.x, -chunk_pos.y),
            ChunkMesh::new(device, chunk, self.texture.clone(), 0.1),
        );
    }
}

impl Renderable for ChunkMeshes {
    fn render(&self, render_pass: &mut wgpu::RenderPass) {
        self.meshes
            .values()
            .for_each(|chunk_mesh| chunk_mesh.render(render_pass));
    }
}
