use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TicketStatus {
    Available,
    Reserved,
    Sold,
    Cancelled,
    CheckedIn,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticket {
    pub id: Uuid,
    pub event_id: Uuid,
    pub user_id: Option<Uuid>,
    pub price_cents: u32,
    pub currency: String,
    pub status: TicketStatus,
    pub seat_number: Option<String>,
    pub tier: Option<String>,
    pub issued_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub metadata: Option<String>,
}

impl Ticket {
    pub fn new(
        event_id: Uuid,
        price_cents: u32,
        currency: String,
        tier: Option<String>,
        seat_number: Option<String>,
        metadata: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Ticket {
            id: Uuid::new_v4(),
            event_id,
            user_id: None,
            price_cents,
            currency,
            status: TicketStatus::Available,
            seat_number,
            tier,
            issued_at: now,
            updated_at: now,
            metadata,
        }
    }

    pub fn reserve(&mut self, user_id: Uuid) -> Result<(), String> {
        match self.status {
            TicketStatus::Available => {
                self.status = TicketStatus::Reserved;
                self.user_id = Some(user_id);
                self.updated_at = Utc::now();
                Ok(())
            }
            _ => Err("Ticket is not available for reservation.".to_string()),
        }
    }

    pub fn sell(&mut self, user_id: Uuid) -> Result<(), String> {
        match self.status {
            TicketStatus::Available | TicketStatus::Reserved => {
                self.status = TicketStatus::Sold;
                self.user_id = Some(user_id);
                self.updated_at = Utc::now();
                Ok(())
            }
            _ => Err("Ticket is not available for sale.".to_string()),
        }
    }

    pub fn cancel(&mut self) {
        self.status = TicketStatus::Cancelled;
        self.updated_at = Utc::now();
    }

    pub fn check_in(&mut self) -> Result<(), String> {
        if self.status == TicketStatus::Sold {
            self.status = TicketStatus::CheckedIn;
            self.updated_at = Utc::now();
            Ok(())
        } else {
            Err("Only sold tickets can be checked in.".to_string())
        }
    }

    pub fn update_seat(&mut self, new_seat: Option<String>) {
        self.seat_number = new_seat;
        self.updated_at = Utc::now();
    }

    pub fn update_tier(&mut self, new_tier: Option<String>) {
        self.tier = new_tier;
        self.updated_at = Utc::now();
    }

    pub fn summary(&self) -> String {
        format!(
            "Ticket ID: {}, Event: {}, Status: {:?}, Price: {} {}",
            self.id, self.event_id, self.status, self.price_cents, self.currency
        )
    }
}

pub mod ticket_filters {
    use super::*;

    pub fn available_tickets(tickets: &[Ticket]) -> Vec<Ticket> {
        tickets
            .iter()
            .cloned()
            .filter(|t| t.status == TicketStatus::Available)
            .collect()
    }

    pub fn user_tickets(tickets: &[Ticket], user_id: Uuid) -> Vec<Ticket> {
        tickets
            .iter()
            .cloned()
            .filter(|t| t.user_id == Some(user_id))
            .collect()
    }

    pub fn tickets_for_event(tickets: &[Ticket], event_id: Uuid) -> Vec<Ticket> {
        tickets
            .iter()
            .cloned()
            .filter(|t| t.event_id == event_id)
            .collect()
    }
}

pub mod validation {
    use super::*;

    pub fn validate_ticket(ticket: &Ticket) -> Result<(), String> {
        if ticket.price_cents == 0 {
            return Err("Ticket price must be greater than 0.".to_string());
        }
        if ticket.currency.trim().is_empty() {
            return Err("Currency cannot be empty.".to_string());
        }
        Ok(())
    }
}
