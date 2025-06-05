use std::io::Cursor;

use anyhow::Ok;
use tokio::io::BufReader;
use wgpu::util::DeviceExt;

use super::{model, object::Context, texture};

fn load_string(file_name: &str) -> anyhow::Result<String> {
    let path = std::path::Path::new(env!("OUT_DIR"))
        .join("res")
        .join(file_name);
    let txt = std::fs::read_to_string(path)?;

    Ok(txt)
}

fn load_binary(file_name: &str) -> anyhow::Result<Vec<u8>> {
    let path = std::path::Path::new(env!("OUT_DIR"))
        .join("res")
        .join(file_name);
    let data = std::fs::read(path)?;

    Ok(data)
}

impl<'a> Context<'a> {
    pub fn load_texture(&self, file_name: &str) -> anyhow::Result<texture::Texture> {
        let data = load_binary(file_name)?;
        texture::Texture::from_bytes(
            &self.renderer_state.device,
            &self.renderer_state.queue,
            &data,
            file_name,
        )
    }

    pub async fn load_model(&self, file_name: &str) -> anyhow::Result<model::Model> {
        let obj_text = load_string(file_name)?;
        let obj_cursor = Cursor::new(obj_text);
        let mut obj_reader = BufReader::new(obj_cursor);

        let (models, obj_materials) = tobj::tokio::load_obj_buf(
            &mut obj_reader,
            &tobj::LoadOptions {
                single_index: true,
                triangulate: true,
                ..Default::default()
            },
            |p| async move {
                let mat_text = load_string(p.to_str().unwrap()).unwrap();
                tobj::load_mtl_buf(&mut std::io::BufReader::new(Cursor::new(mat_text)))
            },
        )
        .await?;

        let mut materials = Vec::new();
        for material in obj_materials? {
            let diffuse_texture = self.load_texture(&material.diffuse_texture.unwrap())?;
            let bind_group =
                self.renderer_state
                    .device
                    .create_bind_group(&wgpu::BindGroupDescriptor {
                        label: None,
                        layout: &self.renderer_state.diffuse_bind_group_layout,
                        entries: &[
                            wgpu::BindGroupEntry {
                                binding: 0,
                                resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                            },
                            wgpu::BindGroupEntry {
                                binding: 1,
                                resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                            },
                        ],
                    });

            materials.push(model::Material {
                name: material.name,
                diffuse_texture,
                bind_group,
            });
        }

        let meshes = models
            .into_iter()
            .map(|m| {
                let vertices = (0..m.mesh.positions.len() / 3)
                    .map(|i| {
                        if m.mesh.normals.is_empty() {
                            model::ModelVertex {
                                position: [
                                    m.mesh.positions[i * 3],
                                    m.mesh.positions[i * 3 + 1],
                                    m.mesh.positions[i * 3 + 2],
                                ],
                                text_coords: [
                                    m.mesh.texcoords[i * 2],
                                    1.0 - m.mesh.texcoords[i * 2 + 1],
                                ],
                                normal: [0.0, 0.0, 0.0],
                            }
                        } else {
                            model::ModelVertex {
                                position: [
                                    m.mesh.positions[i * 3],
                                    m.mesh.positions[i * 3 + 1],
                                    m.mesh.positions[i * 3 + 2],
                                ],
                                text_coords: [
                                    m.mesh.texcoords[i * 2],
                                    1.0 - m.mesh.texcoords[i * 2 + 1],
                                ],
                                normal: [
                                    m.mesh.normals[i * 3],
                                    m.mesh.normals[i * 3 + 1],
                                    m.mesh.normals[i * 3 + 2],
                                ],
                            }
                        }
                    })
                    .collect::<Vec<_>>();

                let device = &self.renderer_state.device;
                let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(&format!("{:?} Vertex Buffer", file_name)),
                    contents: bytemuck::cast_slice(&vertices),
                    usage: wgpu::BufferUsages::VERTEX,
                });
                let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(&format!("{:?} Index Buffer", file_name)),
                    contents: bytemuck::cast_slice(&m.mesh.indices),
                    usage: wgpu::BufferUsages::INDEX,
                });

                model::Mesh {
                    name: file_name.to_string(),
                    vertex_buffer,
                    index_buffer,
                    num_elements: m.mesh.indices.len() as u32,
                    material: m.mesh.material_id.unwrap_or(0),
                }
            })
            .collect::<Vec<_>>();

        Ok(model::Model { meshes, materials })
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
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(format!("{} Index Buffer", label).as_str()),
            contents: bytemuck::cast_slice(&indices),
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

        let material = model::Material {
            name: format!("{} Material", label),
            diffuse_texture: texture,
            bind_group,
        };

        let mesh = model::Mesh {
            name: format!("{} Mesh", label),
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
