use cgmath::{Point2, Point3};

use crate::engine::model::{self, ModelVertex};

#[derive(Debug, Clone, Copy)]
pub enum BlockFace {
    Front,
    Back,
    Left,
    Right,
    Top,
    Bottom,
}

impl BlockFace {
    pub fn get_normal(&self) -> Point3<f32> {
        match self {
            BlockFace::Front => Point3::new(0.0, 0.0, 1.0),
            BlockFace::Back => Point3::new(0.0, 0.0, -1.0),
            BlockFace::Left => Point3::new(-1.0, 0.0, 0.0),
            BlockFace::Right => Point3::new(1.0, 0.0, 0.0),
            BlockFace::Top => Point3::new(0.0, 1.0, 0.0),
            BlockFace::Bottom => Point3::new(0.0, -1.0, 0.0),
        }
    }

    pub fn get_position(&self, position: Point3<f32>) -> [Point3<f32>; 4] {
        let x = position.x;
        let y = position.y;
        let z = position.z;

        match self {
            BlockFace::Front => [
                Point3::new(x, y, z + 1.0),
                Point3::new(x + 1.0, y, z + 1.0),
                Point3::new(x + 1.0, y + 1.0, z + 1.0),
                Point3::new(x, y + 1.0, z + 1.0),
            ],
            BlockFace::Back => [
                Point3::new(x + 1.0, y, z),
                Point3::new(x, y, z),
                Point3::new(x, y + 1.0, z),
                Point3::new(x + 1.0, y + 1.0, z),
            ],
            BlockFace::Left => [
                Point3::new(x, y, z),
                Point3::new(x, y, z + 1.0),
                Point3::new(x, y + 1.0, z + 1.0),
                Point3::new(x, y + 1.0, z),
            ],
            BlockFace::Right => [
                Point3::new(x + 1.0, y, z + 1.0),
                Point3::new(x + 1.0, y, z),
                Point3::new(x + 1.0, y + 1.0, z),
                Point3::new(x + 1.0, y + 1.0, z + 1.0),
            ],
            BlockFace::Top => [
                Point3::new(x, y + 1.0, z + 1.0),
                Point3::new(x + 1.0, y + 1.0, z + 1.0),
                Point3::new(x + 1.0, y + 1.0, z),
                Point3::new(x, y + 1.0, z),
            ],
            BlockFace::Bottom => [
                Point3::new(x, y, z),
                Point3::new(x + 1.0, y, z),
                Point3::new(x + 1.0, y, z + 1.0),
                Point3::new(x, y, z + 1.0),
            ],
        }
    }

    pub fn get_tex_coords() -> [Point2<f32>; 4] {
        [
            Point2::new(0.0, 1.0),
            Point2::new(1.0, 1.0),
            Point2::new(1.0, 0.0),
            Point2::new(0.0, 0.0),
        ]
    }
}

pub trait Block {
    #[allow(unused_variables)]
    fn get_texture_index(&self, face: BlockFace) -> u32 {
        0
    }

    fn get_vertices(&self, position: Point3<f32>) -> (Vec<model::ModelVertex>, Vec<u32>) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut vertex_count = 0;

        for face in [
            BlockFace::Front,
            BlockFace::Back,
            BlockFace::Right,
            BlockFace::Left,
            BlockFace::Top,
            BlockFace::Bottom,
        ] {
            let positions = face.get_position(position);
            let tex_coords = BlockFace::get_tex_coords();
            let normal = face.get_normal();
            let tex_index = self.get_texture_index(face);

            for i in 0..4 {
                vertices.push(ModelVertex {
                    position: positions[i].into(),
                    text_coords: tex_coords[i].into(),
                    normal: normal.into(),
                    tex_index,
                });
            }

            let base = vertex_count;
            indices.extend_from_slice(&[base, base + 1, base + 2, base + 2, base + 3, base]);
            vertex_count += 4;
        }

        (vertices, indices)
    }
}
