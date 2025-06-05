use crate::engine::{app::App, model, object::Context};

const BLOCK_VERTICES: &[model::ModelVertex] = &[
    // Front face
    model::ModelVertex {
        position: [-0.5, -0.5, 0.5],
        text_coords: [0.0, 1.0],
        normal: [0.0, 0.0, 1.0],
    },
    model::ModelVertex {
        position: [0.5, -0.5, 0.5],
        text_coords: [1.0, 1.0],
        normal: [0.0, 0.0, 1.0],
    },
    model::ModelVertex {
        position: [0.5, 0.5, 0.5],
        text_coords: [1.0, 0.0],
        normal: [0.0, 0.0, 1.0],
    },
    model::ModelVertex {
        position: [-0.5, 0.5, 0.5],
        text_coords: [0.0, 0.0],
        normal: [0.0, 0.0, 1.0],
    },
    // Back face
    model::ModelVertex {
        position: [-0.5, -0.5, -0.5],
        text_coords: [1.0, 1.0],
        normal: [0.0, 0.0, -1.0],
    },
    model::ModelVertex {
        position: [-0.5, 0.5, -0.5],
        text_coords: [1.0, 0.0],
        normal: [0.0, 0.0, -1.0],
    },
    model::ModelVertex {
        position: [0.5, 0.5, -0.5],
        text_coords: [0.0, 0.0],
        normal: [0.0, 0.0, -1.0],
    },
    model::ModelVertex {
        position: [0.5, -0.5, -0.5],
        text_coords: [0.0, 1.0],
        normal: [0.0, 0.0, -1.0],
    },
    // Top face
    model::ModelVertex {
        position: [-0.5, 0.5, -0.5],
        text_coords: [0.0, 1.0],
        normal: [0.0, 1.0, 0.0],
    },
    model::ModelVertex {
        position: [-0.5, 0.5, 0.5],
        text_coords: [0.0, 0.0],
        normal: [0.0, 1.0, 0.0],
    },
    model::ModelVertex {
        position: [0.5, 0.5, 0.5],
        text_coords: [1.0, 0.0],
        normal: [0.0, 1.0, 0.0],
    },
    model::ModelVertex {
        position: [0.5, 0.5, -0.5],
        text_coords: [1.0, 1.0],
        normal: [0.0, 1.0, 0.0],
    },
    // Bottom face
    model::ModelVertex {
        position: [-0.5, -0.5, -0.5],
        text_coords: [1.0, 1.0],
        normal: [0.0, -1.0, 0.0],
    },
    model::ModelVertex {
        position: [0.5, -0.5, -0.5],
        text_coords: [0.0, 1.0],
        normal: [0.0, -1.0, 0.0],
    },
    model::ModelVertex {
        position: [0.5, -0.5, 0.5],
        text_coords: [0.0, 0.0],
        normal: [0.0, -1.0, 0.0],
    },
    model::ModelVertex {
        position: [-0.5, -0.5, 0.5],
        text_coords: [1.0, 0.0],
        normal: [0.0, -1.0, 0.0],
    },
    // Right face
    model::ModelVertex {
        position: [0.5, -0.5, -0.5],
        text_coords: [1.0, 1.0],
        normal: [1.0, 0.0, 0.0],
    },
    model::ModelVertex {
        position: [0.5, 0.5, -0.5],
        text_coords: [1.0, 0.0],
        normal: [1.0, 0.0, 0.0],
    },
    model::ModelVertex {
        position: [0.5, 0.5, 0.5],
        text_coords: [0.0, 0.0],
        normal: [1.0, 0.0, 0.0],
    },
    model::ModelVertex {
        position: [0.5, -0.5, 0.5],
        text_coords: [0.0, 1.0],
        normal: [1.0, 0.0, 0.0],
    },
    // Left face
    model::ModelVertex {
        position: [-0.5, -0.5, -0.5],
        text_coords: [0.0, 1.0],
        normal: [-1.0, 0.0, 0.0],
    },
    model::ModelVertex {
        position: [-0.5, -0.5, 0.5],
        text_coords: [1.0, 1.0],
        normal: [-1.0, 0.0, 0.0],
    },
    model::ModelVertex {
        position: [-0.5, 0.5, 0.5],
        text_coords: [1.0, 0.0],
        normal: [-1.0, 0.0, 0.0],
    },
    model::ModelVertex {
        position: [-0.5, 0.5, -0.5],
        text_coords: [0.0, 0.0],
        normal: [-1.0, 0.0, 0.0],
    },
];

const BLOCK_INDICES: &[u32] = &[
    0, 1, 2, 2, 3, 0, // Front face
    4, 5, 6, 6, 7, 4, // Back face
    8, 9, 10, 10, 11, 8, // Top face
    12, 13, 14, 14, 15, 12, // Bottom face
    16, 17, 18, 18, 19, 16, // Right face
    20, 21, 22, 22, 23, 20, // Left face
];

impl<'a> Context<'a> {
    pub fn create_block(&self, texture_path: &str, name: &str) -> anyhow::Result<model::Model> {
        let texture = self.load_texture(texture_path)?;

        self.create_model(BLOCK_VERTICES, BLOCK_INDICES, texture, name)
    }
}
