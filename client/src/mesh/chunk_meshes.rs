use std::{collections::BTreeMap, sync::Arc};

use crate::{
    graphics::{Graphics, Renderable, Texture},
    mesh::ChunkMesh,
};

pub struct ChunkMeshes {
    meshes: BTreeMap<(i64, i64), ChunkMesh>,
}

impl ChunkMeshes {
    pub fn new(
        graphics: &Graphics,
        chunks_radius: u8,
        chunk_size: u8,
        scale: f32,
    ) -> anyhow::Result<Self> {
        let mut meshes: BTreeMap<(i64, i64), ChunkMesh> = BTreeMap::new();

        let texture = Arc::new(Texture::from_file(graphics, "src/assets/grass_block.png")?);

        let size_i64 = chunks_radius as i64;

        for x in -(size_i64)..=size_i64 {
            for y in -(size_i64)..=size_i64 {
                meshes.insert(
                    (-x, -y),
                    ChunkMesh::new(
                        &graphics.device,
                        [x, y],
                        chunk_size * 2 + 1,
                        chunk_size,
                        scale,
                        texture.clone(),
                    )?,
                );
            }
        }

        Ok(Self { meshes })
    }
}

impl Renderable for ChunkMeshes {
    fn render(&self, render_pass: &mut wgpu::RenderPass) {
        self.meshes
            .values()
            .for_each(|chunk_mesh| chunk_mesh.render(render_pass));
    }
}
