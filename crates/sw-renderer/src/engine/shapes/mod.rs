use super::{context::Context, instance::Instance, renderer::Renderer};

pub mod rect;

pub trait Drawable {
    fn render<'a>(
        &'a self,
        render_pass: &mut wgpu::RenderPass<'a>,
        renderer: &'a Renderer,
        instances: &[crate::engine::instance::Instance],
    );

    fn update_uniforms(&self, ctx: &Context, instances: &[crate::engine::instance::Instance]);
}
