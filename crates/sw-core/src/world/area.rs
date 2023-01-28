use super::chunk::ChunkCoord;

/// Quad area generates a rectangular area for a chunk coord.
pub struct QuadArea {
    start: ChunkCoord,
    end: ChunkCoord,
    cur: ChunkCoord,
}

impl QuadArea {
    pub fn new(coord: ChunkCoord, radius: usize) -> QuadArea {
        let start = ChunkCoord {
            x: coord.x.saturating_sub(radius),
            y: coord.y.saturating_sub(radius),
        };

        let end = ChunkCoord {
            x: coord.x.saturating_add(radius),
            y: coord.y.saturating_add(radius),
        };

        QuadArea {
            start,
            end,
            cur: start,
        }
    }
}

impl Iterator for QuadArea {
    type Item = ChunkCoord;

    fn next(&mut self) -> Option<Self::Item> {
        let coord = self.cur;
        if self.cur.y > self.end.y {
            None
        } else {
            if self.cur.x >= self.end.x {
                self.cur.y += 1;
                self.cur.x = self.start.x;
            } else {
                self.cur.x += 1;
            }
            Some(coord)
        }
    }
}
