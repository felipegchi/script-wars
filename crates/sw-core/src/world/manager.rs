//! Module for managing chunks and tickets.

use std::collections::VecDeque;
use std::time::{Duration, Instant};

use ahash::{AHashMap, AHashSet};

use super::chunk::{Chunk, ChunkCoord};
use super::generators::WorldGenerator;
use super::ticket::{Ticket, TicketManager};
use crate::id::{Entity, Id};

pub const TIMEOUT_SECS: Duration = Duration::from_secs(2);

/// A request to unload a chunk after a certain amount of time. It's not unloaded immediately to
/// prevent chunks from being unloaded and loaded again in a short amount of time. In order to
/// unload a chunk please check the [ChunkManager::flush] method.
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

/// A chunk manager is responsible for managing chunks and tickets. It also manages the loading and
/// unloading of chunks.
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

    /// Creates a new ticket based on an entity id. This ticket is used to load chunks.
    pub fn new_ticket(&mut self, entity: Id<Entity>) -> Ticket {
        self.tickets.new_ticket(entity)
    }

    /// Flushes the chunk manager. This removes chunks that are no longer needed.
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

    /// Removes a ticket from a chunk. If the chunk is no longer needed, it is unloaded.
    pub fn remove_ticket(&mut self, ticket: Ticket, coord: ChunkCoord) {
        self.tickets.remove(ticket, coord);
        if let Some(tickets) = self.tickets.get_tickets(&coord) {
            if tickets.is_empty() {
                self.tickets.remove_chunk(&coord);
                self.to_unload.push_back(ChunkUnloadRequest::new(coord))
            }
        }
    }

    /// Adds a ticket to a chunk. If the chunk is not loaded, it is generated. If the chunk is
    /// already loaded, the ticket is added to the chunk
    pub fn add_ticket(&mut self, ticket: Ticket, coord: ChunkCoord) {
        if !self.chunks.contains_key(&coord) {
            // TODO: Check parallelization
            self.chunks.insert(coord, self.generator.generate(coord));
        }

        self.tickets.add(ticket, coord)
    }

    /// TODO: Rewrite this function.
    pub fn set_ticket(&mut self, ticket: Ticket, new_coords: AHashSet<ChunkCoord>) {
        let old_coords = self
            .tickets
            .get_coords(&ticket)
            .cloned()
            .unwrap_or_default();

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
