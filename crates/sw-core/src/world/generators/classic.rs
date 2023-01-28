use noise::NoiseFn;

use crate::world::chunk::{Chunk, ChunkCoord, Tile, CHUNK_HEIGHT, CHUNK_WIDTH};

use super::WorldGenerator;

const MAGNITUDE: usize = 2000;
const FREQUENCY: f64 = 50.0;

pub struct ClassicGenenerator {
    noise: noise::Perlin,
}

impl ClassicGenenerator {
    pub fn new(seed: u32) -> ClassicGenenerator {
        ClassicGenenerator {
            noise: noise::Perlin::new(seed),
        }
    }
}

impl WorldGenerator for ClassicGenenerator {
    fn generate(&mut self, coord: ChunkCoord) -> Chunk {
        let mut data: [Tile; CHUNK_WIDTH * CHUNK_HEIGHT] = [Tile::Grass; CHUNK_WIDTH * CHUNK_HEIGHT];

        for y in 0..CHUNK_HEIGHT {
            for x in 0..CHUNK_WIDTH {
                let index = CHUNK_HEIGHT * y + x;

                let abs_x = FREQUENCY * ((coord.x * CHUNK_WIDTH + x) as f64 / MAGNITUDE as f64);
                let abs_y = FREQUENCY * ((coord.y * CHUNK_HEIGHT + y) as f64 / MAGNITUDE as f64);

                let value = self.noise.get([abs_x, abs_y])
                    + 0.5 * self.noise.get([abs_x * 2.0, abs_y * 2.0]);

                if f64::powf(value, 0.2) > 0.1 {
                    data[index] = Tile::Wall;
                } else {
                    data[index] = Tile::Grass;
                }
            }
        }

        Chunk { data }
    }
}
