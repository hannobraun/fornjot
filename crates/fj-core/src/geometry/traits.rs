//! # Traits that abstract over curve or surface geometry
//!
//! Fornjot's geometry is built on the concept of a uniform representation:
//! Polylines to represent curves and triangle meshes to represent surfaces. The
//! traits in this module provide the interface between this uniform
//! representation and specific geometry code.
//!
//! ## Implementation Note
//!
//! As of this writing, the transition from the previous, more limited, geometry
//! system to the new one based on uniform representation is still ongoing. As a
//! result of that, this module might still be incomplete.
