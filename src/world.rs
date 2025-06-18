use std::{collections::HashMap, sync::Arc};

use cgmath::Point3;

use crate::{
    chunk::{Chunk, ChunkPosition},
    engine::object::{Context, Object},
    world_gen::WorldGenerator,
};

pub struct World {
    chunks: HashMap<ChunkPosition, Chunk>,
    pub render_distance: u32,

    generator: Arc<WorldGenerator>,
}

impl World {
    pub fn new(render_distance: u32) -> Self {
        let seed = rand::random();

        Self {
            chunks: HashMap::new(),
            render_distance,
            generator: Arc::new(WorldGenerator::new(seed)),
        }
    }

    pub fn get_chunk(&self, position: &ChunkPosition) -> Option<&Chunk> {
        self.chunks.get(position)
    }

    pub fn get_chunk_mut(&mut self, position: &ChunkPosition) -> Option<&mut Chunk> {
        self.chunks.get_mut(position)
    }

    pub fn load_chunk(&mut self, position: ChunkPosition, ctx: &mut Context) {
        if self.chunks.contains_key(&position) {
            return;
        }

        let mut chunk = Chunk::new(position);
        self.generator.generate_chunk(&mut chunk);
        chunk.build_mesh(self, ctx);
        if let Some(mesh) = chunk.mesh.as_ref() {
            ctx.spawn_model(mesh);
        }
        chunk.need_rebuilt = false;

        self.chunks.insert(position, chunk);
        self.mark_neighbors_for_rebuild(&position);
    }

    pub fn unload_chunk(&mut self, position: ChunkPosition, ctx: &mut Context) {
        if let Some(chunk) = self.get_chunk(&position) {
            if let Some(mesh) = chunk.mesh.as_ref() {
                ctx.despawn_model(mesh);
            }

            self.chunks.remove(&position);
            self.mark_neighbors_for_rebuild(&position);
        }
    }

    fn mark_neighbors_for_rebuild(&mut self, position: &ChunkPosition) {
        let neighbors = [
            ChunkPosition::new(position.x - 1, position.z),
            ChunkPosition::new(position.x + 1, position.z),
            ChunkPosition::new(position.x, position.z - 1),
            ChunkPosition::new(position.x, position.z + 1),
        ];

        for neighbor_pos in &neighbors {
            if let Some(neighbor_chunk) = self.chunks.get_mut(neighbor_pos) {
                neighbor_chunk.need_rebuilt = true;
            }
        }
    }

    fn update_chunks_around_player(&mut self, player_pos: Point3<f32>, ctx: &mut Context) {
        let player_chunk = ChunkPosition::from_world_pos(player_pos.x, player_pos.z);

        for x in (player_chunk.x - self.render_distance as i32)
            ..(player_chunk.x + self.render_distance as i32)
        {
            for z in (player_chunk.z - self.render_distance as i32)
                ..(player_chunk.z + self.render_distance as i32)
            {
                self.load_chunk(ChunkPosition::new(x, z), ctx);
            }
        }

        let chunks_to_unload: Vec<ChunkPosition> = self
            .chunks
            .keys()
            .filter(|pos| {
                let dx = (pos.x - player_chunk.x).abs();
                let dz = (pos.z - player_chunk.z).abs();
                dx > self.render_distance as i32 || dz > self.render_distance as i32
            })
            .cloned()
            .collect();

        for pos in chunks_to_unload {
            self.unload_chunk(pos, ctx);
        }
    }

    pub fn rebuild_chunk_meshes(&mut self, ctx: &mut Context) {
        let chunks_to_rebuild: Vec<ChunkPosition> = self
            .chunks
            .iter()
            .filter(|(_, chunk)| chunk.need_rebuilt)
            .map(|(pos, _)| *pos)
            .collect();

        for pos in chunks_to_rebuild {
            if let Some(chunk) = self.chunks.get_mut(&pos) {
                if let Some(old_mesh) = chunk.mesh.as_ref() {
                    ctx.despawn_model(old_mesh);
                }
            }

            if let Some(mut chunk) = self.chunks.remove(&pos) {
                chunk.build_mesh(self, ctx);
                if let Some(new_mesh) = chunk.mesh.as_ref() {
                    ctx.spawn_model(new_mesh);
                }
                chunk.need_rebuilt = false;

                self.chunks.insert(pos, chunk);
            }
        }
    }
}

impl Object for World {
    #![allow(unused_variables)]
    fn update(&mut self, ctx: &mut Context, delta: f32) {
        self.update_chunks_around_player(ctx.get_camera_position(), ctx);
        self.rebuild_chunk_meshes(ctx);
    }
}
