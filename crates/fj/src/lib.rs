//! # Fornjot
//!
//! [Fornjot] is an early-stage b-rep CAD kernel written in Rust. The kernel is
//! split into multiple libraries that can be used semi-independently, and this
//! is one of those.
//!
//! This crate serves as a convenient entryway to Fornjot, re-exporting all
//! crates that make up Fornjot.
//!
//! [Fornjot]: https://www.fornjot.app/

mod cli;
mod debug;
mod instance;
mod process_model;

pub use self::{
    cli::Arguments,
    debug::{DEBUG_WINDOW, DebugWindow},
    instance::{Error, Instance, Result},
    process_model::process_model,
};

pub use fj_core as core;
pub use fj_export as export;
pub use fj_viewer as viewer;
