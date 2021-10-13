use std::fmt;

use nalgebra::{
    allocator::Allocator, vector, Const, DefaultAllocator, DimNameAdd,
    DimNameSum, U1,
};

use crate::{
    geometry::{
        attributes::Vertices as _,
        operations::{Sweep, Transform},
        shapes::Vertex,
    },
    math::Vector,
};

/// A 1-dimensional edge
///
/// Defined as a sweep of a 0-dimensional `Vertex` over a straight path of a
/// given length.
pub type Edge = Sweep<Vertex, Vector<1>>;

impl Edge {
    /// Create a new `Edge`
    ///
    /// The length is initially set to `1.0`.
    pub fn new() -> Self {
        Sweep {
            shape: Vertex,
            path: vector![1.0],
        }
    }

    /// Update length
    ///
    /// Returns a copy of `self`, with the length replaced with `length`.
    pub fn with_length(mut self, length: f32) -> Self {
        self.path.x = length;
        self
    }
}

impl<const D: usize> Transform<Edge, D>
where
    Const<D>: DimNameAdd<U1>,
    DefaultAllocator: Allocator<f32, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>
        + Allocator<f32, DimNameSum<Const<D>, U1>, U1>,
{
    pub fn display(&self) -> impl fmt::Display {
        let vertices = self.vertices();

        let a = vertices[0];
        let b = vertices[1];

        format!("{} -> {}", a.display(), b.display())
    }
}
