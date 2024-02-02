//! Main entry point to the `fj-core` API
//!
//! See [`Instance`].

use crate::services::Services;

/// An instance of the Fornjot core
///
/// This is the main entry point to the Fornjot API.
pub struct Instance {
    /// Event-sourced background services
    pub services: Services,
}
