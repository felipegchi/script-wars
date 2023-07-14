//! This is the event system used for handling user events. It is required for inverting the control
//! between the engine and the game. The engine should always be in control of the world and entity
//! rules and the user should only make some actions in order to change the direction of each entity.

use shrev::EventChannel;
use specs::prelude::*;

/// Events that users can trigger.
pub enum UserEvent {
    CreateEntity,
    DestroyEntity,
}

pub struct EventHandler {
    reader: ReaderId<UserEvent>,
}

impl<'a> System<'a> for EventHandler {
    type SystemData = Read<'a, EventChannel<UserEvent>>;

    fn run(&mut self, data: Self::SystemData) {
        for event in data.read(&mut self.reader) {
            match event {
                UserEvent::CreateEntity => println!("ata"),
                UserEvent::DestroyEntity => println!("boom"),
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.reader = world
            .fetch_mut::<EventChannel<UserEvent>>()
            .register_reader();
    }
}
