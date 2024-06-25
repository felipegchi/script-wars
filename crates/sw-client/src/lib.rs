use structures::{graphics::Graphics, rect::ChunkModel};
use sw_engine::Plugin;
use systems::render::RendererSystem;

pub mod systems;
pub mod structures;

pub struct RendererPlugin {
    pub graphics: Graphics,
}

impl Plugin for RendererPlugin {
    fn setup(self, world: &mut sw_engine::WorldBuilder) {
        world.with_resource(self.graphics);
        world.with_resource(ChunkModel::new());
        world.with_system(RendererSystem, "renderer system", &[]);
    }
}