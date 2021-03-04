use indexmap::IndexSet;
use nalgebra::Point2;
use parry2d::shape::Segment;

use crate::geometry::point::Pnt2;

/// A vertex chain
///
/// Quite literally, a chain of vertices. The first and the last vertex in the
/// chain are considered to be connected.
///
/// Vertex chains are the basis of polygons. A polygon often consists of a
/// single vertex chain, but it might consist of multiple, if the polygon has
/// holes.
///
/// Vertex chains are considered "positive", i.e. forming a polygon, if their
/// vertices are in counter-clockwise order. They are considered "negative",
/// i.e. holes in another polygon, if their vertices are in clockwise order.
pub struct VertexChain(IndexSet<Pnt2>);

impl VertexChain {
    pub fn new() -> Self {
        Self(IndexSet::new())
    }

    /// Insert new vertex into the chain
    pub fn insert(&mut self, vertex: impl Into<Pnt2>) {
        self.0.insert(vertex.into());
    }

    /// Remove a vertex from the chain
    pub fn remove(&mut self, vertex: impl Into<Pnt2>) {
        self.0.remove(&vertex.into());
    }

    /// Returns the line segments forming the vertex chain
    pub fn segments(&self) -> Vec<Segment> {
        // This gets us access to the `windows` method. Certainly not the best
        // way to implement this. It work that way, because the vertices were in
        // a `Vec` originally, and this was the easiest way to change that over
        // to an `IndexSet`.
        let vertices: Vec<_> = self.0.iter().map(|&pnt| pnt).collect();

        let mut edges = Vec::new();

        edges.extend(vertices.windows(2).map(|window| {
            let a = window[0].into();
            let b = window[1].into();
            Segment::new(a, b)
        }));

        let first = vertices.first().unwrap().into();
        let last = vertices.last().unwrap().into();
        edges.push(Segment::new(last, first));

        edges
    }
}

impl From<&[Point2<f32>]> for VertexChain {
    fn from(points: &[Point2<f32>]) -> Self {
        let points: IndexSet<_> =
            points.into_iter().map(|point| point.into()).collect();
        Self(points)
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::Point2;
    use parry2d::shape::Segment;

    use super::VertexChain;

    #[test]
    fn vertex_chain_should_remove_vertex() {
        let a = Point2::new(0.0, 0.0);
        let b = Point2::new(1.0, 0.0);
        let c = Point2::new(0.0, 1.0);

        let mut vertex_chain = VertexChain::new();
        vertex_chain.insert(a);
        vertex_chain.insert(b);
        vertex_chain.insert(c);

        vertex_chain.remove(b);
        let segments = vertex_chain.segments();

        // This is a degenerate case, but for the purposes of this test, it
        // doesn't matter.
        assert_eq!(segments, vec![Segment::new(a, c), Segment::new(c, a)]);
    }

    #[test]
    fn vertex_chain_should_return_segments() {
        let a = Point2::new(0.0, 0.0);
        let b = Point2::new(1.0, 0.0);
        let c = Point2::new(0.0, 1.0);

        let mut vertex_chain = VertexChain::new();
        vertex_chain.insert(a);
        vertex_chain.insert(b);
        vertex_chain.insert(c);

        let segments = vertex_chain.segments();

        assert_eq!(
            segments,
            vec![Segment::new(a, b), Segment::new(b, c), Segment::new(c, a)]
        );
    }
}
