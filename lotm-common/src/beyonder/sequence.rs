//! Sequence abilities for each pathway.
//!
//! Each pathway has 9 sequences (9 = lowest, 1 = highest).

use serde::{Deserialize, Serialize};

/// Sequence abilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequenceAbility {
    pub name: String,
    pub description: String,
    pub power_level: f32,
    pub madness_increase: f32,
}

/// All sequences for a pathway
pub fn get_sequence_abilities(pathway: &crate::beyonder::Pathway, sequence: u8) -> Vec<SequenceAbility> {
    match pathway {
        crate::beyonder::Pathway::Fool => fool_sequences(sequence),
        crate::beyonder::Pathway::Error => error_sequences(sequence),
        crate::beyonder::Pathway::Door => door_sequences(sequence),
        crate::beyonder::Pathway::RedPriest => red_priest_sequences(sequence),
        crate::beyonder::Pathway::Paragon => paragon_sequences(sequence),
        crate::beyonder::Pathway::Visionary => visionary_sequences(sequence),
        crate::beyonder::Pathway::Farmer => farmer_sequences(sequence),
        crate::beyonder::Pathway::Mother => mother_sequences(sequence),
        crate::beyonder::Pathway::HangedMan => hanged_man_sequences(sequence),
    }
}

// Fool Pathway Sequences
fn fool_sequences(sequence: u8) -> Vec<SequenceAbility> {
    match sequence {
        9 => vec![
            SequenceAbility {
                name: "Spirit Vision".into(),
                description: "See spirits and supernatural entities".into(),
                power_level: 1.0,
                madness_increase: 0.05,
            },
            SequenceAbility {
                name: "Divination".into(),
                description: "Perform basic divination rituals".into(),
                power_level: 1.2,
                madness_increase: 0.05,
            },
        ],
        8 => vec![
            SequenceAbility {
                name: "Clown".into(),
                description: "Enhanced agility and balance".into(),
                power_level: 2.0,
                madness_increase: 0.08,
            },
        ],
        7 => vec![
            SequenceAbility {
                name: "Fire Dancer".into(),
                description: "Control flames and perform fire dances".into(),
                power_level: 3.0,
                madness_increase: 0.1,
            },
        ],
        5 => vec![
            SequenceAbility {
                name: "Spirit Guide".into(),
                description: "Communicate with and command spirits".into(),
                power_level: 5.0,
                madness_increase: 0.15,
            },
        ],
        1 => vec![
            SequenceAbility {
                name: "The Fool".into(),
                description: "Manipulate fate, resurrect, transform".into(),
                power_level: 10.0,
                madness_increase: 0.5,
            },
        ],
        _ => vec![],
    }
}

// Error Pathway Sequences
fn error_sequences(sequence: u8) -> Vec<SequenceAbility> {
    match sequence {
        9 => vec![
            SequenceAbility {
                name: "Scholar of Yore".into(),
                description: "Access knowledge from the past".into(),
                power_level: 1.0,
                madness_increase: 0.05,
            },
        ],
        6 => vec![
            SequenceAbility {
                name: "Decipherer".into(),
                description: "Decode any language or cipher".into(),
                power_level: 4.0,
                madness_increase: 0.12,
            },
        ],
        4 => vec![
            SequenceAbility {
                name: "Time Traveler".into(),
                description: "Manipulate time in limited ways".into(),
                power_level: 7.0,
                madness_increase: 0.25,
            },
        ],
        1 => vec![
            SequenceAbility {
                name: "Error".into(),
                description: "Exploit the errors in reality itself".into(),
                power_level: 10.0,
                madness_increase: 0.5,
            },
        ],
        _ => vec![],
    }
}

// Door Pathway Sequences
fn door_sequences(sequence: u8) -> Vec<SequenceAbility> {
    match sequence {
        9 => vec![
            SequenceAbility {
                name: "Apprentice".into(),
                description: "Basic magical abilities".into(),
                power_level: 1.0,
                madness_increase: 0.05,
            },
        ],
        7 => vec![
            SequenceAbility {
                name: "Teleportation".into(),
                description: "Short-range teleportation".into(),
                power_level: 3.5,
                madness_increase: 0.1,
            },
        ],
        5 => vec![
            SequenceAbility {
                name: "Secret Traveler".into(),
                description: "Travel through barriers and sealed spaces".into(),
                power_level: 5.5,
                madness_increase: 0.15,
            },
        ],
        1 => vec![
            SequenceAbility {
                name: "Door".into(),
                description: "Open doors to anywhere, absolute freedom".into(),
                power_level: 10.0,
                madness_increase: 0.5,
            },
        ],
        _ => vec![],
    }
}

// Red Priest Pathway
fn red_priest_sequences(sequence: u8) -> Vec<SequenceAbility> {
    match sequence {
        9 => vec![
            SequenceAbility {
                name: "Spectator".into(),
                description: "Enhanced perception and observation".into(),
                power_level: 1.0,
                madness_increase: 0.05,
            },
        ],
        6 => vec![
            SequenceAbility {
                name: "Pyromancer".into(),
                description: "Control and create massive flames".into(),
                power_level: 4.5,
                madness_increase: 0.15,
            },
        ],
        1 => vec![
            SequenceAbility {
                name: "Red Priest".into(),
                description: "Ultimate destruction through fire and war".into(),
                power_level: 10.0,
                madness_increase: 0.5,
            },
        ],
        _ => vec![],
    }
}

// Placeholder implementations for other pathways
fn paragon_sequences(sequence: u8) -> Vec<SequenceAbility> {
    match sequence {
        1 => vec![SequenceAbility {
            name: "Black Emperor".into(),
            description: "Absolute authority and order".into(),
            power_level: 10.0,
            madness_increase: 0.5,
        }],
        _ => vec![],
    }
}

fn visionary_sequences(sequence: u8) -> Vec<SequenceAbility> {
    match sequence {
        1 => vec![SequenceAbility {
            name: "White Tower".into(),
            description: "Ultimate wisdom and creation".into(),
            power_level: 10.0,
            madness_increase: 0.5,
        }],
        _ => vec![],
    }
}

fn farmer_sequences(sequence: u8) -> Vec<SequenceAbility> {
    match sequence {
        1 => vec![SequenceAbility {
            name: "Farmer".into(),
            description: "Control over life and death".into(),
            power_level: 10.0,
            madness_increase: 0.5,
        }],
        _ => vec![],
    }
}

fn mother_sequences(sequence: u8) -> Vec<SequenceAbility> {
    match sequence {
        1 => vec![SequenceAbility {
            name: "Mother".into(),
            description: "Ultimate fertility and corruption".into(),
            power_level: 10.0,
            madness_increase: 0.5,
        }],
        _ => vec![],
    }
}

fn hanged_man_sequences(sequence: u8) -> Vec<SequenceAbility> {
    match sequence {
        1 => vec![SequenceAbility {
            name: "Hanged Man".into(),
            description: "Sacrifice and redemption".into(),
            power_level: 10.0,
            madness_increase: 0.5,
        }],
        _ => vec![],
    }
}
