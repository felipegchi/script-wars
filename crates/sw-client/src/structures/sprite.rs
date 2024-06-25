//! Sprite component so we can render stuff to the screen.

use specs::{Component, VecStorage};

#[derive(Component)]
#[storage(VecStorage)]
pub struct Sprite {
    texture: usize
}