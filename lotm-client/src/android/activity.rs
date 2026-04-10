//! Android Activity handling
//!
//! Handles Android application lifecycle events.

#[cfg(target_os = "android")]
use winit::platform::android::activity::AndroidApp;

/// Android activity manager
#[cfg(target_os = "android")]
pub struct AndroidActivity {
    app: AndroidApp,
    resumed: bool,
    focused: bool,
}

#[cfg(target_os = "android")]
impl AndroidActivity {
    pub fn new(app: AndroidApp) -> Self {
        tracing::info!("Android activity created");
        Self {
            app,
            resumed: false,
            focused: false,
        }
    }

    /// Handle resume event
    pub fn on_resume(&mut self) {
        self.resumed = true;
        tracing::info!("Android app resumed");
    }

    /// Handle pause event
    pub fn on_pause(&mut self) {
        self.resumed = false;
        tracing::info!("Android app paused");
    }

    /// Handle destroy event
    pub fn on_destroy(&mut self) {
        tracing::info!("Android app destroyed");
    }

    /// Check if app is active
    pub fn is_active(&self) -> bool {
        self.resumed && self.focused
    }

    /// Get Android app reference
    pub fn app(&self) -> &AndroidApp {
        &self.app
    }
}

/// Initialize Android support
#[cfg(target_os = "android")]
pub fn init_android(app: AndroidApp) -> AndroidActivity {
    // Initialize Android logging
    android_logger::init_once(
        android_logger::Config::default()
            .with_min_level(log::Level::Info)
            .with_tag("LOTM"),
    );

    tracing::info!("LOTM Android initializing...");

    AndroidActivity::new(app)
}
