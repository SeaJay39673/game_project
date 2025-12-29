use crate::mesh::{MeshData, VertexData};

pub struct TileMesh;

impl TileMesh {
    pub fn to_mesh_data() -> MeshData {
        let vertices: Vec<VertexData> = vec![
            VertexData {
                position: [-1.0, -1.0, 0.0],
                uv: [0.0, 1.0],
                color: [255, 255, 255, 255],
            },
            VertexData {
                position: [-1.0, 1.0, 0.0],
                uv: [0.0, 0.0],
                color: [255, 255, 255, 255],
            },
            VertexData {
                position: [1.0, 1.0, 0.0],
                uv: [1.0, 0.0],
                color: [255, 255, 255, 255],
            },
            VertexData {
                position: [1.0, -1.0, 0.0],
                uv: [1.0, 1.0],
                color: [255, 255, 255, 255],
            },
        ];

        let indices: Vec<u32> = vec![0, 1, 2, 2, 3, 0];

        MeshData {
            vertices: vertices,
            indices: Some(indices),
        }
    }
}
