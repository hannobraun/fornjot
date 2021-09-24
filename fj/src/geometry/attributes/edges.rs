use nalgebra::{
    allocator::Allocator, Const, DefaultAllocator, DimNameAdd, DimNameSum, U1,
};

use crate::geometry::{operations, shapes};

/// The edges that make up a shape
///
/// Since the edges of a shape are going to have a position and orientation in
/// space, `D` defines the dimension of those edges' positions.
pub trait Edges<const D: usize>
where
    Const<D>: DimNameAdd<U1>,
    DefaultAllocator:
        Allocator<f32, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
{
    /// Return the edges of the shape
    fn edges(&self) -> Vec<operations::Transform<shapes::Edge, D>>;
}
