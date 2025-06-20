use std::ops::Range;

pub(in crate::engine) trait Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static>;
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ModelVertex {
    pub position: [f32; 3],
    pub text_coords: [f32; 2],
    pub normal: [f32; 3],
    pub tex_index: u32,
}

impl Vertex for ModelVertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<ModelVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    shader_location: 2,
                },
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Uint32,
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 3,
                },
            ],
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Material {
    pub(in crate::engine) bind_group: wgpu::BindGroup,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Mesh {
    pub(in crate::engine) vertex_buffer: wgpu::Buffer,
    pub(in crate::engine) index_buffer: wgpu::Buffer,
    pub(in crate::engine) num_elements: u32,
    pub material: usize,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Model {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
}

pub(in crate::engine) trait DrawModel<'a> {
    fn draw_mesh_instanced(
        &mut self,
        mesh: &'a Mesh,
        material: &'a Material,
        camera_bind_group: &'a wgpu::BindGroup,
        instances: Range<u32>,
    );
    fn draw_model(&mut self, model: &'a Model, camera_bind_group: &'a wgpu::BindGroup);
    fn draw_model_instanced(
        &mut self,
        model: &'a Model,
        camera_bind_group: &'a wgpu::BindGroup,
        instances: Range<u32>,
    );
}

impl<'a, 'b> DrawModel<'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn draw_mesh_instanced(
        &mut self,
        mesh: &'b Mesh,
        material: &'b Material,
        camera_bind_group: &'b wgpu::BindGroup,
        instances: Range<u32>,
    ) {
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        self.set_bind_group(0, &material.bind_group, &[]);
        self.set_bind_group(1, camera_bind_group, &[]);
        self.draw_indexed(0..mesh.num_elements, 0, instances);
    }

    fn draw_model(&mut self, model: &'b Model, camera_bind_group: &'b wgpu::BindGroup) {
        self.draw_model_instanced(model, camera_bind_group, 0..1);
    }

    fn draw_model_instanced(
        &mut self,
        model: &'b Model,
        camera_bind_group: &'b wgpu::BindGroup,
        instances: Range<u32>,
    ) {
        for mesh in &model.meshes {
            let material = &model.materials[mesh.material];
            self.draw_mesh_instanced(mesh, material, camera_bind_group, instances.clone());
        }
    }
}
