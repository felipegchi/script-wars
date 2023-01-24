use cgmath::Vector3;

use super::vertex::Desc;

/// Instances of a Mesh.
pub struct Instance {
    pub(crate) position: cgmath::Point2<f32>,
    pub(crate) tex_coords: cgmath::Point2<f32>,
    pub(crate) z_index: f32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct InstanceRaw {
    pub(crate) position: [f32; 2],
    pub(crate) tex_coords: [f32; 2]
}

impl Instance {
    pub fn to_raw(&self) -> InstanceRaw {
        InstanceRaw {
            position: self.position.into(),
            tex_coords: self.tex_coords.into()
        }
    }
}

impl InstanceRaw {
    const ATTRIBS: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        2 => Float32x2,
        3 => Float32x2
    ];
}

impl Desc for InstanceRaw {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBS,
        }
    }
}
