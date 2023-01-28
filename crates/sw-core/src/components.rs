use specs::{Component, DenseVecStorage};

#[derive(Component)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}