//! Extension traits that provide simplified syntax for various operations
//!
//! Import the prelude (`use fj::prelude::*;`) to make these traits available to
//! you.

use nalgebra::{
    allocator::Allocator, Const, DefaultAllocator, DimNameAdd, DimNameSum, U1,
};

use crate::{
    geometry::operations,
    math::{self, Vector},
    model,
};

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

/// Provides convenient syntax for [`operations::Sweep`]
///
/// This trait is implemented for all types, but most features of the resulting
/// [`operations::Sweep`] will only be available for types that represent
/// shapes.
pub trait Sweep<Path>: Sized {
    /// Create a sweep of `self` over `path`
    fn sweep(self, path: Path) -> operations::Sweep<Self, Path> {
        operations::Sweep { shape: self, path }
    }
}

impl<T, Path> Sweep<Path> for T {}

/// Provides convenient syntax for [`operations::Transform`]
///
/// This trait is implemented for all types, but most features of the resulting
/// [`operations::Transform`] will only be available for types that represent
/// shapes.
pub trait Transform<const D: usize>: Sized
where
    Const<D>: DimNameAdd<U1>,
    DefaultAllocator:
        Allocator<f32, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
{
    /// Transform `self` with `transform`
    fn transform(
        self,
        transform: impl Into<math::Transform<D>>,
    ) -> operations::Transform<Self, D> {
        operations::Transform {
            shape: self,
            transform: transform.into(),
        }
    }
}

impl<T, const D: usize> Transform<D> for T
where
    Const<D>: DimNameAdd<U1>,
    DefaultAllocator:
        Allocator<f32, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
{
}

/// Provides convenient syntax for [`operations::Translate`]
///
/// This trait is implemented for all types, but most features of the resulting
/// [`operations::Translate`] will only be available for types that represent
/// shapes.
pub trait Translate<const D: usize>: Sized {
    /// Translate `self` by `offset`
    fn translate(self, offset: Vector<D>) -> operations::Translate<Self, D>;
}

impl<T, const D: usize> Translate<D> for T {
    fn translate(self, offset: Vector<D>) -> operations::Translate<Self, D> {
        operations::Translate {
            shape: self,
            offset,
        }
    }
}
