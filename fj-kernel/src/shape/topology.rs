use std::collections::HashSet;

use fj_debug::DebugInfo;
use fj_math::{Point, Scalar, Triangle, Vector};

use crate::{
    geometry::{Circle, Curve, Line},
    topology::{Cycle, Edge, Face, Vertex},
};

use super::{
    handle::{Handle, Storage},
    Cycles, Edges, Geometry, Iter, ValidationError, ValidationResult, Vertices,
};

/// The vertices of a shape
pub struct Topology<'r> {
    pub(super) min_distance: Scalar,

    pub(super) geometry: Geometry<'r>,

    pub(super) vertices: &'r mut Vertices,
    pub(super) edges: &'r mut Edges,
    pub(super) cycles: &'r mut Cycles,
}

impl Topology<'_> {
    /// Add a vertex to the shape
    ///
    /// Validates that the vertex is structurally sound (i.e. the point it
    /// refers to is part of the shape). Returns an error, if that is not the
    /// case.
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
    /// In the future, this method is likely to validate more than it already
    /// does. See documentation of [`crate::kernel`] for some context on that.
    pub fn add_vertex(&mut self, vertex: Vertex) -> ValidationResult<Vertex> {
        if !self.geometry.points.contains(vertex.point.storage()) {
            return Err(ValidationError::Structural(()));
        }
        for existing in &*self.vertices {
            let distance =
                (existing.get().point() - vertex.point()).magnitude();

            if distance < self.min_distance {
                return Err(ValidationError::Uniqueness);
            }
        }

        let storage = Storage::new(vertex);
        let handle = storage.handle();
        self.vertices.push(storage);

        Ok(handle)
    }

    /// Add an edge to the shape
    ///
    /// Validates that the edge is structurally sound (i.e. the curve and
    /// vertices it refers to are part of the shape). Returns an error, if that
    /// is not the case.
    ///
    /// # Vertices
    ///
    /// If vertices are provided in `vertices`, they must be on `curve`.
    ///
    /// This constructor will convert the vertices into curve coordinates. If
    /// they are not on the curve, this will result in their projection being
    /// converted into curve coordinates, which is likely not the caller's
    /// intention.
    pub fn add_edge(&mut self, edge: Edge) -> ValidationResult<Edge> {
        let mut missing_curve = None;
        let mut missing_vertices = HashSet::new();

        if !self.geometry.curves.contains(edge.curve.storage()) {
            missing_curve = Some(edge.curve.clone());
        }
        for vertices in &edge.vertices {
            for vertex in vertices {
                if !self.vertices.contains(vertex.storage()) {
                    missing_vertices.insert(vertex.clone());
                }
            }
        }

        if missing_curve.is_some() || !missing_vertices.is_empty() {
            return Err(ValidationError::Structural((
                missing_curve,
                missing_vertices,
            )));
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
            vertices.clone().map(|vertex| vertex.get().point()),
        )));
        self.add_edge(Edge {
            curve,
            vertices: Some(vertices),
        })
    }

    /// Add a cycle to the shape
    ///
    /// Validates that the cycle is structurally sound (i.e. the edges it refers
    /// to are part of the shape). Returns an error, if that is not the case.
    ///
    /// # Implementation note
    ///
    /// The validation of the cycle should be extended to cover more cases:
    /// - That those edges form a cycle.
    /// - That the cycle is not self-overlapping.
    /// - That there exists no duplicate cycle, with the same edges.
    pub fn add_cycle(&mut self, cycle: Cycle) -> ValidationResult<Cycle> {
        let mut missing_edges = HashSet::new();
        for edge in &cycle.edges {
            if !self.edges.contains(edge.storage()) {
                missing_edges.insert(edge.clone());
            }
        }

        if !missing_edges.is_empty() {
            return Err(ValidationError::Structural(missing_edges));
        }

        let storage = Storage::new(cycle);
        let handle = storage.handle();
        self.cycles.push(storage);

        Ok(handle)
    }

    /// Add a face to the shape
    ///
    /// Validates that the face is structurally sound (i.e. the surface and
    /// cycles it refers to are part of the shape). Returns an error, if that is
    /// not the case.
    pub fn add_face(&mut self, face: Face) -> ValidationResult<Face> {
        if let Face::Face {
            surface, cycles, ..
        } = &face
        {
            let mut missing_surface = None;
            let mut missing_cycles = HashSet::new();

            if !self.geometry.surfaces.contains(surface.storage()) {
                missing_surface = Some(surface.clone());
            }
            for cycle in cycles {
                if !self.cycles.contains(cycle.storage()) {
                    missing_cycles.insert(cycle.clone());
                }
            }

            if missing_surface.is_some() || !missing_cycles.is_empty() {
                return Err(ValidationError::Structural((
                    missing_surface,
                    missing_cycles,
                )));
            }
        }

        let storage = Storage::new(face);
        let handle = storage.handle();

        self.geometry.faces.push(storage);

        Ok(handle)
    }

    /// Access iterator over all vertices
    ///
    /// The caller must not make any assumptions about the order of vertices.
    pub fn vertices(&self) -> Iter<Vertex> {
        Iter::new(self.vertices)
    }

    /// Access iterator over all edges
    ///
    /// The caller must not make any assumptions about the order of edges.
    pub fn edges(&self) -> Iter<Edge> {
        Iter::new(self.edges)
    }

    /// Access an iterator over all cycles
    ///
    /// The caller must not make any assumptions about the order of cycles.
    pub fn cycles(&self) -> Iter<Cycle> {
        Iter::new(self.cycles)
    }

    /// Access an iterator over all faces
    ///
    /// The caller must not make any assumptions about the order of faces.
    pub fn faces(&self) -> Iter<Face> {
        Iter::new(self.geometry.faces)
    }

    pub fn triangles(
        &self,
        tolerance: Scalar,
        out: &mut Vec<Triangle<3>>,
        debug_info: &mut DebugInfo,
    ) {
        for face in &*self.geometry.faces {
            face.get().triangles(tolerance, out, debug_info);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ops::{Deref, DerefMut};

    use fj_math::{Point, Scalar};

    use crate::{
        geometry::{Curve, Line, Surface},
        shape::{handle::Handle, Shape, ValidationError},
        topology::{Cycle, Edge, Face, Vertex},
    };

    const MIN_DISTANCE: f64 = 5e-7;

    #[test]
    fn add_vertex() -> anyhow::Result<()> {
        let mut shape = Shape::new().with_min_distance(MIN_DISTANCE);
        let mut other = Shape::new();

        let point = shape.geometry().add_point(Point::from([0., 0., 0.]));
        shape.topology().add_vertex(Vertex { point })?;

        // Should fail, as `point` is not part of the shape.
        let point = other.geometry().add_point(Point::from([1., 0., 0.]));
        let result = shape.topology().add_vertex(Vertex { point });
        assert!(matches!(result, Err(ValidationError::Structural(()))));

        // `point` is too close to the original point. `assert!` is commented,
        // because that only causes a warning to be logged right now.
        let point = shape.geometry().add_point(Point::from([5e-8, 0., 0.]));
        let result = shape.topology().add_vertex(Vertex { point });
        assert!(matches!(result, Err(ValidationError::Uniqueness)));

        // `point` is farther than `MIN_DISTANCE` away from original point.
        // Should work.
        let point = shape.geometry().add_point(Point::from([5e-6, 0., 0.]));
        shape.topology().add_vertex(Vertex { point })?;

        Ok(())
    }

    #[test]
    fn add_edge() -> anyhow::Result<()> {
        let mut shape = TestShape::new();
        let mut other = TestShape::new();

        let curve = other.add_curve();
        let a = other.add_vertex()?;
        let b = other.add_vertex()?;

        // Shouldn't work. Nothing has been added to `shape`.
        let err = shape
            .topology()
            .add_edge(Edge {
                curve: curve.clone(),
                vertices: Some([a.clone(), b.clone()]),
            })
            .unwrap_err();
        assert!(err.missing_curve(&curve));
        assert!(err.missing_vertex(&a));
        assert!(err.missing_vertex(&b));

        let curve = shape.add_curve();
        let a = shape.add_vertex()?;
        let b = shape.add_vertex()?;

        // Everything has been added to `shape` now. Should work!
        shape.topology().add_edge(Edge {
            curve,
            vertices: Some([a, b]),
        })?;

        Ok(())
    }

    #[test]
    fn add_cycle() -> anyhow::Result<()> {
        let mut shape = TestShape::new();
        let mut other = TestShape::new();

        // Trying to refer to edge that is not from the same shape. Should fail.
        let edge = other.add_edge()?;
        let err = shape
            .topology()
            .add_cycle(Cycle {
                edges: vec![edge.clone()],
            })
            .unwrap_err();
        assert!(err.missing_edge(&edge));

        // Referring to edge that *is* from the same shape. Should work.
        let edge = shape.add_edge()?;
        shape.topology().add_cycle(Cycle { edges: vec![edge] })?;

        Ok(())
    }

    #[test]
    fn add_face() -> anyhow::Result<()> {
        let mut shape = TestShape::new();
        let mut other = TestShape::new();

        let surface = other.add_surface();
        let cycle = other.add_cycle()?;

        // Nothing has been added to `shape`. Should fail.
        let err = shape
            .topology()
            .add_face(Face::Face {
                surface: surface.clone(),
                cycles: vec![cycle.clone()],
                color: [255, 0, 0, 255],
            })
            .unwrap_err();
        assert!(err.missing_surface(&surface));
        assert!(err.missing_cycle(&cycle));

        let surface = shape.add_surface();
        let cycle = shape.add_cycle()?;

        // Everything has been added to `shape` now. Should work!
        shape.topology().add_face(Face::Face {
            surface,
            cycles: vec![cycle],
            color: [255, 0, 0, 255],
        })?;

        Ok(())
    }

    struct TestShape {
        inner: Shape,
        next_point: Point<3>,
    }

    impl TestShape {
        fn new() -> Self {
            Self {
                inner: Shape::new(),
                next_point: Point::from([0., 0., 0.]),
            }
        }

        fn add_curve(&mut self) -> Handle<Curve> {
            self.geometry().add_curve(Curve::Line(Line::from_points([
                Point::from([0., 0., 0.]),
                Point::from([1., 0., 0.]),
            ])))
        }

        fn add_surface(&mut self) -> Handle<Surface> {
            self.geometry().add_surface(Surface::x_y_plane())
        }

        fn add_vertex(&mut self) -> anyhow::Result<Handle<Vertex>> {
            let point = self.next_point;
            self.next_point.x += Scalar::ONE;

            let point = self.geometry().add_point(point);
            let vertex = self.topology().add_vertex(Vertex { point })?;

            Ok(vertex)
        }

        fn add_edge(&mut self) -> anyhow::Result<Handle<Edge>> {
            let vertices = [(); 2].map(|()| self.add_vertex().unwrap());
            let edge = self.topology().add_line_segment(vertices)?;
            Ok(edge)
        }

        fn add_cycle(&mut self) -> anyhow::Result<Handle<Cycle>> {
            let edge = self.add_edge()?;
            let cycle =
                self.topology().add_cycle(Cycle { edges: vec![edge] })?;
            Ok(cycle)
        }
    }

    impl Deref for TestShape {
        type Target = Shape;

        fn deref(&self) -> &Self::Target {
            &self.inner
        }
    }

    impl DerefMut for TestShape {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.inner
        }
    }
}
