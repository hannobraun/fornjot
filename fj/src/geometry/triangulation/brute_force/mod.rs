//! Basic triangulation algorithm
//!
//! This is a brute-force algorithm that I've come up with myself, and that's
//! designed to work with exactly the polygons I need it for right now, and not
//! more.

pub mod algorithm;

pub use self::algorithm::triangulate;
