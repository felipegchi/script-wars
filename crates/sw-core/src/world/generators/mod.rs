use super::chunk::{Chunk, ChunkCoord};

pub mod classic;

// TODO: Change to channels if this take too much time.

/// Trait for world generators that takes only the coordinates
/// of the chunk and deterministically returns a piece of the
/// map (chunk).
pub trait WorldGenerator {
    fn generate(&mut self, coord: ChunkCoord) -> Chunk;
}
