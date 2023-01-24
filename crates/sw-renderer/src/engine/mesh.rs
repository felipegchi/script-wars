use super::instance::{Instance, InstanceRaw};
use super::{context::Context, vertex::Vertex};
use wgpu::util::DeviceExt;
use wgpu::BufferDescriptor;

/// Stores vertex and indice info.
pub struct Mesh {
    pub(crate) vertex_buffer: wgpu::Buffer,
    pub(crate) index_buffer: wgpu::Buffer,
    pub(crate) instance_buffer: wgpu::Buffer,
    pub(crate) num_indices: u32,
}

impl Mesh {
    pub fn new(context: &Context, vertices: &[Vertex], indices: &[u16]) -> Mesh {
        let vertex_buffer = context
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let instance_buffer = context.device.create_buffer(&BufferDescriptor {
            label: Some("Instance Buffer"),
            size: std::mem::size_of::<[InstanceRaw; 32]>() as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let index_buffer = context
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(indices),
                usage: wgpu::BufferUsages::INDEX,
            });

        Mesh {
            vertex_buffer,
            index_buffer,
            instance_buffer,
            num_indices: indices.len() as u32,
        }
    }
}
