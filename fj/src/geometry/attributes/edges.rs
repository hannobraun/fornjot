use crate::geometry::shapes::{self, Edge};

/// The edges that make up a shape
///
/// `D` defines the dimension of the edges.
pub trait Edges<const D: usize> {
    /// Return the edges of the shape
    fn edges(&self) -> Vec<Edge<D>>;
}

impl<const D: usize, const N: usize> Edges<D> for shapes::Polygon<D, N> {
    fn edges(&self) -> Vec<Edge<D>> {
        let mut edges = Vec::new();

        edges.extend(self.points().windows(2).map(|window| {
            let a = window[0];
            let b = window[1];

            Edge::from([a, b])
        }));

        edges.push(Edge::from([
            *self.points().last().unwrap(),
            *self.points().first().unwrap(),
        ]));

        edges
    }
}
