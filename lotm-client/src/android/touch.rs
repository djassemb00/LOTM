//! Touch input handling for mobile devices.

use winit::event::TouchPhase;

/// Touch state
#[derive(Debug, Clone)]
pub struct TouchState {
    pub id: u64,
    pub position: (f64, f64),
    pub previous_position: (f64, f64),
    pub phase: TouchPhase,
}

impl TouchState {
    pub fn new(id: u64, position: (f64, f64), phase: TouchPhase) -> Self {
        Self {
            id,
            position,
            previous_position: position,
            phase,
        }
    }

    /// Calculate delta movement
    pub fn delta(&self) -> (f64, f64) {
        (
            self.position.0 - self.previous_position.0,
            self.position.1 - self.previous_position.1,
        )
    }

    /// Check if touch just started
    pub fn is_start(&self) -> bool {
        matches!(self.phase, TouchPhase::Started)
    }

    /// Check if touch just ended
    pub fn is_end(&self) -> bool {
        matches!(self.phase, TouchPhase::Ended | TouchPhase::Cancelled)
    }
}

/// Touch gesture recognizer
pub struct GestureRecognizer {
    touches: Vec<TouchState>,
    pinch_start_distance: Option<f64>,
}

impl GestureRecognizer {
    pub fn new() -> Self {
        Self {
            touches: Vec::new(),
            pinch_start_distance: None,
        }
    }

    /// Update touch state
    pub fn update_touch(&mut self, touch: TouchState) -> Option<Gesture> {
        match touch.phase {
            TouchPhase::Started => {
                self.touches.push(touch.clone());
                if self.touches.len() == 2 {
                    // Start of pinch gesture
                    self.pinch_start_distance = Some(self.pinch_distance());
                }
                Some(Gesture::TouchStart(touch))
            }
            TouchPhase::Moved => {
                if let Some(existing) = self.touches.iter_mut().find(|t| t.id == touch.id) {
                    existing.previous_position = existing.position;
                    existing.position = touch.position;
                }

                if self.touches.len() == 2 {
                    // Pinch gesture
                    let current_distance = self.pinch_distance();
                    if let Some(start_distance) = self.pinch_start_distance {
                        let scale = current_distance / start_distance;
                        return Some(Gesture::Pinch { scale });
                    }
                }

                Some(Gesture::TouchMove(touch))
            }
            TouchPhase::Ended | TouchPhase::Cancelled => {
                self.touches.retain(|t| t.id != touch.id);
                self.pinch_start_distance = None;
                Some(Gesture::TouchEnd(touch))
            }
        }
    }

    fn pinch_distance(&self) -> f64 {
        if self.touches.len() >= 2 {
            let t0 = &self.touches[0];
            let t1 = &self.touches[1];
            let dx = t0.position.0 - t1.position.0;
            let dy = t0.position.1 - t1.position.1;
            (dx * dx + dy * dy).sqrt()
        } else {
            0.0
        }
    }
}

/// Recognized gesture
pub enum Gesture {
    TouchStart(TouchState),
    TouchMove(TouchState),
    TouchEnd(TouchState),
    Pinch { scale: f64 },
    Swipe { dx: f64, dy: f64 },
    Tap { position: (f64, f64) },
}
