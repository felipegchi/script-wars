use crate::structures::graphics::Graphics;
use specs::{System, WriteExpect};
use sw_renderer::Pass;

/// Renders all the meshes.
pub struct RendererSystem;

impl<'a> System<'a> for RendererSystem {
    type SystemData = WriteExpect<'a, Graphics>;

    fn run(&mut self, mut info: Self::SystemData) {
        info.update_camera();

        // Gives a surface to create a new frame of.
        let output = info.renderer.surface.get_current_texture().unwrap();

        // Describes a new texture view so it can handle textures from the surface.
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // Create a new encoder so we can just send commands to the GPU in a queue.
        let mut encoder =
            info.renderer
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

        info.primary
            .draw(&view, &mut encoder, &info.materials, &[], &info.globals);

        info.ui
            .draw(&view, &mut encoder, &info.materials, &[], &info.globals);

        // Submits the commands to the GPU.
        info.renderer.queue.submit(std::iter::once(encoder.finish()));

        // Presents the frame to the screen.
        output.present();
    }
}
