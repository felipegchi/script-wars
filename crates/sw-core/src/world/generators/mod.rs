//! Traits and structures for world generation. The world generator is responsible for generating
//! chunks of the world.

use super::chunk::{Chunk, ChunkCoord};

pub mod classic;

/// Trait for world generators that takes only the coordinates
/// of the chunk and deterministically returns a piece of the
/// map (chunk).
pub trait WorldGenerator {
    fn generate(&mut self, coord: ChunkCoord) -> Chunk;
}
