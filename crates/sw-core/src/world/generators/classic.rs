//! This is a classic world generator, which generates a world with a perlin noise. It is used in
//! the `World` struct to generate chunks of the world.

use noise::NoiseFn;

use crate::world::chunk::{Chunk, ChunkCoord, Tile, CHUNK_HEIGHT, CHUNK_WIDTH};

use super::WorldGenerator;

const MAGNITUDE: usize = 2000;
const FREQUENCY: f64 = 50.0;

const CHUNK_TILES: usize = CHUNK_WIDTH * CHUNK_HEIGHT;

/// A classic world generator. It generates a world with a perlin noise.
pub struct ClassicGenerator {
    noise: noise::Perlin,
}

impl ClassicGenerator {
    pub fn new(seed: u32) -> ClassicGenerator {
        ClassicGenerator {
            noise: noise::Perlin::new(seed),
        }
    }
}

impl WorldGenerator for ClassicGenerator {
    fn generate(&mut self, coord: ChunkCoord) -> Chunk {
        let mut data: [Tile; CHUNK_TILES] = [Tile(0); CHUNK_TILES];

        for y in 0..CHUNK_HEIGHT {
            for x in 0..CHUNK_WIDTH {
                let index = CHUNK_HEIGHT * y + x;

                let abs_x = FREQUENCY * ((coord.x * CHUNK_WIDTH + x) as f64 / MAGNITUDE as f64);
                let abs_y = FREQUENCY * ((coord.y * CHUNK_HEIGHT + y) as f64 / MAGNITUDE as f64);

                let value = self.noise.get([abs_x, abs_y])
                    + 0.5 * self.noise.get([abs_x * 2.0, abs_y * 2.0]);

                let value = f64::powf(value, 0.2);

                data[index] = Tile(if value > 0.1 { 0 } else { 1 });
            }
        }

        Chunk {
            data: vec![data],
            depth: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classic_generator() {
        let mut generator = ClassicGenerator::new(2);

        let chunk = generator.generate(ChunkCoord::new(0, 0));

        for x in 0..CHUNK_HEIGHT {
            for y in 0..CHUNK_WIDTH {
                let index = CHUNK_HEIGHT * x + y;
                let tile = chunk.data[0][index];
                assert!(tile.0 == 0 || tile.0 == 1);
            }
        }
    }
}
