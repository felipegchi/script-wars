use crate::engine::context::Context;
use crate::engine::instance::Instance;
use crate::engine::mesh::Mesh;
use crate::engine::renderer::Renderer;
use crate::engine::{renderer::Shader, vertex::Vertex};

use super::Drawable;

pub const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.5, 0.5, 0.0],
        tex_coords: [1.0, 0.0],
    },
    Vertex {
        position: [-0.5, -0.5, 0.0],
        tex_coords: [0.0, 1.0],
    },
    Vertex {
        position: [0.5, -0.5, 0.0],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        position: [0.5, 0.5, 0.0],
        tex_coords: [1.0, 1.0],
    },
];

pub const INDICES: &[u16] = &[0, 1, 2, 0, 2, 3];

pub struct SpriteMesh {
    shader: Shader,
    mesh: Mesh,
}

impl SpriteMesh {
    pub fn new(shader: Shader, mesh: Mesh) -> SpriteMesh {
        SpriteMesh { shader, mesh }
    }
}

impl Drawable for SpriteMesh {
    fn render<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
        renderer: &'a Renderer,
        instances: &[Instance],
    ) {
        render_pass.set_pipeline(&renderer.render_pipelines[self.shader.0]);

        render_pass.set_bind_group(0, &renderer.bind_groups[0], &[]);
        render_pass.set_bind_group(1, &renderer.bind_groups[1], &[]);

        render_pass.set_vertex_buffer(0, self.mesh.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.mesh.instance_buffer.slice(..));

        render_pass.set_index_buffer(self.mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.mesh.num_indices, 0, 0..instances.len() as _);
    }

    fn update_uniforms(&self, ctx: &Context, instances: &[crate::engine::instance::Instance]) {
        let data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
        ctx.queue.write_buffer(&self.mesh.instance_buffer, 0, bytemuck::cast_slice(&data))
    }
}
