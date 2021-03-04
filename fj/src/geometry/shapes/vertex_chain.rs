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
#[derive(Debug)]
pub struct VertexChain(IndexSet<Pnt2>);

impl VertexChain {
    pub fn new() -> Self {
        Self(IndexSet::new())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
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

    /// Returns the neighbors of the given point
    pub fn neighbors_of(&self, vertex: impl Into<Pnt2>) -> Option<Neighbors> {
        let mut neighbors = IndexSet::new();

        let index = self.0.get_index_of(&vertex.into())?;

        let prev = index.overflowing_sub(1).0.clamp(0, self.0.len() - 1);
        let next = index.overflowing_add(1).0 % self.0.len();

        // Can't panic, as we did the whole clamping and remainder dance above.
        let prev = *self.0.get_index(prev).unwrap();
        let next = *self.0.get_index(next).unwrap();

        neighbors.insert(prev);
        neighbors.insert(next);

        Some(Neighbors(neighbors))
    }
}

impl From<&[Point2<f32>]> for VertexChain {
    fn from(points: &[Point2<f32>]) -> Self {
        let points: IndexSet<_> =
            points.into_iter().map(|point| point.into()).collect();
        Self(points)
    }
}

pub struct Neighbors(IndexSet<Pnt2>);

impl Neighbors {
    pub fn contains(&self, vertex: impl Into<Pnt2>) -> bool {
        self.0.contains(&vertex.into())
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

    #[test]
    fn vertex_chain_should_return_neighbors_of_a_vertex() {
        let a = Point2::new(0.0, 0.0);
        let b = Point2::new(1.0, 0.0);
        let c = Point2::new(1.0, 1.0);
        let d = Point2::new(0.0, 1.0);

        let mut vertex_chain = VertexChain::new();
        vertex_chain.insert(a);
        vertex_chain.insert(b);
        vertex_chain.insert(c);
        vertex_chain.insert(d);

        let neighbors_of_a = vertex_chain.neighbors_of(a).unwrap();
        let neighbors_of_b = vertex_chain.neighbors_of(b).unwrap();
        let neighbors_of_c = vertex_chain.neighbors_of(c).unwrap();
        let neighbors_of_d = vertex_chain.neighbors_of(d).unwrap();

        assert!(!neighbors_of_a.contains(a));
        assert!(neighbors_of_a.contains(b));
        assert!(!neighbors_of_a.contains(c));
        assert!(neighbors_of_a.contains(d));

        assert!(neighbors_of_b.contains(a));
        assert!(!neighbors_of_b.contains(b));
        assert!(neighbors_of_b.contains(c));
        assert!(!neighbors_of_b.contains(d));

        assert!(!neighbors_of_c.contains(a));
        assert!(neighbors_of_c.contains(b));
        assert!(!neighbors_of_c.contains(c));
        assert!(neighbors_of_c.contains(d));

        assert!(neighbors_of_d.contains(a));
        assert!(!neighbors_of_d.contains(b));
        assert!(neighbors_of_d.contains(c));
        assert!(!neighbors_of_d.contains(d));
    }
}
