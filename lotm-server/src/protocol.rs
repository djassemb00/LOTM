//! Network protocol definitions for LOTM multiplayer.
//!
//! This module defines all network messages used between client and server.

use serde::{Deserialize, Serialize};

/// Network messages from client to server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    /// Player movement
    PlayerMove {
        entity_id: u64,
        position: (f32, f32, f32),
        rotation: (f32, f32),
    },
    /// Player action
    PlayerAction {
        entity_id: u64,
        action: PlayerAction,
    },
    /// Chat message
    Chat {
        message: String,
    },
    /// Request chunk
    RequestChunk {
        x: i32,
        y: i32,
    },
    /// Beyonder ability use
    UseAbility {
        entity_id: u64,
        pathway: u8,
        sequence: u8,
        target: Option<(f32, f32, f32)>,
    },
    /// Potion consumption
    ConsumePotion {
        entity_id: u64,
        pathway: u8,
        sequence: u8,
    },
}

/// Network messages from server to client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    /// Welcome message on connect
    Welcome {
        entity_id: u64,
        seed: u32,
    },
    /// Chunk data
    ChunkData {
        x: i32,
        y: i32,
        chunk: lotm_common::types::Chunk,
    },
    /// Entity update
    EntityUpdate {
        entities: Vec<EntityInfo>,
    },
    /// Player joined
    PlayerJoined {
        entity_id: u64,
        name: String,
        position: (f32, f32, f32),
    },
    /// Player left
    PlayerLeft {
        entity_id: u64,
    },
    /// Chat message
    Chat {
        sender: String,
        message: String,
    },
    /// World event
    WorldEvent {
        event: WorldEvent,
    },
    /// Beyonder advancement result
    AdvancementResult {
        success: bool,
        madness: f32,
    },
}

/// Player actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlayerAction {
    Attack,
    Interact,
    Jump,
    UseItem,
    OpenInventory,
}

/// Entity information for synchronization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityInfo {
    pub id: u64,
    pub entity_type: EntityType,
    pub position: (f32, f32, f32),
    pub rotation: (f32, f32),
    pub velocity: (f32, f32, f32),
}

/// Entity types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntityType {
    Player {
        name: String,
        pathway: Option<u8>,
        sequence: Option<u8>,
    },
    NPC {
        npc_type: NPCType,
        name: String,
    },
    Monster {
        monster_type: MonsterType,
        level: u8,
    },
    Item {
        item_id: String,
        quantity: u32,
    },
}

/// NPC types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NPCType {
    Merchant,
    QuestGiver,
    ChurchMember,
    BeyonderTrainer,
}

/// Monster types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MonsterType {
    CorruptedCreature,
    FogEntity,
    AncientMonster,
    SealedArtifact,
}

/// World events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorldEvent {
    /// Corruption surge in an area
    CorruptionSurge {
        position: (i32, i32),
        radius: f32,
        duration: f32,
    },
    /// Beyonder gathering
    BeyonderGathering {
        position: (i32, i32),
        pathway: u8,
    },
    /// Sealed artifact appeared
    SealedArtifactAppeared {
        position: (i32, i32, i32),
        artifact_name: String,
    },
    /// Church ritual started
    ChurchRitual {
        church_name: String,
        benefits: Vec<String>,
    },
}
