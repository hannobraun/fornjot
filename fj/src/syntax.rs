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
/// This trait is implemented for all types. The call `shape.sweep(distance)`
/// will create a sweep of `shape` over `distance`.
pub trait Sweep<Path>: Sized {
    fn sweep(self, path: Path) -> operations::Sweep<Self, Path>;
}

impl<T, Path> Sweep<Path> for T {
    fn sweep(self, path: Path) -> operations::Sweep<Self, Path> {
        operations::Sweep { shape: self, path }
    }
}

/// Provides convenient syntax for [`operations::Transform`]
///
/// This trait is implemented for all types. The call
/// `shape.transform(transform)` will transform `shape` using `transform`.
pub trait Transform<T, const D: usize>: Sized
where
    Const<D>: DimNameAdd<U1>,
    DefaultAllocator:
        Allocator<f32, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
{
    fn transform(
        self,
        transform: impl Into<math::Transform<D>>,
    ) -> operations::Transform<T, D>;
}

impl<T, const D: usize> Transform<T, D> for T
where
    Const<D>: DimNameAdd<U1>,
    DefaultAllocator:
        Allocator<f32, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
{
    fn transform(
        self,
        transform: impl Into<math::Transform<D>>,
    ) -> operations::Transform<T, D> {
        operations::Transform {
            shape: self,
            transform: transform.into(),
        }
    }
}

/// Provides convenient syntax for [`operations::Translate`]
///
/// This trait is implemented for all types. The call `shape.translate(offset)`
/// will translate `shape` by `offset`.
pub trait Translate<T, const D: usize>: Sized {
    fn translate(self, offset: Vector<D>) -> operations::Translate<T, D>;
}

impl<T, const D: usize> Translate<T, D> for T {
    fn translate(self, offset: Vector<D>) -> operations::Translate<T, D> {
        operations::Translate {
            shape: self,
            offset,
        }
    }
}
