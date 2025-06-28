//! Bindings for the xterm.js public API.
//!
//! This module provides Rust bindings for the xterm.js terminal emulator library.
//! The bindings are organized into several submodules for better maintainability:
//!
//! - [`types`] - Basic types, enums, and constants
//! - [`events`] - Event data structures and related types
//! - [`options`] - Terminal configuration and options
//! - [`buffer`] - Buffer-related interfaces and types
//! - [`interfaces`] - Core interfaces like Disposable, Marker, etc.
//! - [`terminal`] - The main Terminal implementation
//!
//! For detailed information about interface mirroring and design patterns,
//! see the original xterm.rs documentation and individual module documentation.

// Re-export all public types from submodules
pub use buffer::*;
pub use events::*;
pub use interfaces::*;
pub use options::*;
pub use terminal::*;
pub use types::*;

pub mod buffer;
pub mod events;
pub mod interfaces;
pub mod options;
pub mod terminal;
pub mod types;
