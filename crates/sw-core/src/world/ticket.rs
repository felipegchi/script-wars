//! Module for managing chunks and tickets. A Ticket is an abstraction of an entity that is loading
//! chunks. A [TicketManager] is used to manage tickets and chunks.

use super::ChunkCoord;
use crate::id::{Entity, Id};
use ahash::{AHashMap, AHashSet};

/// A ticket is an abstraction of an entity that is loading chunks.
#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub struct Ticket(Id<Entity>);

#[derive(Default)]
pub struct TicketManager {
    by_chunk: AHashMap<ChunkCoord, AHashSet<Ticket>>,
    by_ticket: AHashMap<Ticket, AHashSet<ChunkCoord>>,
}

impl TicketManager {
    /// Creates a new ticket based on an entity id.
    pub fn new_ticket(&mut self, entity: Id<Entity>) -> Ticket {
        Ticket(entity)
    }

    /// Adds a ticket to a chunk.
    pub fn add(&mut self, ticket: Ticket, chunk: ChunkCoord) {
        self.by_ticket.entry(ticket).or_default().insert(chunk);
        self.by_chunk.entry(chunk).or_default().insert(ticket);
    }

    /// Removes a ticket from a chunk.
    pub fn remove(&mut self, ticket: Ticket, chunk: ChunkCoord) {
        self.by_ticket.entry(ticket).and_modify(|coords| {
            coords.remove(&chunk);
        });

        self.by_chunk.entry(chunk).and_modify(|tickets| {
            tickets.remove(&ticket);
        });
    }

    /// Removes a chunk from the ticket manager.
    pub fn remove_chunk(&mut self, coord: &ChunkCoord) -> Option<AHashSet<Ticket>> {
        self.by_chunk.remove(coord)
    }

    /// Gets the tickets for a chunk.
    pub fn get_tickets(&self, coord: &ChunkCoord) -> Option<&AHashSet<Ticket>> {
        self.by_chunk.get(coord)
    }

    /// Gets the chunks for a ticket.
    pub fn get_coords(&self, ticket: &Ticket) -> Option<&AHashSet<ChunkCoord>> {
        self.by_ticket.get(ticket)
    }

    /// Sets the chunks for a ticket.
    pub fn set_coords(&mut self, ticket: Ticket, coords: AHashSet<ChunkCoord>) {
        self.by_ticket.insert(ticket, coords);
    }
}
