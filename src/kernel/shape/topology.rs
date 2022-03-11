use tracing::warn;

use crate::{
    debug::DebugInfo,
    kernel::{
        geometry::{Circle, Curve, Line},
        topology::{
            edges::{Cycle, Edge},
            faces::Face,
            vertices::Vertex,
        },
    },
    math::{Point, Scalar, Triangle, Vector},
};

use super::{
    geometry::Geometry,
    handle::{Handle, Storage},
    Cycles, Edges, Faces, ValidationError, ValidationResult, Vertices,
};

/// The vertices of a shape
pub struct Topology<'r> {
    pub(super) min_distance: Scalar,

    pub(super) geometry: Geometry,

    pub(super) vertices: &'r mut Vertices,
    pub(super) edges: &'r mut Edges,
    pub(super) cycles: &'r mut Cycles,
    pub(super) faces: &'r mut Faces,
}

impl Topology<'_> {
    /// Add a vertex to the shape
    ///
    /// Logs a warning, if the vertex is not unique, meaning if another vertex
    /// defined by the same point already exists.
    ///
    /// In the context of of vertex uniqueness, points that are close to each
    /// other are considered identical. The minimum distance between distinct
    /// vertices can be configured using [`Shape::with_minimum_distance`].
    ///
    /// # Implementation note
    ///
    /// This method is intended to actually validate vertex uniqueness: To
    /// panic, if duplicate vertices are found. This is currently not possible,
    /// as the presence of bugs in the sweep and transform code would basically
    /// break ever model, due to validation errors.
    ///
    /// In the future, this method is likely to validate more than just vertex
    /// uniqueness. See documentation of [`crate::kernel`] for some context on
    /// that.
    pub fn add_vertex(&mut self, vertex: Vertex) -> ValidationResult<Vertex> {
        // Make sure the new vertex is a minimum distance away from all existing
        // vertices. This minimum distance is defined to be half a µm, which
        // should provide more than enough precision for common use cases, while
        // being large enough to catch all invalid cases.
        for existing in &*self.vertices {
            let distance = (existing.point() - vertex.point()).magnitude();

            if distance < self.min_distance {
                warn!(
                    "Invalid vertex: {vertex:?}; \
                    identical vertex at {existing:?}",
                );
            }
        }

        let storage = Storage::new(vertex);
        let handle = storage.handle();
        self.vertices.push(storage);

        Ok(handle)
    }

    /// Access iterator over all vertices
    ///
    /// The caller must not make any assumptions about the order of vertices.
    pub fn vertices(&self) -> impl Iterator<Item = Handle<Vertex>> + '_ {
        self.vertices.iter().map(|storage| storage.handle())
    }

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
    pub fn add_edge(&mut self, edge: Edge) -> ValidationResult<Edge> {
        for vertices in &edge.vertices {
            for vertex in vertices {
                if !self.vertices.contains(vertex.storage()) {
                    return Err(ValidationError::Structural);
                }
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
        }));
        self.add_edge(Edge {
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
        )));
        self.add_edge(Edge {
            curve,
            vertices: Some(vertices),
        })
    }

    /// Access iterator over all edges
    ///
    /// The caller must not make any assumptions about the order of edges.
    pub fn edges(&self) -> impl Iterator<Item = Handle<Edge>> + '_ {
        self.edges.iter().map(|storage| storage.handle())
    }

    /// Add a cycle to the shape
    ///
    /// # Panics
    ///
    /// Panics, if the edges of the cycles are not part of this shape.
    ///
    /// # Implementation note
    ///
    /// The validation of the cycle should be extended to cover more cases:
    /// - That those edges form a cycle.
    /// - That the cycle is not self-overlapping.
    /// - That there exists no duplicate cycle, with the same edges.
    pub fn add_cycle(&mut self, cycle: Cycle) -> ValidationResult<Cycle> {
        for edge in &cycle.edges {
            if !self.edges.contains(edge.storage()) {
                return Err(ValidationError::Structural);
            }
        }

        let storage = Storage::new(cycle);
        let handle = storage.handle();
        self.cycles.push(storage);

        Ok(handle)
    }

    /// Access an iterator over all cycles
    pub fn cycles(&self) -> impl Iterator<Item = Handle<Cycle>> + '_ {
        self.cycles.iter().map(|storage| storage.handle())
    }

    /// Add a face to the shape
    pub fn add_face(&mut self, face: Face) -> ValidationResult<Face> {
        let storage = Storage::new(face);
        let handle = storage.handle();

        self.faces.push(storage);

        Ok(handle)
    }

    /// Access an iterator over all faces
    pub fn faces(&self) -> impl Iterator<Item = Handle<Face>> + '_ {
        self.faces.iter().map(|storage| storage.handle())
    }

    pub fn triangles(
        &self,
        tolerance: Scalar,
        out: &mut Vec<Triangle<3>>,
        debug_info: &mut DebugInfo,
    ) {
        for face in &*self.faces {
            face.triangles(tolerance, out, debug_info);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        kernel::{
            shape::{handle::Handle, Shape, ValidationError},
            topology::{
                edges::{Cycle, Edge},
                vertices::Vertex,
            },
        },
        math::Point,
    };

    const MIN_DISTANCE: f64 = 5e-7;

    #[test]
    fn add_vertex() -> anyhow::Result<()> {
        let mut shape = Shape::new().with_min_distance(MIN_DISTANCE);

        let point = shape.geometry().add_point(Point::from([0., 0., 0.]));
        shape.topology().add_vertex(Vertex { point })?;

        // `point` is too close to the original point. `assert!` is commented,
        // because that only causes a warning to be logged right now.
        let point = shape.geometry().add_point(Point::from([5e-6, 0., 0.]));
        let _result = shape.topology().add_vertex(Vertex { point });
        // assert!(matches!(result, Err(ValidationError::Uniqueness)));

        // `point` is farther than `MIN_DISTANCE` away from original point.
        // Should work.
        let point = shape.geometry().add_point(Point::from([5e-6, 0., 0.]));
        shape.topology().add_vertex(Vertex { point })?;

        Ok(())
    }

    #[test]
    fn add_edge() -> anyhow::Result<()> {
        let mut shape = Shape::new();
        let mut other = Shape::new();

        let a = other.geometry().add_point(Point::from([0., 0., 0.]));
        let b = other.geometry().add_point(Point::from([1., 0., 0.]));

        let a = other.topology().add_vertex(Vertex { point: a })?;
        let b = other.topology().add_vertex(Vertex { point: b })?;

        // Shouldn't work. None of the vertices have been added to `shape`.
        let result = shape.topology().add_line_segment([a.clone(), b.clone()]);
        assert!(matches!(result, Err(ValidationError::Structural)));

        let a = shape.geometry().add_point(a.point());
        let a = shape.topology().add_vertex(Vertex { point: a })?;

        // Shouldn't work. Only `a` has been added to `shape`.
        let result = shape.topology().add_line_segment([a.clone(), b.clone()]);
        assert!(matches!(result, Err(ValidationError::Structural)));

        let b = shape.geometry().add_point(b.point());
        let b = shape.topology().add_vertex(Vertex { point: b })?;

        // Both `a` and `b` have been added to `shape`. Should work!
        shape.topology().add_line_segment([a, b])?;

        Ok(())
    }

    #[test]
    fn add_cycle() -> anyhow::Result<()> {
        let (mut shape, edge) = TestShape::new()?;
        let (_, other_edge) = TestShape::new()?;

        // Trying to refer to edge that is not from the same shape. Should fail.
        let result = shape.inner.topology().add_cycle(Cycle {
            edges: vec![other_edge],
        });
        assert!(matches!(result, Err(ValidationError::Structural)));

        // Referring to edge that *is* from the same shape. Should work.
        shape
            .inner
            .topology()
            .add_cycle(Cycle { edges: vec![edge] })?;

        Ok(())
    }

    struct TestShape {
        inner: Shape,
    }

    impl TestShape {
        fn new() -> anyhow::Result<(Self, Handle<Edge>)> {
            let mut inner = Shape::new();

            let a = inner.geometry().add_point(Point::from([0., 0., 0.]));
            let b = inner.geometry().add_point(Point::from([1., 0., 0.]));

            let a = inner.topology().add_vertex(Vertex { point: a })?;
            let b = inner.topology().add_vertex(Vertex { point: b })?;

            let edge = inner.topology().add_line_segment([a, b])?;

            Ok((Self { inner }, edge))
        }
    }
}
