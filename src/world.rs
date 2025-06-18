use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use cgmath::Point3;
use crossbeam::channel::{Receiver, Sender};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    chunk::{Chunk, ChunkPosition},
    engine::object::{Context, Object},
    world_gen::WorldGenerator,
};

struct GenJob {
    position: ChunkPosition,
}

pub struct World {
    chunks: HashMap<ChunkPosition, Chunk>,
    in_flight: HashSet<ChunkPosition>,
    job_tx: Sender<GenJob>,
    result_rx: Receiver<Chunk>,

    pub render_distance: u32,
    last_player_chunk: Option<ChunkPosition>,
}

impl World {
    pub fn new(render_distance: u32) -> Self {
        let (job_tx, job_rx) = crossbeam::channel::unbounded::<GenJob>();
        let (result_tx, result_rx) = crossbeam::channel::unbounded::<Chunk>();
        let generator = Arc::new(WorldGenerator::new(rand::random()));

        rayon::spawn({
            let generator = Arc::clone(&generator);
            move || {
                while let Ok(first_job) = job_rx.recv() {
                    let mut jobs = vec![first_job];
                    jobs.extend(job_rx.try_iter());

                    jobs.into_par_iter().for_each(|job| {
                        let mut chunk = Chunk::new(job.position);
                        generator.generate_chunk(&mut chunk);

                        result_tx.send(chunk).unwrap();
                    });
                }
            }
        });

        Self {
            chunks: HashMap::new(),
            in_flight: HashSet::new(),
            job_tx,
            result_rx,
            render_distance,
            last_player_chunk: None,
        }
    }

    pub fn get_chunk(&self, position: &ChunkPosition) -> Option<&Chunk> {
        self.chunks.get(position)
    }

    pub fn get_chunk_mut(&mut self, position: &ChunkPosition) -> Option<&mut Chunk> {
        self.chunks.get_mut(position)
    }

    pub fn load_chunk(&mut self, position: ChunkPosition) {
        if self.chunks.contains_key(&position) || self.in_flight.contains(&position) {
            return;
        }

        self.job_tx.send(GenJob { position }).unwrap();
        self.in_flight.insert(position);
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

        if self.last_player_chunk == Some(player_chunk) {
            return;
        }
        self.last_player_chunk = Some(player_chunk);

        let mut should_be_loaded = HashSet::new();
        let rd = self.render_distance as i32;

        for dx in -rd..=rd {
            for dz in -rd..=rd {
                if dx * dx + dz * dz <= rd * rd {
                    should_be_loaded
                        .insert(ChunkPosition::new(player_chunk.x + dx, player_chunk.z + dz));
                }
            }
        }

        for pos in &should_be_loaded {
            if !self.chunks.contains_key(pos) || !self.in_flight.contains(pos) {
                self.load_chunk(*pos);
            }
        }

        let chunks_to_unload: Vec<ChunkPosition> = self
            .chunks
            .keys()
            .filter(|pos| !should_be_loaded.contains(pos))
            .cloned()
            .collect();

        for pos in chunks_to_unload {
            self.unload_chunk(pos, ctx);
        }
    }

    fn flush_generated_chunks(&mut self, ctx: &mut Context) {
        let drained: Vec<Chunk> = self.result_rx.try_iter().collect();

        for mut chunk in drained {
            let data = chunk.build_mesh(self);
            chunk.upload_mesh(data, ctx);

            ctx.spawn_model(chunk.mesh.as_ref().unwrap());

            self.mark_neighbors_for_rebuild(&chunk.position);

            self.in_flight.remove(&chunk.position);
            self.chunks.insert(chunk.position, chunk);
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
                let data = chunk.build_mesh(self);
                chunk.upload_mesh(data, ctx);
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
        self.flush_generated_chunks(ctx);
        self.update_chunks_around_player(ctx.get_camera_position(), ctx);
        self.rebuild_chunk_meshes(ctx);
    }
}
