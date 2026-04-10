//! ECS (Entity Component System) - simplified implementation.

use std::collections::HashMap;

/// Simple ECS world
pub struct EcsWorld {
    entities: HashMap<u64, Entity>,
    next_id: u64,
}

impl EcsWorld {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            next_id: 0,
        }
    }

    /// Create a new entity
    pub fn create_entity(&mut self) -> EntityId {
        let id = EntityId(self.next_id);
        self.next_id += 1;
        self.entities.insert(self.next_id - 1, Entity::new());
        id
    }

    /// Get entity by ID
    pub fn get_entity(&self, id: EntityId) -> Option<&Entity> {
        self.entities.get(&id.0)
    }

    /// Get mutable entity by ID
    pub fn get_entity_mut(&mut self, id: EntityId) -> Option<&mut Entity> {
        self.entities.get_mut(&id.0)
    }
}

impl Default for EcsWorld {
    fn default() -> Self {
        Self::new()
    }
}

/// Entity ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityId(pub u64);

/// Entity with components
#[derive(Debug, Clone)]
pub struct Entity {
    pub position: Option<Position>,
    pub velocity: Option<Velocity>,
    pub rotation: Option<Rotation>,
}

impl Entity {
    pub fn new() -> Self {
        Self {
            position: None,
            velocity: None,
            rotation: None,
        }
    }
}

impl Default for Entity {
    fn default() -> Self {
        Self::new()
    }
}

/// Position component
#[derive(Debug, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Velocity component
#[derive(Debug, Clone)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Rotation component
#[derive(Debug, Clone)]
pub struct Rotation {
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
}
