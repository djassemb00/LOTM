//! Event system.

use std::collections::VecDeque;

/// Event bus
pub struct EventBus {
    events: VecDeque<Box<dyn Event>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            events: VecDeque::new(),
        }
    }

    /// Push an event
    pub fn push<E: Event + 'static>(&mut self, event: E) {
        self.events.push_back(Box::new(event));
    }

    /// Process all events
    pub fn process(&mut self, handler: &mut dyn EventHandler) {
        while let Some(event) = self.events.pop_front() {
            handler.handle_event(event.as_ref());
        }
    }
}

/// Event trait
pub trait Event: Send + Sync {
    fn event_type(&self) -> &str;
}

/// Event handler trait
pub trait EventHandler {
    fn handle_event(&mut self, event: &dyn Event);
}

// Common events
#[derive(Debug, Clone)]
pub struct PlayerMoved {
    pub entity_id: u64,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Event for PlayerMoved {
    fn event_type(&self) -> &str {
        "player_moved"
    }
}

#[derive(Debug, Clone)]
pub struct ChunkLoaded {
    pub x: i32,
    pub y: i32,
}

impl Event for ChunkLoaded {
    fn event_type(&self) -> &str {
        "chunk_loaded"
    }
}
