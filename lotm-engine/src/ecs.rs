//! ECS (Entity Component System) integration.

use specs::*;

/// World ECS dispatcher
pub struct EcsWorld {
    pub world: World,
}

impl EcsWorld {
    pub fn new() -> Self {
        let mut world = World::new();

        // Register common components
        world.register::<Position>();
        world.register::<Velocity>();
        world.register::<Rotation>();

        Self { world }
    }
}

/// Position component
#[derive(Debug, Clone, Component)]
#[storage(VecStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Velocity component
#[derive(Debug, Clone, Component)]
#[storage(VecStorage)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Rotation component
#[derive(Debug, Clone, Component)]
#[storage(VecStorage)]
pub struct Rotation {
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
}
