//! # New Fornjot Core Code
//!
//! The code in this module represents a new approach that is going to replace
//! the rest of this crate. It resulted from a
//! [series of experiments][experiments], the [latest of which][2025-12-03]
//! turned out successful.
//!
//! This is going to be a piecemeal transition, hence the new module. In
//! addition, the migration of the experiment into this module is also going to
//! be piecemeal, as the code is going to require clean-up and documentation.
//! What you see here right now, most likely isn't everything yet.
//!
//! [experiments]: https://github.com/hannobraun/fornjot/tree/main/experiments
//! [2025-12-03]: https://github.com/hannobraun/fornjot/tree/main/experiments/2025-12-03

pub mod approx;
pub mod geometry;
#[allow(missing_docs)] // temporary, during transition
pub mod operations;
pub mod topology;
