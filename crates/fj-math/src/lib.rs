//! # Fornjot Math Library
//!
//! [Fornjot] is an early-stage b-rep CAD kernel written in Rust. The kernel is
//! split into multiple libraries that can be used semi-independently, and this
//! is one of those.
//!
//! This library provides basic math types for Fornjot. It is built on
//! [nalgebra] and [Parry], but provides an interface that is specifically
//! tailored to the needs of Fornjot.
//!
//!
//! ## Failing [`From`]/[`Into`] implementations
//!
//! Please note that any [`From`]/[`Into`] implementation that convert floating
//! point numbers into [`Scalar`] can panic. These conversions call
//! [`Scalar::from_f64`] internally and panic under the same conditions. This
//! affects [`Scalar`] itself, but also any other types in this crate that
//! provide conversions from types that involve `f64`.
//!
//! This explicitly goes against the mandate of [`From`]/[`Into`], whose
//! documentation states that implementations must not fail. This is a
//! deliberate design decision. The intended use case of `Scalar` is math code
//! that considers NaN results a bug, not a recoverable error.
//!
//! For this use case, having easy conversions available is an advantage, and
//! explicit `unwrap`/`expect` calls would add nothing. In addition, the
//! [`From`]/[`Into`] documentation fails to provide any reasons for its
//! mandate.
//!
//! [Fornjot]: https://www.fornjot.app/
//! [nalgebra]: https://nalgebra.org/
//! [Parry]: https://www.parry.rs/

mod aabb;
mod arc;
mod bivector;
mod circle;
mod coordinates;
mod line;
mod line_segment;
mod point;
mod poly_chain;
mod scalar;
mod transform;
mod triangle;
mod vector;

pub use self::{
    aabb::Aabb,
    arc::Arc,
    bivector::Bivector,
    circle::Circle,
    coordinates::{Uv, Xyz, T},
    line::Line,
    line_segment::LineSegment,
    point::Point,
    poly_chain::PolyChain,
    scalar::{Scalar, Sign},
    transform::Transform,
    triangle::{Triangle, Winding},
    vector::Vector,
};
