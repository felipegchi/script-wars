/// A chunk is a 64x64 tile area of the world. It is used to store the world data in a more efficient
/// way. The world is split into chunks, and the chunks are generated on demand.

pub const CHUNK_WIDTH: usize = 64;
pub const CHUNK_HEIGHT: usize = 64;

/// A chunk coordinate is a coordinate of a chunk in the world.
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

/// A tile is a single tile in the world like a grass or water tile.
#[derive(Clone, Copy)]
pub struct Tile(pub usize);

/// A chunk is a 64x64 tile area of the world. It is used to store the world data in a more
/// efficient way. The world is split into chunks, and the chunks are generated on demand.
pub struct Chunk {
    pub data: Vec<[Tile; CHUNK_WIDTH * CHUNK_HEIGHT]>,
    pub depth: usize,
}
