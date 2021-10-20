use crate::geometry::shapes::Edge2;

/// The edges that make up a shape
///
/// `D` defines the dimension of the edges.
pub trait Edges2<const D: usize> {
    /// Return the edges of the shape
    fn edges(&self) -> Vec<Edge2<D>>;
}
