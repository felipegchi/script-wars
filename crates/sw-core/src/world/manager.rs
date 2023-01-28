use std::collections::VecDeque;
use std::time::{Duration, Instant};

use ahash::{AHashMap, AHashSet};

use crate::id::{Id, Entity};

use super::generators::WorldGenerator;

use super::chunk::{Chunk, ChunkCoord};
use super::ticket::{Ticket, TicketManager};

pub const TIMEOUT_SECS: Duration = Duration::from_secs(2);

pub struct ChunkUnloadRequest {
    pub coord: ChunkCoord,
    pub timeout: Instant,
}

impl ChunkUnloadRequest {
    pub fn new(coord: ChunkCoord) -> ChunkUnloadRequest {
        ChunkUnloadRequest {
            coord,
            timeout: Instant::now() + TIMEOUT_SECS,
        }
    }
}

pub struct ChunkManager {
    pub chunks: AHashMap<ChunkCoord, Chunk>,
    pub to_unload: VecDeque<ChunkUnloadRequest>,
    pub tickets: TicketManager,
    generator: Box<dyn WorldGenerator + Send + Sync>,
}


impl ChunkManager {
    pub fn new(generator: Box<dyn WorldGenerator + Send + Sync>) -> Self {
        ChunkManager {
            chunks: Default::default(),
            to_unload: Default::default(),
            tickets: Default::default(),
            generator,
        }
    }

    pub fn new_ticket(&mut self, entity: Id<Entity>) -> Ticket {
        self.tickets.new_ticket(entity)
    }

    pub fn flush(&mut self) -> Vec<Chunk> {
        let mut chunks = vec![];

        while let Some(request) = self.to_unload.get(0) {
            if !request.timeout.elapsed().is_zero() {
                let request = self.to_unload.pop_front().unwrap();
                if self.tickets.get_tickets(&request.coord).is_none() {
                    if let Some(chunk) = self.chunks.remove(&request.coord) {
                        chunks.push(chunk);
                    }
                }
            } else {
                break;
            }
        }

        chunks
    }

    pub fn remove_ticket(&mut self, ticket: Ticket, coord: ChunkCoord) {
        self.tickets.remove(ticket, coord);
        if let Some(tickets) = self.tickets.get_tickets(&coord) {
            if tickets.is_empty() {
                self.tickets.remove_chunk(&coord);
                self.to_unload.push_back(ChunkUnloadRequest::new(coord))
            }
        }
    }

    pub fn add_ticket(&mut self, ticket: Ticket, coord: ChunkCoord) {
        if !self.chunks.contains_key(&coord) {
            self.chunks.insert(coord, self.generator.generate(coord));
        }

        self.tickets.add(ticket, coord)
    }

    pub fn set_ticket(&mut self, ticket: Ticket, new_coords: AHashSet<ChunkCoord>) {
        let old_coords = self.tickets.get_coords(&ticket).cloned().unwrap_or_default();

        let removed_coords: Vec<_> = old_coords.difference(&new_coords).cloned().collect();
        let new_coords: Vec<_> = new_coords.difference(&old_coords).cloned().collect();

        for removed in removed_coords {
            self.remove_ticket(ticket, removed);
        }

        for added in new_coords {
            self.add_ticket(ticket, added);
        }
    }
}
