use crate::{
    kernel::{
        geometry::{Circle, Curve, Line},
        topology::{edges::Edge, vertices::Vertex},
    },
    math::{Point, Scalar, Vector},
};

use super::{
    geometry::Geometry,
    handle::{Handle, Storage},
    EdgesInner, ValidationResult, VerticesInner,
};

/// The edges of a shape
pub struct Edges<'r> {
    pub(super) geometry: Geometry,
    pub(super) vertices: &'r mut VerticesInner,
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
    pub fn add(&mut self, edge: Edge) -> ValidationResult<Edge> {
        for vertices in &edge.vertices {
            for vertex in vertices {
                assert!(
                    self.vertices.contains(vertex.storage()),
                    "Edge validation failed: {vertex:?} is not part of shape",
                );
            }
        }

        let storage = Storage::new(edge);
        let handle = storage.handle();

        self.edges.push(storage);

        Ok(handle)
    }

    /// Add a circle to the shape
    ///
    /// Calls [`Edges::add`] internally, and is subject to the same
    /// restrictions.
    pub fn add_circle(&mut self, radius: Scalar) -> ValidationResult<Edge> {
        let curve = self.geometry.add_curve(Curve::Circle(Circle {
            center: Point::origin(),
            radius: Vector::from([radius, Scalar::ZERO]),
        }))?;
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
    ) -> ValidationResult<Edge> {
        let curve = self.geometry.add_curve(Curve::Line(Line::from_points(
            vertices.clone().map(|vertex| vertex.point()),
        )))?;
        self.add(Edge {
            curve,
            vertices: Some(vertices),
        })
    }

    /// Access iterator over all edges
    ///
    /// The caller must not make any assumptions about the order of edges.
    pub fn all(&self) -> impl Iterator<Item = Handle<Edge>> + '_ {
        self.edges.iter().map(|storage| storage.handle())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        kernel::{shape::Shape, topology::vertices::Vertex},
        math::Point,
    };

    #[test]
    fn add_valid() {
        let mut shape = Shape::new();

        let a = shape
            .geometry()
            .add_point(Point::from([0., 0., 0.]))
            .unwrap();
        let b = shape
            .geometry()
            .add_point(Point::from([1., 0., 0.]))
            .unwrap();

        let a = shape.vertices().add(Vertex { point: a }).unwrap();
        let b = shape.vertices().add(Vertex { point: b }).unwrap();

        shape.edges().add_line_segment([a, b]).unwrap();
    }

    #[test]
    #[should_panic]
    fn add_invalid() {
        let mut shape = Shape::new();
        let mut other = Shape::new();

        let a = shape
            .geometry()
            .add_point(Point::from([0., 0., 0.]))
            .unwrap();
        let b = shape
            .geometry()
            .add_point(Point::from([1., 0., 0.]))
            .unwrap();

        let a = other.vertices().add(Vertex { point: a }).unwrap();
        let b = other.vertices().add(Vertex { point: b }).unwrap();

        shape.edges().add_line_segment([a, b]).unwrap();
    }
}
