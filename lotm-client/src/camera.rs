//! Camera system with multiple view modes.

use glam::Vec3;

/// Camera modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CameraMode {
    /// First person view
    FirstPerson,
    /// Third person view (over shoulder)
    ThirdPerson,
    /// Top-down view (RTS style)
    TopDown,
    /// Free camera (spectator)
    Free,
}

/// Camera state
pub struct Camera {
    pub position: Vec3,
    pub rotation: Vec3,
    pub mode: CameraMode,
    pub fov: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            position: Vec3::new(0.0, 50.0, 0.0),
            rotation: Vec3::new(-45.0, 0.0, 0.0),
            mode: CameraMode::TopDown,
            fov: 60.0,
        }
    }

    /// Switch to next camera mode
    pub fn cycle_mode(&mut self) {
        self.mode = match self.mode {
            CameraMode::FirstPerson => CameraMode::ThirdPerson,
            CameraMode::ThirdPerson => CameraMode::TopDown,
            CameraMode::TopDown => CameraMode::Free,
            CameraMode::Free => CameraMode::FirstPerson,
        };

        tracing::info!("Camera mode: {:?}", self.mode);
    }

    /// Update camera based on mode
    pub fn update(&mut self, target_position: Vec3) {
        match self.mode {
            CameraMode::FirstPerson => {
                // Camera at player position
                self.position = target_position + Vec3::new(0.0, 1.7, 0.0);
            }
            CameraMode::ThirdPerson => {
                // Camera behind player
                let offset = Vec3::new(0.0, 3.0, 8.0);
                self.position = target_position + offset;
            }
            CameraMode::TopDown => {
                // Camera above looking down
                self.position = target_position + Vec3::new(0.0, 50.0, 0.0);
            }
            CameraMode::Free => {
                // Free camera - no automatic positioning
            }
        }
    }
}
