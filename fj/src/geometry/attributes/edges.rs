use nalgebra::{
    allocator::Allocator, Const, DefaultAllocator, DimNameAdd, DimNameSum, U1,
};

use crate::{
    geometry::{
        operations,
        shapes::{self, Edge},
    },
    math,
};

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

impl<const D: usize> Edges<D> for Edge
where
    Const<D>: DimNameAdd<U1>,
    DefaultAllocator:
        Allocator<f32, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
{
    fn edges(&self) -> Vec<operations::Transform<shapes::Edge, D>> {
        vec![operations::Transform {
            shape: *self,
            transform: math::Transform::identity(),
        }]
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        geometry::shapes::Edge, math::Transform, syntax::Transform as _,
    };

    use super::Edges;

    #[test]
    fn test_edges_for_edge() {
        let edge = Edge::from_length(1.0);

        assert_eq!(
            <Edge as Edges<1>>::edges(&edge),
            [edge.transform(Transform::identity())]
        );
        assert_eq!(
            <Edge as Edges<2>>::edges(&edge),
            [edge.transform(Transform::identity())]
        );
        assert_eq!(
            <Edge as Edges<3>>::edges(&edge),
            [edge.transform(Transform::identity())]
        );
    }
}
