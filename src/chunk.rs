use cgmath::Point3;

use crate::{
    GrassBlock,
    blocks::{Block, BlockFace, BlockType},
    engine::{
        model::{Model, ModelVertex},
        object::{Context, Object},
    },
};

pub const CHUNK_SIZE: usize = 16;
pub const CHUNK_HEIGHT: usize = 256;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkPosition {
    pub x: i32,
    pub z: i32,
}

impl ChunkPosition {
    pub fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }

    pub fn from_world_pos(world_x: f32, world_z: f32) -> Self {
        Self {
            x: (world_x / CHUNK_SIZE as f32).floor() as i32,
            z: (world_z / CHUNK_SIZE as f32).floor() as i32,
        }
    }
}

pub struct Chunk {
    pub position: ChunkPosition,
    pub blocks: [[[BlockType; CHUNK_SIZE]; CHUNK_HEIGHT]; CHUNK_SIZE],
    pub mesh: Option<Model>,
    pub need_rebuilt: bool,
}

impl Chunk {
    pub fn new(position: ChunkPosition) -> Self {
        Self {
            position,
            blocks: [[[BlockType::Grass(GrassBlock::default()); CHUNK_SIZE]; CHUNK_HEIGHT];
                CHUNK_SIZE],
            mesh: None,
            need_rebuilt: true,
        }
    }

    pub fn get_block(&self, x: usize, y: usize, z: usize) -> Option<&BlockType> {
        if x < CHUNK_SIZE && y < CHUNK_HEIGHT && z < CHUNK_SIZE {
            Some(&self.blocks[x][y][z])
        } else {
            None
        }
    }

    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block: BlockType) {
        if x < CHUNK_SIZE && y < CHUNK_HEIGHT && z < CHUNK_SIZE {
            self.blocks[x][y][z] = block;
            self.need_rebuilt = true;
        }
    }

    pub fn generate_terrain(&mut self) {
        self.blocks =
            [[[BlockType::Grass(GrassBlock::default()); CHUNK_SIZE]; CHUNK_HEIGHT]; CHUNK_SIZE];
        self.need_rebuilt = true;
    }

    pub fn build_mesh(&mut self, ctx: &mut Context) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_HEIGHT {
                for z in 0..CHUNK_SIZE {
                    let block = &self.blocks[x][y][z];

                    let world_pos = Point3::new(
                        (self.position.x * CHUNK_SIZE as i32 + x as i32) as f32,
                        y as f32,
                        (self.position.z * CHUNK_SIZE as i32 + z as i32) as f32,
                    );

                    for face in [
                        BlockFace::Front,
                        BlockFace::Back,
                        BlockFace::Left,
                        BlockFace::Right,
                        BlockFace::Top,
                        BlockFace::Bottom,
                    ] {
                        if self.should_hide_face(x, y, z, face) {
                            continue;
                        }

                        let face_vertices = face.get_vertices(world_pos);
                        let tex_coords = BlockFace::get_tex_coords();
                        let normal = face.get_normal();
                        let tex_index = block.get_texture_index(face);

                        for i in 0..4 {
                            vertices.push(ModelVertex {
                                position: face_vertices[i].into(),
                                text_coords: tex_coords[i].into(),
                                normal: normal.into(),
                                tex_index,
                            });
                        }

                        let base = vertices.len() as u32;
                        indices.extend_from_slice(&[
                            base,
                            base + 1,
                            base + 2,
                            base + 2,
                            base + 3,
                            base,
                        ]);
                    }
                }
            }
        }

        let texture_array = ctx
            .load_texture_array(&["grass_block_top.png", "dirt.png", "grass_block_side.png"])
            .unwrap();

        self.mesh = Some(
            ctx.create_model(
                vertices.as_slice(),
                indices.as_slice(),
                texture_array,
                &format!("Chunk({}-{})", self.position.x, self.position.z),
            )
            .unwrap(),
        );
    }

    fn should_hide_face(&self, x: usize, y: usize, z: usize, face: BlockFace) -> bool {
        let (nx, ny, nz) = match face {
            BlockFace::Front => (x as i32, y as i32, z as i32 + 1),
            BlockFace::Back => (x as i32, y as i32, z as i32 - 1),
            BlockFace::Left => (x as i32 - 1, y as i32, z as i32),
            BlockFace::Right => (x as i32 + 1, y as i32, z as i32),
            BlockFace::Top => (x as i32, y as i32 + 1, z as i32),
            BlockFace::Bottom => (x as i32, y as i32 - 1, z as i32),
        };

        if nx >= 0
            && nx < CHUNK_SIZE as i32
            && ny >= 0
            && ny < CHUNK_HEIGHT as i32
            && nz >= 0
            && nz < CHUNK_SIZE as i32
        {
            let neighbor = &self.blocks[nx as usize][ny as usize][nz as usize];
            !neighbor.is_transparent()
        } else {
            false
        }
    }
}

impl Object for Chunk {
    #[allow(unused_variables)]
    fn update(&mut self, ctx: &mut Context, delta: f32) {
        if self.need_rebuilt {
            self.build_mesh(ctx);
            let _ = ctx.spawn_model(self.mesh.as_ref().unwrap());
            self.need_rebuilt = false;
        }
    }
}
