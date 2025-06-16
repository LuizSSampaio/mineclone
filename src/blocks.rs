use cgmath::{Point2, Point3};
use enum_dispatch::enum_dispatch;

use crate::GrassBlock;

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

    pub fn get_vertices(&self, position: Point3<f32>) -> [Point3<f32>; 4] {
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

#[enum_dispatch(Block)]
#[derive(Debug, Clone, Copy)]
pub enum BlockType {
    Air(AirBlock),
    Grass(GrassBlock),
}

#[enum_dispatch]
pub trait Block {
    #[allow(unused_variables)]
    fn get_texture_index(&self, face: BlockFace) -> u32 {
        0
    }

    fn is_transparent(&self) -> bool {
        false
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct AirBlock {}

impl Block for AirBlock {
    fn is_transparent(&self) -> bool {
        true
    }
}
