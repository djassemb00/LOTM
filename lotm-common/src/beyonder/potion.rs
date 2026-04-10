//! Potion system for Beyonder advancement.

use serde::{Deserialize, Serialize};

/// Potion ingredients and effects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Potion {
    pub pathway: crate::beyonder::Pathway,
    pub sequence: u8,
    pub name: String,
    pub ingredients: Vec<Ingredient>,
    pub success_rate: f32,
    pub madness_on_fail: f32,
}

/// Potion ingredient
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ingredient {
    pub name: String,
    pub quantity: u32,
    pub rarity: Rarity,
}

/// Ingredient rarity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

impl Potion {
    /// Create a potion for a specific pathway and sequence
    pub fn new(pathway: crate::beyonder::Pathway, sequence: u8) -> Self {
        let ingredients = Self::generate_ingredients(pathway, sequence);
        let success_rate = Self::calculate_success_rate(sequence);

        Self {
            pathway,
            sequence,
            name: format!("{} Sequence {} Potion", pathway.chinese_name(), sequence),
            ingredients,
            success_rate,
            madness_on_fail: (10 - sequence) as f32 * 0.05,
        }
    }

    fn generate_ingredients(pathway: crate::beyonder::Pathway, sequence: u8) -> Vec<Ingredient> {
        let mut ingredients = Vec::new();

        // Base ingredients based on pathway
        match pathway {
            crate::beyonder::Pathway::Fool => {
                ingredients.push(Ingredient {
                    name: "Spirit Essence".into(),
                    quantity: 3,
                    rarity: Rarity::Rare,
                });
                ingredients.push(Ingredient {
                    name: "Fate Thread".into(),
                    quantity: 1,
                    rarity: Rarity::Epic,
                });
            }
            crate::beyonder::Pathway::Error => {
                ingredients.push(Ingredient {
                    name: "Time Fragment".into(),
                    quantity: 2,
                    rarity: Rarity::Epic,
                });
                ingredients.push(Ingredient {
                    name: "Deception Powder".into(),
                    quantity: 5,
                    rarity: Rarity::Uncommon,
                });
            }
            crate::beyonder::Pathway::Door => {
                ingredients.push(Ingredient {
                    name: "Space Crystal".into(),
                    quantity: 1,
                    rarity: Rarity::Rare,
                });
                ingredients.push(Ingredient {
                    name: "Portal Dust".into(),
                    quantity: 10,
                    rarity: Rarity::Common,
                });
            }
            _ => {
                ingredients.push(Ingredient {
                    name: "Base Material".into(),
                    quantity: 5,
                    rarity: Rarity::Common,
                });
            }
        }

        // Higher sequences need rarer ingredients
        if sequence <= 5 {
            ingredients.push(Ingredient {
                name: "Divine Blood".into(),
                quantity: 1,
                rarity: Rarity::Legendary,
            });
        }

        ingredients
    }

    fn calculate_success_rate(sequence: u8) -> f32 {
        // Lower sequence numbers = higher difficulty = lower success rate
        match sequence {
            9..=7 => 0.9,
            6..=4 => 0.7,
            3..=2 => 0.5,
            1 => 0.3,
            _ => 1.0,
        }
    }

    /// Consume the potion
    pub fn consume(&self) -> PotionResult {
        use rand::Rng;
        let mut rng = rand::rng();

        let roll: f32 = rng.random();

        if roll < self.success_rate {
            PotionResult::Success
        } else {
            PotionResult::Failure {
                madness_increase: self.madness_on_fail,
            }
        }
    }
}

/// Result of consuming a potion
#[derive(Debug, Clone)]
pub enum PotionResult {
    Success,
    Failure { madness_increase: f32 },
}
