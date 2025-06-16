use std::collections::HashMap;

use cgmath::Point3;

use crate::{
    chunk::{Chunk, ChunkPosition},
    engine::object::{Context, Object},
};

pub struct World {
    chunks: HashMap<ChunkPosition, Chunk>,
    pub render_distance: u32,
}

impl World {
    pub fn new(render_distance: u32) -> Self {
        Self {
            chunks: HashMap::new(),
            render_distance,
        }
    }

    pub fn get_chunk(&self, position: &ChunkPosition) -> Option<&Chunk> {
        self.chunks.get(position)
    }

    pub fn get_chunk_mut(&mut self, position: &ChunkPosition) -> Option<&mut Chunk> {
        self.chunks.get_mut(position)
    }

    pub fn load_chunk(&mut self, position: ChunkPosition, ctx: &mut Context) {
        self.chunks.entry(position).or_insert_with(|| {
            let mut chunk = Chunk::new(position);
            chunk.generate_terrain();
            chunk.build_mesh(ctx);
            if let Some(mesh) = chunk.mesh.as_ref() {
                ctx.spawn_model(mesh);
            }
            chunk.need_rebuilt = false;
            chunk
        });
    }

    pub fn unload_chunk(&mut self, position: ChunkPosition, ctx: &mut Context) {
        if let Some(chunk) = self.get_chunk(&position) {
            if let Some(mesh) = chunk.mesh.as_ref() {
                ctx.despawn_model(mesh);
            }

            self.chunks.remove(&position);
        }
    }

    pub fn update_chunks_around_player(&mut self, player_pos: Point3<f32>, ctx: &mut Context) {
        let player_chunk = ChunkPosition::from_world_pos(player_pos.x, player_pos.z);

        for x in (player_chunk.x - self.render_distance as i32)
            ..(player_chunk.z + self.render_distance as i32)
        {
            for z in (player_chunk.x - self.render_distance as i32)
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
        for chunk in self.chunks.values_mut() {
            if chunk.need_rebuilt {
                if let Some(old_mesh) = chunk.mesh.as_ref() {
                    ctx.despawn_model(old_mesh);
                }
                chunk.build_mesh(ctx);
                if let Some(new_mesh) = chunk.mesh.as_ref() {
                    ctx.spawn_model(new_mesh);
                }
                chunk.need_rebuilt = false;
            }
        }
    }
}

impl Object for World {
    #![allow(unused_variables)]
    fn update(&mut self, ctx: &mut Context, delta: f32) {
        self.update_chunks_around_player(Point3::new(0.0, 0.0, 0.0), ctx);
        // self.rebuild_chunk_meshes(ctx);
    }
}
