//! Android support for LOTM
//!
//! This module provides Android-specific integration including:
//! - Touch input handling
//! - Android activity lifecycle
//! - Virtual joystick for movement
//! - Touch gestures for camera control

#[cfg(target_os = "android")]
pub mod activity;
pub mod touch;
pub mod virtual_joystick;
