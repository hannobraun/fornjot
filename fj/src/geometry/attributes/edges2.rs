use crate::geometry::shapes::{self, Edge2};

/// The edges that make up a shape
///
/// `D` defines the dimension of the edges.
pub trait Edges2<const D: usize> {
    /// Return the edges of the shape
    fn edges(&self) -> Vec<Edge2<D>>;
}

impl<const D: usize, const N: usize> Edges2<D> for shapes::Polygon<D, N> {
    fn edges(&self) -> Vec<Edge2<D>> {
        let mut edges = Vec::new();

        edges.extend(self.points().windows(2).map(|window| {
            let a = window[0];
            let b = window[1];

            Edge2::from([a, b])
        }));

        edges.push(Edge2::from([
            *self.points().last().unwrap(),
            *self.points().first().unwrap(),
        ]));

        edges
    }
}
