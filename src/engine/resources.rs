use anyhow::Ok;
use wgpu::util::DeviceExt;

use super::{model, object::Context, texture};

fn load_binary(file_name: &str) -> anyhow::Result<Vec<u8>> {
    let path = std::path::Path::new(env!("OUT_DIR"))
        .join("res")
        .join(file_name);
    let data = std::fs::read(path)?;

    Ok(data)
}

impl<'a> Context<'a> {
    pub fn load_texture_array(&self, file_names: &[&str]) -> anyhow::Result<texture::Texture> {
        let mut images = Vec::new();

        for file_name in file_names {
            let data = load_binary(file_name)?;
            let img = image::load_from_memory(&data)?;
            images.push(img);
        }

        let (width, height) = (images[0].width(), images[0].height());
        for (i, img) in images.iter().enumerate() {
            if img.width() != width || img.height() != height {
                return Err(anyhow::anyhow!(
                    "Texture {} has different dimensions ({}, {}) than first texture ({}, {})",
                    file_names[i],
                    img.width(),
                    img.height(),
                    width,
                    height
                ));
            }
        }

        texture::Texture::from_image_array(
            &self.renderer_state.device,
            &self.renderer_state.queue,
            &images,
            Some(&format!("texture_array_{}", file_names.join("_"))),
        )
    }

    pub fn create_model(
        &self,
        vertices: &[model::ModelVertex],
        indices: &[u32],
        texture: texture::Texture,
        label: &str,
    ) -> anyhow::Result<model::Model> {
        let device = &self.renderer_state.device;

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(format!("{} Vertex Buffer", label).as_str()),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(format!("{} Index Buffer", label).as_str()),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(format!("{} Texture Bind Group", label).as_str()),
            layout: &self.renderer_state.diffuse_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
            ],
        });

        let material = model::Material { bind_group };

        let mesh = model::Mesh {
            vertex_buffer,
            index_buffer,
            num_elements: indices.len() as u32,
            material: 0,
        };

        Ok(model::Model {
            meshes: vec![mesh],
            materials: vec![material],
        })
    }
}
