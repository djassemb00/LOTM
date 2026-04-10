//! Virtual joystick for mobile movement control.

/// Virtual joystick state
pub struct VirtualJoystick {
    pub center: (f32, f32),
    pub current: (f32, f32),
    pub radius: f32,
    pub active: bool,
    pub touch_id: Option<u64>,
}

impl VirtualJoystick {
    pub fn new(center_x: f32, center_y: f32, radius: f32) -> Self {
        Self {
            center: (center_x, center_y),
            current: (center_x, center_y),
            radius,
            active: false,
            touch_id: None,
        }
    }

    /// Handle touch start
    pub fn on_touch_start(&mut self, touch_id: u64, x: f32, y: f32) -> bool {
        // Check if touch is within joystick area
        let dx = x - self.center.0;
        let dy = y - self.center.1;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance < self.radius * 2.0 {
            self.active = true;
            self.touch_id = Some(touch_id);
            self.update_position(x, y);
            true
        } else {
            false
        }
    }

    /// Handle touch move
    pub fn on_touch_move(&mut self, touch_id: u64, x: f32, y: f32) -> bool {
        if self.touch_id == Some(touch_id) && self.active {
            self.update_position(x, y);
            true
        } else {
            false
        }
    }

    /// Handle touch end
    pub fn on_touch_end(&mut self, touch_id: u64) {
        if self.touch_id == Some(touch_id) {
            self.active = false;
            self.touch_id = None;
            self.current = self.center;
        }
    }

    fn update_position(&mut self, x: f32, y: f32) {
        let mut dx = x - self.center.0;
        let mut dy = y - self.center.1;
        let distance = (dx * dx + dy * dy).sqrt();

        // Clamp to radius
        if distance > self.radius {
            dx = dx / distance * self.radius;
            dy = dy / distance * self.radius;
        }

        self.current = (self.center.0 + dx, self.center.1 + dy);
    }

    /// Get normalized input (-1.0 to 1.0)
    pub fn input(&self) -> (f32, f32) {
        if !self.active {
            return (0.0, 0.0);
        }

        let dx = (self.current.0 - self.center.0) / self.radius;
        let dy = (self.current.1 - self.center.1) / self.radius;

        (dx.clamp(-1.0, 1.0), dy.clamp(-1.0, 1.0))
    }
}

/// Dual virtual joysticks (left for movement, right for camera)
pub struct DualJoysticks {
    pub left: VirtualJoystick,
    pub right: VirtualJoystick,
}

impl DualJoysticks {
    pub fn new(screen_width: f32, screen_height: f32) -> Self {
        let joystick_radius = 80.0;

        Self {
            left: VirtualJoystick::new(
                screen_width * 0.2,
                screen_height * 0.8,
                joystick_radius,
            ),
            right: VirtualJoystick::new(
                screen_width * 0.8,
                screen_height * 0.8,
                joystick_radius,
            ),
        }
    }

    /// Handle touch event
    pub fn handle_touch(&mut self, touch_id: u64, x: f32, y: f32, phase: winit::event::TouchPhase) {
        match phase {
            winit::event::TouchPhase::Started => {
                // Try left joystick first
                if !self.left.on_touch_start(touch_id, x, y) {
                    // Try right joystick
                    self.right.on_touch_start(touch_id, x, y);
                }
            }
            winit::event::TouchPhase::Moved => {
                self.left.on_touch_move(touch_id, x, y);
                self.right.on_touch_move(touch_id, x, y);
            }
            winit::event::TouchPhase::Ended | winit::event::TouchPhase::Cancelled => {
                self.left.on_touch_end(touch_id);
                self.right.on_touch_end(touch_id);
            }
        }
    }

    /// Get movement input
    pub fn movement_input(&self) -> (f32, f32) {
        self.left.input()
    }

    /// Get camera input
    pub fn camera_input(&self) -> (f32, f32) {
        self.right.input()
    }
}
