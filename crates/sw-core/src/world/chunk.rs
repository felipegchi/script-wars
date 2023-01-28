pub const CHUNK_WIDTH: usize = 64;
pub const CHUNK_HEIGHT: usize = 64;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct ChunkCoord {
    pub x: usize,
    pub y: usize,
}

impl ChunkCoord {
    pub fn new(x: usize, y: usize) -> ChunkCoord {
        ChunkCoord { x, y }
    }
}

#[derive(Clone, Copy)]
pub enum Tile {
    Grass,
    Wall,
}

pub struct Chunk {
    pub data: [Tile; CHUNK_WIDTH * CHUNK_HEIGHT],
}
