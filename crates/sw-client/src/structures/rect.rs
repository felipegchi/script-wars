//! Definition of a Cube in the renderer realm. It's used in order to generate chunks and other structures
//! based on Cubes, its not widely used by itself.

use cgmath::Vector3;
use sw_engine::Position;
use sw_renderer::{MaterialId, Mesh, Model, ModelIndex, ModelInstance, ModelVertex, Renderer};

macro_rules! vertex {
    ($position:expr, $tex_coords:expr) => {
        ModelVertex {
            position: $position,
            tex_coords: $tex_coords,
        }
    };
}

#[rustfmt::skip]
pub const VERTICES: &[ModelVertex] = &[
    vertex!([-1.0, -1.0, 1.0], [1.0, 1.0]),
    vertex!([1.0, -1.0, 1.0], [0.0, 1.0]), 
    vertex!([1.0, 1.0, 1.0], [0.0, 0.0]),  
    vertex!([-1.0, 1.0, 1.0], [1.0, 0.0]), 
];

#[rustfmt::skip]
pub const INDICES: &[ModelIndex] = &[0, 1, 2, 3, 0, 2];

pub fn face(number: usize) -> &'static [ModelVertex] {
    &VERTICES[number * 4..(number + 1) * 4]
}

pub struct ChunkModel {
    pub model: Model,
    pub mesh: Mesh,
}


impl ChunkModel {
    pub fn from_data(
        position: &Position,
        material_id: MaterialId,
        renderer: &Renderer,
    ) -> (Model, Mesh) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        indices.extend(INDICES.iter().map(|x| x + vertices.len() as u16));
        vertices.extend(VERTICES.iter());

        let mesh = Mesh::from_vertex(
            renderer,
            "Chunk".to_owned(),
            &vertices,
            &indices,
            &[ModelInstance::from_position(Vector3::new(position.x as f32, position.y as f32, position.z as f32))],
            material_id,
        );

        (Model { vertices, indices }, mesh)
    }
}