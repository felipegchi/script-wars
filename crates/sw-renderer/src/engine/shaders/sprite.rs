use std::fs;

use crate::engine::instance::InstanceRaw;
use crate::engine::vertex::Desc;
use crate::engine::vertex::Vertex;
use crate::engine::Renderer;
use crate::engine::Context;

pub fn sprite_pipeline(context: &Context, renderer: &Renderer) -> wgpu::RenderPipeline {

    let file = fs::read_to_string("./crates/sw-renderer/src/resources/shader.wgsl").unwrap();

    let shader = context
        .device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Renderer: Sprite Render"),
            source: wgpu::ShaderSource::Wgsl(file.into()),
        });

    let render_pipeline_layout =
        context
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Renderer: Sprite Render Pipeline Layout"),
                bind_group_layouts: &[&renderer.global_bind_layout],
                push_constant_ranges: &[],
            });

    context
        .device
        .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Renderer: Sprite Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc(), InstanceRaw::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: context.config.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        })
}
