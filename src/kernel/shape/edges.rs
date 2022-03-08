use crate::{
    kernel::{
        geometry::{Circle, Curve, Line},
        topology::{edges::Edge, vertices::Vertex},
    },
    math::{Point, Scalar, Vector},
};

use super::{
    curves::Curves,
    handle::{Handle, Storage},
    EdgesInner,
};

/// The edges of a shape
pub struct Edges<'r> {
    pub(super) curves: Curves,
    pub(super) edges: &'r mut EdgesInner,
}

impl Edges<'_> {
    /// Add an edge to the shape
    ///
    /// If vertices are provided in `vertices`, they must be on `curve`.
    ///
    /// This constructor will convert the vertices into curve coordinates. If
    /// they are not on the curve, this will result in their projection being
    /// converted into curve coordinates, which is likely not the caller's
    /// intention.
    ///
    /// # Implementation notes
    ///
    /// Right now this is just an overly complicated constructor for `Edge`. In
    /// the future, it can add the edge to the proper internal data structures,
    /// and validate any constraints that apply to edge creation.
    pub fn add(&mut self, edge: Edge) -> Handle<Edge> {
        let storage = Storage::new(edge);
        let handle = storage.handle();

        self.edges.push(storage);

        handle
    }

    /// Add a circle to the shape
    ///
    /// Calls [`Edges::add`] internally, and is subject to the same
    /// restrictions.
    pub fn add_circle(&mut self, radius: Scalar) -> Handle<Edge> {
        let curve = self.curves.add(Curve::Circle(Circle {
            center: Point::origin(),
            radius: Vector::from([radius, Scalar::ZERO]),
        }));
        self.add(Edge {
            curve,
            vertices: None,
        })
    }

    /// Add a line segment to the shape
    ///
    /// Calls [`Edges::add`] internally, and is subject to the same
    /// restrictions.
    pub fn add_line_segment(
        &mut self,
        vertices: [Handle<Vertex>; 2],
    ) -> Handle<Edge> {
        let curve = self.curves.add(Curve::Line(Line::from_points(
            vertices.clone().map(|vertex| vertex.point()),
        )));
        self.add(Edge {
            curve,
            vertices: Some(vertices),
        })
    }
}
