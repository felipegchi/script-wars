use std::time::Instant;

use specs::Builder;
use specs::Component;
use specs::DenseVecStorage;
use specs::World;
use specs::WorldExt;

pub type EventCallback = &'static dyn Fn(&specs::World, Event);

pub enum Event {
    NewEntity(specs::Entity)
}

#[derive(Component, Debug)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

pub struct State {
    pub world: World,
    pub events: EventCallback,
    pub last_time: Instant
}

impl State {
    pub fn new(events: EventCallback) -> Self {
        let mut world: specs::World = World::new();

        world.register::<Position>();

        State {
            world,
            events,
            last_time: instant::Instant::now()
        }
    }

    pub fn init(&mut self) {
        let entity = self.world
            .create_entity()
            .with(Position { x: 0.0, y: 0.0 })
            .build();

        (self.events)(&self.world, Event::NewEntity(entity));

        let entity = self.world
            .create_entity()
            .with(Position { x: 0.0, y: 1.2 })
            .build();

        (self.events)(&self.world, Event::NewEntity(entity))
    }

    pub fn update(&mut self) {
        let now = instant::Instant::now();
        let delta = now - self.last_time;
        self.last_time = now;

        
    }
}
