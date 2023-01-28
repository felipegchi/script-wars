pub mod components;
pub mod entity;
pub mod id;
pub mod world;

use components::Position;
use rand::Rng;
use specs::prelude::*;
use world::generators::classic::ClassicGenenerator;
use world::manager::ChunkManager;

use shrev::EventChannel;

pub enum Event {
    A,
    B,
}

#[derive(Default)]
pub struct EventHandler {
    reader: Option<ReaderId<Event>>,
}

impl<'a> System<'a> for EventHandler {
    type SystemData = Read<'a, EventChannel<Event>>;

    fn run(&mut self, data: Self::SystemData) {
        for event in data.read(&mut self.reader.as_mut().unwrap()) {
            match event {
                Event::A => (),
                Event::B => (),
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.reader = Some(world.fetch_mut::<EventChannel<Event>>().register_reader());
    }
}