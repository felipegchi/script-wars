//! The entrypoint for the Voxelia engine. This thing exposes a way to create a simulation of a voxelia
//! world and provide ways to interact with the world in a high-level way.

pub mod core;
pub mod events;

pub use core::*;

use specs::{Component, VecStorage};

/// Generic position in the world or the screen.
#[derive(Component)]
#[storage(VecStorage)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Position {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

/// Plugin for rendering and creating chunks.
pub struct BasicPlugin;

impl Plugin for BasicPlugin {
    fn setup(self, world: &mut WorldBuilder) {
        world.with_component::<Position>()
    }
}