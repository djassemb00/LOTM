//! Madness system - the cost of Beyonder power.
//!
//! As Beyonders advance, they accumulate madness.
//! Too much madness leads to loss of control and death.

use serde::{Deserialize, Serialize};

/// Madness state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Madness {
    /// Current madness level (0.0 - 1.0)
    pub level: f32,
    /// Whispers from the abyss
    pub whispers: Vec<String>,
    /// Loss of control threshold
    pub loss_threshold: f32,
}

impl Madness {
    pub fn new() -> Self {
        Self {
            level: 0.0,
            whispers: Vec::new(),
            loss_threshold: 0.8,
        }
    }

    /// Add madness
    pub fn add(&mut self, amount: f32) {
        self.level = (self.level + amount).min(1.0);

        // Add whispers at high madness
        if self.level > 0.5 && rand::random::<f32>() < self.level * 0.3 {
            self.whispers.push(Self::random_whisper());
        }
    }

    /// Reduce madness through meditation
    pub fn meditate(&mut self, duration_seconds: f32) {
        let reduction = duration_seconds * 0.001;
        self.level = (self.level - reduction).max(0.0);

        // Clear some whispers
        if self.level < 0.5 {
            self.whispers.clear();
        }
    }

    /// Check if lost control
    pub fn has_lost_control(&self) -> bool {
        self.level >= self.loss_threshold
    }

    /// Get madness state description
    pub fn state_description(&self) -> &'static str {
        match self.level {
            0.0..=0.2 => "Sane - Clear minded",
            0.2..=0.4 => "Uneasy - Minor hallucinations",
            0.4..=0.6 => "Disturbed - Frequent whispers",
            0.6..=0.8 => "Unstable - Reality warping",
            0.8..=1.0 => "Critical - Loss of control imminent",
            _ => "Lost Control - Beyond redemption",
        }
    }

    fn random_whisper() -> String {
        let whispers = [
            "The Fool... he sees all...",
            "Error in the fabric of reality...",
            "Open the door... open it...",
            "Fire consumes everything...",
            "Order must be maintained...",
            "Knowledge is power... and madness...",
            "Life and death... a cycle...",
            "The Mother hungers...",
            "Sacrifice... sacrifice...",
            "They are watching...",
            "The gray fog calls to you...",
            "Above the gray fog sits a figure...",
        ];
        whispers[rand::random::<usize>() % whispers.len()].to_string()
    }
}

impl Default for Madness {
    fn default() -> Self {
        Self::new()
    }
}
