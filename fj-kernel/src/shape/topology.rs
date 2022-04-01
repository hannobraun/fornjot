use std::marker::PhantomData;

use fj_math::Scalar;

use crate::topology::{Cycle, Edge, Face, Vertex};

use super::{stores::Stores, validate::Validate as _, Iter, ValidationResult};

/// The vertices of a shape
pub struct Topology<'r> {
    pub(super) min_distance: Scalar,
    pub(super) stores: Stores,
    pub(super) _lifetime: PhantomData<&'r ()>,
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
        vertex.validate(self.min_distance, &self.stores)?;
        let handle = self.stores.vertices.insert(vertex);
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
        edge.validate(self.min_distance, &self.stores)?;
        let handle = self.stores.edges.insert(edge);
        Ok(handle)
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
        cycle.validate(self.min_distance, &self.stores)?;
        let handle = self.stores.cycles.insert(cycle);
        Ok(handle)
    }

    /// Add a face to the shape
    ///
    /// Validates that the face is structurally sound (i.e. the surface and
    /// cycles it refers to are part of the shape). Returns an error, if that is
    /// not the case.
    pub fn add_face(&mut self, face: Face) -> ValidationResult<Face> {
        face.validate(self.min_distance, &self.stores)?;
        let handle = self.stores.faces.insert(face);
        Ok(handle)
    }

    /// Access iterator over all vertices
    ///
    /// The caller must not make any assumptions about the order of vertices.
    pub fn vertices(&self) -> Iter<Vertex> {
        self.stores.vertices.iter()
    }

    /// Access iterator over all edges
    ///
    /// The caller must not make any assumptions about the order of edges.
    pub fn edges(&self) -> Iter<Edge> {
        self.stores.edges.iter()
    }

    /// Access an iterator over all cycles
    ///
    /// The caller must not make any assumptions about the order of cycles.
    pub fn cycles(&self) -> Iter<Cycle> {
        self.stores.cycles.iter()
    }

    /// Access an iterator over all faces
    ///
    /// The caller must not make any assumptions about the order of faces.
    pub fn faces(&self) -> Iter<Face> {
        self.stores.faces.iter()
    }
}

#[cfg(test)]
mod tests {
    use std::ops::{Deref, DerefMut};

    use fj_math::{Point, Scalar};

    use crate::{
        geometry::{Curve, Surface},
        shape::{Handle, Shape, ValidationError},
        topology::{Cycle, Edge, Face, Vertex},
    };

    const MIN_DISTANCE: f64 = 5e-7;

    #[test]
    fn add_vertex() -> anyhow::Result<()> {
        let mut shape = Shape::new().with_min_distance(MIN_DISTANCE);
        let mut other = Shape::new();

        let point = shape.insert(Point::from([0., 0., 0.]))?;
        shape.topology().add_vertex(Vertex { point })?;

        // Should fail, as `point` is not part of the shape.
        let point = other.insert(Point::from([1., 0., 0.]))?;
        let result = shape.topology().add_vertex(Vertex { point });
        assert!(matches!(result, Err(ValidationError::Structural(_))));

        // `point` is too close to the original point. `assert!` is commented,
        // because that only causes a warning to be logged right now.
        let point = shape.insert(Point::from([5e-8, 0., 0.]))?;
        let result = shape.topology().add_vertex(Vertex { point });
        assert!(matches!(result, Err(ValidationError::Uniqueness)));

        // `point` is farther than `MIN_DISTANCE` away from original point.
        // Should work.
        let point = shape.insert(Point::from([5e-6, 0., 0.]))?;
        shape.topology().add_vertex(Vertex { point })?;

        Ok(())
    }

    #[test]
    fn add_edge() -> anyhow::Result<()> {
        let mut shape = TestShape::new();
        let mut other = TestShape::new();

        let curve = other.add_curve();
        let a = Vertex::build(&mut other).from_point([1., 0., 0.])?;
        let b = Vertex::build(&mut other).from_point([2., 0., 0.])?;

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
        let a = Vertex::build(&mut shape).from_point([1., 0., 0.])?;
        let b = Vertex::build(&mut shape).from_point([2., 0., 0.])?;

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
                exteriors: vec![cycle.clone()],
                interiors: Vec::new(),
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
            exteriors: vec![cycle],
            interiors: Vec::new(),
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
            self.geometry().add_curve(Curve::x_axis())
        }

        fn add_surface(&mut self) -> Handle<Surface> {
            self.geometry().add_surface(Surface::x_y_plane())
        }

        fn add_edge(&mut self) -> anyhow::Result<Handle<Edge>> {
            let vertices = [(); 2].map(|()| {
                let point = self.next_point;
                self.next_point.x += Scalar::ONE;

                let point = self.insert(point).unwrap();
                self.topology().add_vertex(Vertex { point }).unwrap()
            });
            let edge = Edge::build(&mut self.inner)
                .line_segment_from_vertices(vertices)?;

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
