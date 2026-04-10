//! Input handling - keyboard, mouse, touch.

use winit::event::{ElementState, KeyEvent, MouseButton};
use winit::keyboard::KeyCode;
use std::collections::HashSet;

/// Input state
pub struct InputState {
    pub keys_pressed: HashSet<KeyCode>,
    pub mouse_position: (f64, f64),
    pub mouse_buttons: HashSet<MouseButton>,
    pub touch_input: Option<TouchInput>,
}

/// Touch input for mobile
pub struct TouchInput {
    pub position: (f64, f64),
    pub delta: (f64, f64),
    pub pinch_scale: f32,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            keys_pressed: HashSet::new(),
            mouse_position: (0.0, 0.0),
            mouse_buttons: HashSet::new(),
            touch_input: None,
        }
    }

    pub fn handle_key(&mut self, key: &KeyCode, state: ElementState) {
        match state {
            ElementState::Pressed => {
                self.keys_pressed.insert(*key);
            }
            ElementState::Released => {
                self.keys_pressed.remove(key);
            }
        }
    }

    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.keys_pressed.contains(&key)
    }

    /// Check for camera mode switch (C key)
    pub fn should_cycle_camera(&self) -> bool {
        self.keys_pressed.contains(&KeyCode::KeyC)
    }
}
