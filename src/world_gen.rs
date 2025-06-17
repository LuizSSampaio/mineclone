use noise::{NoiseFn, Perlin, Seedable};

use crate::{
    GrassBlock,
    blocks::{AirBlock, BlockType},
    chunk::{CHUNK_HEIGHT, CHUNK_SIZE, Chunk},
};

pub struct WorldGenerator {
    perlin: Perlin,

    amplitude: f32,
    base_height: f32,
    frequency: f32,
}

impl WorldGenerator {
    pub fn new(seed: u32) -> Self {
        Self {
            perlin: Perlin::new(seed),
            amplitude: 32.0,
            base_height: 64.0,
            frequency: 0.03,
        }
    }

    pub fn get_seed(&self) -> u32 {
        self.perlin.seed()
    }

    fn height_at(&self, world_x: i32, world_z: i32) -> usize {
        let n = (self.perlin.get([
            world_x as f64 * self.frequency as f64,
            world_z as f64 * self.frequency as f64,
        ]) + 1.0)
            * 0.5;

        let h = (n * self.amplitude as f64 + self.base_height as f64).round() as i32;

        h.clamp(0, CHUNK_HEIGHT as i32 - 1) as usize
    }

    pub fn generate_chunk(&self, chunk: &mut Chunk) {
        for x in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let world_x = chunk.position.x * CHUNK_SIZE as i32 + x as i32;
                let world_z = chunk.position.z * CHUNK_SIZE as i32 + z as i32;

                let collum_height = self.height_at(world_x, world_z);

                for y in 0..collum_height {
                    chunk.blocks[x][y][z] = BlockType::Grass(GrassBlock::default());
                }
                for y in (collum_height + 1)..CHUNK_HEIGHT {
                    chunk.blocks[x][y][z] = BlockType::Air(AirBlock::default());
                }
            }
        }

        chunk.need_rebuilt = true;
    }
}
