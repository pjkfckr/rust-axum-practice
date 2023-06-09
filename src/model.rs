//! Simplistic Model Layer
//! (with mock-store layer)

use crate::ctx::Ctx;
use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// region: --- Ticket Types
#[derive(Clone, Debug, Serialize)]
pub struct Ticket {
    pub id: u64,
    pub cid: u64,
    pub title: String,
}

#[derive(Deserialize)]
pub struct TicketForCreate {
    pub title: String,
}

// endregion: --- Ticket Types

// region: --- Model Controller
#[derive(Clone)]
pub struct ModelController {
    tickets_store: Arc<Mutex<Vec<Option<Ticket>>>>,
}

// Constructor
impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            tickets_store: Arc::new(Mutex::new(vec![])),
        })
    }
}

// CRUD Implementation
impl ModelController {
    pub async fn create_ticket(&self, _ctx: Ctx, ticket_fc: TicketForCreate) -> Result<Ticket> {
        let mut tickets_store = self.tickets_store.lock().unwrap();
        let id = tickets_store.len() as u64;
        let ticket = Ticket {
            id,
            cid: _ctx.user_id(),
            title: ticket_fc.title,
        };
        tickets_store.push(Some(ticket.clone()));
        Ok(ticket)
    }

    pub async fn list_tickets(&self, _ctx: Ctx) -> Result<Vec<Ticket>> {
        let ticket_store = self.tickets_store.lock().unwrap();

        let tickets = ticket_store.iter().filter_map(|t| t.clone()).collect();
        Ok(tickets)
    }

    pub async fn delete_ticket(&self, _ctx: Ctx, id: u64) -> Result<Ticket> {
        let mut tickets_store = self.tickets_store.lock().unwrap();

        let ticket = tickets_store.get_mut(id as usize).and_then(|t| t.take());
        match ticket {
            Some(ticket) => Ok(ticket),
            None => Err(Error::TicketDeleteFailIdNotFound { id }),
        }
    }
}

// endregion: --- Model Controller
