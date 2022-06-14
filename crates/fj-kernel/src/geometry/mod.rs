//! Geometry objects
//!
//! Simplifying a bit, geometry is responsible for where things are, but now how
//! they are related. The types in this module are referred to by the types in
//! [`crate::topology`], which are responsible for defining how objects are
//! related.

mod points;

pub use self::points::Point;
