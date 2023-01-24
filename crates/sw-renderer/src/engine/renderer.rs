use std::{mem};

use wgpu::RenderPipeline;

use super::{context::Context, instance::Instance, mesh::Mesh, shapes::Drawable, camera::Camera};

#[derive(Clone, Copy)]
pub struct Shader(pub(crate) usize);

#[derive(Clone, Copy)]
pub struct Resource(pub(crate) usize);

/// Global data
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Globals {
    pub(crate) view_proj: [[f32; 4]; 4],
}

/// Store important info in order to render multiple
/// objects and uniform buffers
pub struct Renderer {
    pub(crate) render_pipelines: Vec<wgpu::RenderPipeline>,
    pub(crate) texture_bind_layout: wgpu::BindGroupLayout,
    pub(crate) global_bind_layout: wgpu::BindGroupLayout,
    pub(crate) bind_groups: Vec<wgpu::BindGroup>,
    pub(crate) uniform_buffers: Vec<wgpu::Buffer>,
    pub(crate) resources: Vec<Box<dyn Drawable>>,
    pub(crate) sampler: wgpu::Sampler,
    pub(crate) globals: Globals,
}

impl Renderer {
    pub fn new(context: &Context) -> Renderer {
        // Describes a Texture and the Sampler.
        let texture_descriptor = &wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("Renderer: Texture and Sampler"),
        };

        let texture_bind_layout = context.device.create_bind_group_layout(texture_descriptor);

        // Global Bind Group
        let global_desc = &wgpu::BindGroupLayoutDescriptor {
            label: Some("Renderer: Globals"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(
                        mem::size_of::<[[f32; 4]; 4]>() as wgpu::BufferAddress
                    ),
                },
                count: None,
            }],
        };

        let global_bind_layout = context.device.create_bind_group_layout(global_desc);

        let global_buffer = context.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Index Buffer"),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            size: mem::size_of::<Globals>() as u64,
            mapped_at_creation: false,
        });

        let global_bind_group = context
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("Globals"),
                layout: &global_bind_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: global_buffer.as_entire_binding(),
                }],
            });

        let sampler = context.device.create_sampler(
            &wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::ClampToEdge,
                address_mode_v: wgpu::AddressMode::ClampToEdge,
                address_mode_w: wgpu::AddressMode::ClampToEdge,
                mag_filter: wgpu::FilterMode::Linear,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            }
        );

        Renderer {
            bind_groups: vec![global_bind_group],
            uniform_buffers: vec![global_buffer],
            render_pipelines: Vec::new(),
            texture_bind_layout,
            global_bind_layout,
            resources: Vec::new(),
            globals: Globals {
                view_proj: Default::default(),
            },
            sampler,
        }
    }

    pub fn add_pipeline(&mut self, pipeline: RenderPipeline) -> Shader {
        self.render_pipelines.push(pipeline);
        Shader(self.render_pipelines.len() - 1)
    }

    pub fn add_resource(&mut self, resource: Box<dyn Drawable>) -> Resource {
        self.resources.push(resource);
        Resource(self.resources.len() - 1)
    }

    pub fn render<'a>(&mut self, context: &Context, to_draw: &[(Resource, Vec<Instance>)], globals: Globals) {
        let output = context.surface.get_current_texture().unwrap();

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        context.queue.write_buffer(&self.uniform_buffers[0], 0, bytemuck::cast_slice(&[globals]));

        let mut encoder = context
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            for (resource, instances) in to_draw {
                let drawable = &self.resources[resource.0];
                drawable.update_uniforms(context, instances);
                drawable.render(&mut render_pass, self, instances);
            }
        }

        context.queue.submit(Some(encoder.finish()));
        output.present();
    }
}
