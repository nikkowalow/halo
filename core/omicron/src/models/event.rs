use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EventStatus {
    Draft,
    Published,
    Cancelled,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub location: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub capacity: Option<u32>,
    pub is_private: bool,
    pub creator_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub status: EventStatus,
    pub categories: Vec<String>,
    pub tags: Vec<String>,
    pub image_url: Option<String>,
}

impl Event {
    pub fn new(
        title: String,
        description: Option<String>,
        location: String,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        capacity: Option<u32>,
        is_private: bool,
        creator_id: Uuid,
        categories: Vec<String>,
        tags: Vec<String>,
        image_url: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Event {
            id: Uuid::new_v4(),
            title,
            description,
            location,
            start_time,
            end_time,
            capacity,
            is_private,
            creator_id,
            created_at: now,
            updated_at: now,
            status: EventStatus::Draft,
            categories,
            tags,
            image_url,
        }
    }

    pub fn publish(&mut self) {
        self.status = EventStatus::Published;
        self.updated_at = Utc::now();
    }

    pub fn cancel(&mut self) {
        self.status = EventStatus::Cancelled;
        self.updated_at = Utc::now();
    }

    pub fn complete(&mut self) {
        self.status = EventStatus::Completed;
        self.updated_at = Utc::now();
    }

    pub fn update_title(&mut self, new_title: String) {
        self.title = new_title;
        self.updated_at = Utc::now();
    }

    pub fn update_description(&mut self, new_desc: Option<String>) {
        self.description = new_desc;
        self.updated_at = Utc::now();
    }

    pub fn update_time(&mut self, start: DateTime<Utc>, end: DateTime<Utc>) {
        self.start_time = start;
        self.end_time = end;
        self.updated_at = Utc::now();
    }

    pub fn update_location(&mut self, location: String) {
        self.location = location;
        self.updated_at = Utc::now();
    }

    pub fn update_capacity(&mut self, capacity: Option<u32>) {
        self.capacity = capacity;
        self.updated_at = Utc::now();
    }

    pub fn add_category(&mut self, category: String) {
        if !self.categories.contains(&category) {
            self.categories.push(category);
            self.updated_at = Utc::now();
        }
    }

    pub fn remove_category(&mut self, category: &str) {
        self.categories.retain(|c| c != category);
        self.updated_at = Utc::now();
    }

    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
            self.updated_at = Utc::now();
        }
    }

    pub fn remove_tag(&mut self, tag: &str) {
        self.tags.retain(|t| t != tag);
        self.updated_at = Utc::now();
    }

    pub fn is_upcoming(&self) -> bool {
        self.start_time > Utc::now()
    }

    pub fn is_ongoing(&self) -> bool {
        let now = Utc::now();
        self.start_time <= now && self.end_time >= now
    }

    pub fn is_past(&self) -> bool {
        self.end_time < Utc::now()
    }

    pub fn time_until_start(&self) -> chrono::Duration {
        self.start_time - Utc::now()
    }

    pub fn time_until_end(&self) -> chrono::Duration {
        self.end_time - Utc::now()
    }

    pub fn to_summary(&self) -> String {
        format!(
            "{} - {} at {} (Status: {:?})",
            self.title, self.start_time, self.location, self.status
        )
    }
}

pub mod validation {
    use super::*;

    pub fn validate_event(event: &Event) -> Result<(), String> {
        if event.title.trim().is_empty() {
            return Err("Event title cannot be empty".to_string());
        }
        if event.start_time >= event.end_time {
            return Err("Start time must be before end time".to_string());
        }
        Ok(())
    }
}

pub mod search {
    use super::*;

    pub fn matches_query(event: &Event, query: &str) -> bool {
        let q = query.to_lowercase();
        event.title.to_lowercase().contains(&q)
            || event
                .description
                .as_ref()
                .map(|d| d.to_lowercase().contains(&q))
                .unwrap_or(false)
            || event.location.to_lowercase().contains(&q)
            || event.tags.iter().any(|tag| tag.to_lowercase().contains(&q))
    }

    pub fn filter_upcoming(events: &[Event]) -> Vec<Event> {
        events.iter().cloned().filter(|e| e.is_upcoming()).collect()
    }
}
