use crate::engine::{model, object::Context};

pub struct BlockFaceConfig {
    pub front: u32,
    pub back: u32,
    pub top: u32,
    pub bottom: u32,
    pub right: u32,
    pub left: u32,
}

fn create_block_vertices(face_config: &BlockFaceConfig) -> Vec<model::ModelVertex> {
    vec![
        // Front face
        model::ModelVertex {
            position: [-0.5, -0.5, 0.5],
            text_coords: [0.0, 1.0],
            normal: [0.0, 0.0, 1.0],
            tex_index: face_config.front,
        },
        model::ModelVertex {
            position: [0.5, -0.5, 0.5],
            text_coords: [1.0, 1.0],
            normal: [0.0, 0.0, 1.0],
            tex_index: face_config.front,
        },
        model::ModelVertex {
            position: [0.5, 0.5, 0.5],
            text_coords: [1.0, 0.0],
            normal: [0.0, 0.0, 1.0],
            tex_index: face_config.front,
        },
        model::ModelVertex {
            position: [-0.5, 0.5, 0.5],
            text_coords: [0.0, 0.0],
            normal: [0.0, 0.0, 1.0],
            tex_index: face_config.front,
        },
        // Back face
        model::ModelVertex {
            position: [-0.5, -0.5, -0.5],
            text_coords: [1.0, 1.0],
            normal: [0.0, 0.0, -1.0],
            tex_index: face_config.back,
        },
        model::ModelVertex {
            position: [-0.5, 0.5, -0.5],
            text_coords: [1.0, 0.0],
            normal: [0.0, 0.0, -1.0],
            tex_index: face_config.back,
        },
        model::ModelVertex {
            position: [0.5, 0.5, -0.5],
            text_coords: [0.0, 0.0],
            normal: [0.0, 0.0, -1.0],
            tex_index: face_config.back,
        },
        model::ModelVertex {
            position: [0.5, -0.5, -0.5],
            text_coords: [0.0, 1.0],
            normal: [0.0, 0.0, -1.0],
            tex_index: face_config.back,
        },
        // Top face
        model::ModelVertex {
            position: [-0.5, 0.5, -0.5],
            text_coords: [0.0, 1.0],
            normal: [0.0, 1.0, 0.0],
            tex_index: face_config.top,
        },
        model::ModelVertex {
            position: [-0.5, 0.5, 0.5],
            text_coords: [0.0, 0.0],
            normal: [0.0, 1.0, 0.0],
            tex_index: face_config.top,
        },
        model::ModelVertex {
            position: [0.5, 0.5, 0.5],
            text_coords: [1.0, 0.0],
            normal: [0.0, 1.0, 0.0],
            tex_index: face_config.top,
        },
        model::ModelVertex {
            position: [0.5, 0.5, -0.5],
            text_coords: [1.0, 1.0],
            normal: [0.0, 1.0, 0.0],
            tex_index: face_config.top,
        },
        // Bottom face
        model::ModelVertex {
            position: [-0.5, -0.5, -0.5],
            text_coords: [1.0, 1.0],
            normal: [0.0, -1.0, 0.0],
            tex_index: face_config.bottom,
        },
        model::ModelVertex {
            position: [0.5, -0.5, -0.5],
            text_coords: [0.0, 1.0],
            normal: [0.0, -1.0, 0.0],
            tex_index: face_config.bottom,
        },
        model::ModelVertex {
            position: [0.5, -0.5, 0.5],
            text_coords: [0.0, 0.0],
            normal: [0.0, -1.0, 0.0],
            tex_index: face_config.bottom,
        },
        model::ModelVertex {
            position: [-0.5, -0.5, 0.5],
            text_coords: [1.0, 0.0],
            normal: [0.0, -1.0, 0.0],
            tex_index: face_config.bottom,
        },
        // Right face
        model::ModelVertex {
            position: [0.5, -0.5, -0.5],
            text_coords: [1.0, 1.0],
            normal: [1.0, 0.0, 0.0],
            tex_index: face_config.right,
        },
        model::ModelVertex {
            position: [0.5, 0.5, -0.5],
            text_coords: [1.0, 0.0],
            normal: [1.0, 0.0, 0.0],
            tex_index: face_config.right,
        },
        model::ModelVertex {
            position: [0.5, 0.5, 0.5],
            text_coords: [0.0, 0.0],
            normal: [1.0, 0.0, 0.0],
            tex_index: face_config.right,
        },
        model::ModelVertex {
            position: [0.5, -0.5, 0.5],
            text_coords: [0.0, 1.0],
            normal: [1.0, 0.0, 0.0],
            tex_index: face_config.right,
        },
        // Left face
        model::ModelVertex {
            position: [-0.5, -0.5, -0.5],
            text_coords: [0.0, 1.0],
            normal: [-1.0, 0.0, 0.0],
            tex_index: face_config.left,
        },
        model::ModelVertex {
            position: [-0.5, -0.5, 0.5],
            text_coords: [1.0, 1.0],
            normal: [-1.0, 0.0, 0.0],
            tex_index: face_config.left,
        },
        model::ModelVertex {
            position: [-0.5, 0.5, 0.5],
            text_coords: [1.0, 0.0],
            normal: [-1.0, 0.0, 0.0],
            tex_index: face_config.left,
        },
        model::ModelVertex {
            position: [-0.5, 0.5, -0.5],
            text_coords: [0.0, 0.0],
            normal: [-1.0, 0.0, 0.0],
            tex_index: face_config.left,
        },
    ]
}

const BLOCK_INDICES: &[u32] = &[
    0, 1, 2, 2, 3, 0, // Front face
    4, 5, 6, 6, 7, 4, // Back face
    8, 9, 10, 10, 11, 8, // Top face
    12, 13, 14, 14, 15, 12, // Bottom face
    16, 17, 18, 18, 19, 16, // Right face
    20, 21, 22, 22, 23, 20, // Left face
];

impl<'a> Context<'a> {
    pub fn create_block(
        &self,
        texture_paths: &[&str],
        face_config: &BlockFaceConfig,
        name: &str,
    ) -> anyhow::Result<model::Model> {
        let texture_array = self.load_texture_array(texture_paths)?;
        let vertices = create_block_vertices(face_config);

        self.create_model(&vertices, BLOCK_INDICES, texture_array, name)
    }
}
