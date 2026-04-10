//! Time management.

use std::time::{Duration, Instant};

/// Game time
pub struct Time {
    pub delta_time: Duration,
    pub total_time: Duration,
    pub fps: f32,
    last_frame: Instant,
    frame_count: u32,
    fps_timer: Instant,
}

impl Time {
    pub fn new() -> Self {
        Self {
            delta_time: Duration::ZERO,
            total_time: Duration::ZERO,
            fps: 0.0,
            last_frame: Instant::now(),
            frame_count: 0,
            fps_timer: Instant::now(),
        }
    }

    /// Update time, call once per frame
    pub fn update(&mut self) {
        let now = Instant::now();
        self.delta_time = now - self.last_frame;
        self.total_time += self.delta_time;
        self.last_frame = now;

        self.frame_count += 1;

        if self.fps_timer.elapsed() >= Duration::from_secs(1) {
            self.fps = self.frame_count as f32 / self.fps_timer.elapsed().as_secs_f32();
            self.frame_count = 0;
            self.fps_timer = Instant::now();
        }
    }
}
