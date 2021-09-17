//! Extension traits that provide simplified syntax for various operations
//!
//! Import the prelude (`use fj::prelude::*;`) to make these traits available to
//! you.

use crate::{geometry::operations, model};

/// Provides convenient syntax for [`operations::Difference`]
///
/// This trait is implemented for tuples with two entries. The call
/// `(a, b).difference()` will return the difference of `a` and `b`.
pub trait Difference<A, B> {
    fn difference(self) -> operations::Difference<A, B>;
}

impl<A, B> Difference<A, B> for (A, B) {
    fn difference(self) -> operations::Difference<A, B> {
        operations::Difference {
            a: self.0,
            b: self.1,
        }
    }
}

/// Provides convenient syntax for [`operations::Sweep`]
///
/// This trait is implemented for all types. The call `sketch.sweep(distance)`
/// will create a sweep of `sketch` over `distance`.
pub trait Sweep<Sketch> {
    fn sweep(self, distance: f32) -> operations::LinearSweep<Sketch>;
}

impl<Sketch> Sweep<Sketch> for Sketch {
    fn sweep(self, distance: f32) -> operations::LinearSweep<Sketch> {
        operations::LinearSweep {
            sketch: self,
            distance,
        }
    }
}

/// Provides convenient syntax for [`model::WithResolution`]
///
/// This trait is implemented for all types. The call `geometry.resolution(res)`
/// will wrap `geometry` in a `model::WithResolution` struct, which can then be
/// converted into a triangle mesh, using the resolution specified.
pub trait Resolution: Sized {
    fn resolution(self, resolution: f32) -> model::WithResolution<Self> {
        model::WithResolution {
            geometry: self,
            resolution,
        }
    }
}

impl<Geometry> Resolution for Geometry {}
