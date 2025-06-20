mod blocks;
mod chunk;
mod engine;
mod world;
mod world_gen;

use std::time::Instant;

use anyhow::Ok;
use blocks::{Block, BlockFace};
use cgmath::{Deg, InnerSpace, Vector3};
use engine::{
    app::App,
    camera::Camera,
    object::{Context, Object},
};
use winit::keyboard::KeyCode;
use world::World;

fn main() -> anyhow::Result<()> {
    App::default()
        .add_object(World::new(12))
        .add_object(Camera::new((5.0, 100.0, 25.0), Deg(-90.0), Deg(-20.0)))
        .add_object(FPSCounter::default())
        .run()?;
    Ok(())
}

#[derive(Default, Debug, Clone, Copy)]
struct GrassBlock {}

impl Block for GrassBlock {
    fn get_texture_index(&self, face: blocks::BlockFace) -> u32 {
        match face {
            BlockFace::Top => 0,
            BlockFace::Bottom => 1,
            _ => 2,
        }
    }
}

impl Object for Camera {
    fn update(&mut self, ctx: &mut Context, delta: f32) {
        let speed = 5.0;
        let sensitivity = 0.002;

        let forward_dir = ctx.input.is_key_pressed(KeyCode::KeyW) as i8
            - ctx.input.is_key_pressed(KeyCode::KeyS) as i8;
        let right_dir = ctx.input.is_key_pressed(KeyCode::KeyD) as i8
            - ctx.input.is_key_pressed(KeyCode::KeyA) as i8;
        let up_dir = ctx.input.is_key_pressed(KeyCode::Space) as i8
            - ctx.input.is_key_pressed(KeyCode::ShiftLeft) as i8;

        let (yaw_sin, yaw_cos) = self.yaw.0.sin_cos();
        let forward = Vector3::new(yaw_cos, 0.0, yaw_sin).normalize();
        let right = Vector3::new(-yaw_sin, 0.0, yaw_cos).normalize();
        self.position += forward * (forward_dir as f32) * speed * delta;
        self.position += right * (right_dir as f32) * speed * delta;
        self.position += Vector3::unit_y() * up_dir as f32 * speed * delta;

        let (dx, dy) = ctx.input.mouse_delta;
        self.yaw += cgmath::Rad(dx as f32 * sensitivity);
        self.pitch += cgmath::Rad(-dy as f32 * sensitivity);

        let max_pitch = cgmath::Rad(std::f32::consts::FRAC_PI_2 - 0.01);
        if self.pitch > max_pitch {
            self.pitch = max_pitch;
        } else if self.pitch < -max_pitch {
            self.pitch = -max_pitch;
        }

        ctx.update_camera(self);
        ctx.input.reset_mouse_delta();
    }
}

struct FPSCounter {
    last_update: Instant,
    frame_cout: u32,
}

impl Default for FPSCounter {
    fn default() -> Self {
        Self {
            last_update: Instant::now(),
            frame_cout: 0,
        }
    }
}

impl Object for FPSCounter {
    #[allow(unused_variables)]
    fn update(&mut self, ctx: &mut Context, delta: f32) {
        self.frame_cout += 1;
        let now = Instant::now();
        if now.duration_since(self.last_update).as_secs_f32() >= 1.0 {
            println!(
                "FPS: {:.1}",
                self.frame_cout as f32 / now.duration_since(self.last_update).as_secs_f32()
            );
            self.frame_cout = 0;
            self.last_update = now;
        }
    }
}
