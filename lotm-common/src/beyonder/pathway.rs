//! Beyonder Pathways - the 9 divine paths to godhood.

use serde::{Deserialize, Serialize};
use strum::EnumIter;

/// The 9 Beyonder Pathways
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter)]
pub enum Pathway {
    /// 愚者 - The Fool
    /// Pathway of Mystery, Fate, and Transformation
    Fool,
    /// 错误 - Error
    /// Pathway of Time, Space, and Deception
    Error,
    /// 门 - Door
    /// Pathway of Travel, Knowledge, and Freedom
    Door,
    /// 红祭司 - Red Priest
    /// Pathway of War, Fire, and Destruction
    RedPriest,
    /// 黑皇帝 - Black Emperor
    /// Pathway of Order, Law, and Authority
    Paragon,
    /// 白塔 - White Tower
    /// Pathway of Wisdom, Creation, and Inspiration
    Visionary,
    /// 农民 - Farmer
    /// Pathway of Life, Death, and Harvest
    Farmer,
    /// 母亲 - Mother
    /// Pathway of Fertility, Nature, and Corruption
    Mother,
    /// 倒吊人 - Hanged Man
    /// Pathway of Sacrifice, Pain, and Redemption
    HangedMan,
}

impl Pathway {
    /// Get the Chinese name
    pub fn chinese_name(&self) -> &'static str {
        match self {
            Self::Fool => "愚者",
            Self::Error => "错误",
            Self::Door => "门",
            Self::RedPriest => "红祭司",
            Self::Paragon => "黑皇帝",
            Self::Visionary => "白塔",
            Self::Farmer => "农民",
            Self::Mother => "母亲",
            Self::HangedMan => "倒吊人",
        }
    }

    /// Get the pathway description
    pub fn description(&self) -> &'static str {
        match self {
            Self::Fool => "Mystery, Fate, and Transformation",
            Self::Error => "Time, Space, and Deception",
            Self::Door => "Travel, Knowledge, and Freedom",
            Self::RedPriest => "War, Fire, and Destruction",
            Self::Paragon => "Order, Law, and Authority",
            Self::Visionary => "Wisdom, Creation, and Inspiration",
            Self::Farmer => "Life, Death, and Harvest",
            Self::Mother => "Fertility, Nature, and Corruption",
            Self::HangedMan => "Sacrifice, Pain, and Redemption",
        }
    }

    /// Get the corresponding deity
    pub fn deity(&self) -> &'static str {
        match self {
            Self::Fool => "The Fool / Klein Moretti",
            Self::Error => "Amon / Error",
            Self::Door => "Door / Abraham Family",
            Self::RedPriest => "Ancient Sun God / Red Priest",
            Self::Paragon => "Black Emperor / Roselle Gustav",
            Self::Visionary => "White Tower / Gustav Family",
            Self::Farmer => "Earth Mother / Farmer",
            Self::Mother => "Mother Tree of Desire / Mother",
            Self::HangedMan => "Hanged Man / Adam",
        }
    }

    /// Get all pathways
    pub fn all() -> &'static [Self] {
        &[
            Self::Fool,
            Self::Error,
            Self::Door,
            Self::RedPriest,
            Self::Paragon,
            Self::Visionary,
            Self::Farmer,
            Self::Mother,
            Self::HangedMan,
        ]
    }
}

/// Beyonder characteristics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeyonderInfo {
    pub pathway: Pathway,
    pub sequence: u8, // 1-9 (9 is lowest, 1 is highest)
    pub name: String,
    pub madness: f32, // 0.0 - 1.0
    pub potions_consumed: u32,
}

impl BeyonderInfo {
    pub fn new(pathway: Pathway, sequence: u8, name: String) -> Self {
        Self {
            pathway,
            sequence: sequence.clamp(1, 9),
            name,
            madness: 0.0,
            potions_consumed: 0,
        }
    }

    /// Check if can advance to next sequence
    pub fn can_advance(&self) -> bool {
        self.sequence > 1 && self.madness < 0.5 && self.potions_consumed >= 1
    }

    /// Advance to next sequence
    pub fn advance(&mut self) -> Result<(), String> {
        if !self.can_advance() {
            return Err("Cannot advance: requirements not met".to_string());
        }
        self.sequence -= 1;
        self.potions_consumed = 0;
        Ok(())
    }
}
