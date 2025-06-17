use cgmath::Point3;

use crate::{
    blocks::{AirBlock, Block, BlockFace, BlockType},
    engine::{
        model::{Model, ModelVertex},
        object::Context,
    },
    world::World,
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
            blocks: [[[BlockType::Air(AirBlock::default()); CHUNK_SIZE]; CHUNK_HEIGHT]; CHUNK_SIZE],
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

    pub fn build_mesh(&mut self, world: &World, ctx: &mut Context) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_HEIGHT {
                for z in 0..CHUNK_SIZE {
                    let block = &self.blocks[x][y][z];

                    if matches!(block, BlockType::Air(_)) {
                        continue;
                    }

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
                        if self.should_hide_face(x, y, z, face, world) {
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

    fn should_hide_face(
        &self,
        x: usize,
        y: usize,
        z: usize,
        face: BlockFace,
        world: &World,
    ) -> bool {
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
            self.check_neighbor_chunk(nx, ny, nz, world)
        }
    }

    fn check_neighbor_chunk(&self, x: i32, y: i32, z: i32, world: &World) -> bool {
        if y < 0 || y >= CHUNK_HEIGHT as i32 {
            return y >= CHUNK_HEIGHT as i32;
        }

        let mut chunk_x = self.position.x;
        let mut chunk_z = self.position.z;
        let mut local_x = x;
        let mut local_z = z;

        if x < 0 {
            chunk_x -= 1;
            local_x = CHUNK_SIZE as i32 - 1;
        } else if x >= CHUNK_SIZE as i32 {
            chunk_x += 1;
            local_x = 0;
        }

        if z < 0 {
            chunk_z -= 1;
            local_z = CHUNK_SIZE as i32 - 1;
        } else if z >= CHUNK_SIZE as i32 {
            chunk_z += 1;
            local_z = 0;
        }

        let neighbor_pos = ChunkPosition::new(chunk_x, chunk_z);
        if let Some(neighbor_chunk) = world.get_chunk(&neighbor_pos) {
            let neighbor_block =
                &neighbor_chunk.blocks[local_x as usize][y as usize][local_z as usize];
            !neighbor_block.is_transparent()
        } else {
            false
        }
    }
}
