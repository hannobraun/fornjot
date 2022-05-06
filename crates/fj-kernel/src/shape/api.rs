use fj_math::{Point, Scalar, Transform};

use crate::{
    geometry::{Curve, Surface},
    topology::{Cycle, Edge, Face, Vertex},
};

use super::{
    stores::{
        Curves, Cycles, Edges, Faces, Points, Stores, Surfaces, Vertices,
    },
    Handle, Iter, Object, ValidationResult,
};

/// The boundary representation of a shape
#[derive(Clone, Debug)]
pub struct Shape {
    min_distance: Scalar,
    stores: Stores,
}

impl Shape {
    /// Construct a new shape
    pub fn new() -> Self {
        Self {
            // This should really come from `Self::DEFAULT_MIN_DISTANCE`, or a
            // similarly named constant. Unfortunately `Scalar::from_f64` can't
            // be `const` yet.
            min_distance: Scalar::from_f64(5e-7), // 0.5 µm

            stores: Stores {
                points: Points::new(),
                curves: Curves::new(),
                surfaces: Surfaces::new(),

                vertices: Vertices::new(),
                edges: Edges::new(),
                cycles: Cycles::new(),
                faces: Faces::new(),
            },
        }
    }

    /// Override the minimum distance for this shape
    ///
    /// Used for vertex validation, to determine whether vertices are unique.
    ///
    /// # Implementation note
    ///
    /// This functionality should be exposed to models, eventually. For now it's
    /// just used in unit tests.
    #[cfg(test)]
    pub fn with_min_distance(
        mut self,
        min_distance: impl Into<Scalar>,
    ) -> Self {
        self.min_distance = min_distance.into();
        self
    }

    /// Insert an object into the shape
    ///
    /// Validates the object, and returns an error if it is not valid. See the
    /// documentation of each object for validation requirements.
    pub fn insert<T>(&mut self, object: T) -> ValidationResult<T>
    where
        T: Object,
    {
        object.validate(self.min_distance, &self.stores)?;
        let handle = self.stores.get::<T>().insert(object);
        Ok(handle)
    }

    /// Access the handle of an object
    ///
    /// Returns the handle that refers to the given object, if it is part of the
    /// shape. Returns `None`, if it isn't.
    ///
    /// # Implementation note
    ///
    /// If `object` is present multiple times, the handle of the first that is
    /// found is returned. This is weird. It would be better, if objects were
    /// unique, but currently they are stored in `Vec`s.
    ///
    /// This probably isn't worth thinking too much about right now. At some
    /// point, we need smarter and probably more performant object storage
    /// anyway.
    pub fn get_handle<T>(&self, object: &T) -> Option<Handle<T>>
    where
        T: Object,
    {
        self.stores
            .get::<T>()
            .iter()
            .find(|obj| &obj.get() == object)
    }

    /// Get handle of an identical object, if it exists, or add the object
    ///
    /// In any case, returns a handle that refers to an object that is identical
    /// to the provided object.
    pub fn get_handle_or_insert<T>(&mut self, object: T) -> ValidationResult<T>
    where
        T: Object,
    {
        if let Some(handle) = self.get_handle(&object) {
            return Ok(handle);
        }

        self.insert(object)
    }

    /// Transform the geometry of the shape
    ///
    /// Since the topological types refer to geometry, and don't contain any
    /// geometry themselves, this transforms the whole shape.
    pub fn transform(&mut self, transform: &Transform) {
        self.stores
            .points
            .update(|point| *point = transform.transform_point(point));
        self.stores
            .curves
            .update(|curve| *curve = curve.transform(transform));
        self.stores
            .surfaces
            .update(|surface| *surface = surface.transform(transform));

        // While some faces use triangle representation, we need this weird
        // workaround here.
        self.stores.faces.update(|mut face| {
            use std::ops::DerefMut as _;
            if let Face::Triangles(triangles) = face.deref_mut() {
                for (triangle, _) in triangles {
                    *triangle = transform.transform_triangle(triangle);
                }
            }
        });
    }

    /// Access an iterator over all points
    ///
    /// The caller must not make any assumptions about the order of points.
    pub fn points(&self) -> Iter<Point<3>> {
        self.stores.points.iter()
    }

    /// Access an iterator over all curves
    ///
    /// The caller must not make any assumptions about the order of curves.
    pub fn curves(&self) -> Iter<Curve> {
        self.stores.curves.iter()
    }

    /// Access an iterator over all surfaces
    ///
    /// The caller must not make any assumptions about the order of surfaces.
    pub fn surfaces(&self) -> Iter<Surface> {
        self.stores.surfaces.iter()
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

impl Default for Shape {
    fn default() -> Self {
        Self::new()
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

    fn get_handle() -> anyhow::Result<()> {
        let mut shape = Shape::new();

        let point = Point::from([1., 0., 0.]);
        let curve = Curve::x_axis();
        let surface = Surface::xy_plane();

        assert!(shape.get_handle(&point).is_none());
        assert!(shape.get_handle(&curve).is_none());
        assert!(shape.get_handle(&surface).is_none());

        let point = shape.insert(point)?;
        let curve = shape.insert(curve)?;
        let surface = shape.insert(surface)?;

        assert!(shape.get_handle(&point.get()).as_ref() == Some(&point));
        assert!(shape.get_handle(&curve.get()).as_ref() == Some(&curve));
        assert!(shape.get_handle(&surface.get()).as_ref() == Some(&surface));

        let vertex = Vertex { point };
        let edge = Edge {
            curve,
            vertices: None,
        };

        assert!(shape.get_handle(&vertex).is_none());
        assert!(shape.get_handle(&edge).is_none());

        let vertex = shape.insert(vertex)?;
        let edge = shape.insert(edge)?;

        assert!(shape.get_handle(&vertex.get()).as_ref() == Some(&vertex));
        assert!(shape.get_handle(&edge.get()).as_ref() == Some(&edge));

        let cycle = Cycle { edges: vec![edge] };
        assert!(shape.get_handle(&cycle).is_none());

        let cycle = shape.insert(cycle)?;
        assert!(shape.get_handle(&cycle.get()).as_ref() == Some(&cycle));

        let face = Face::Face {
            surface,
            exteriors: Vec::new(),
            interiors: Vec::new(),
            color: [0, 0, 0, 0],
        };
        assert!(shape.get_handle(&face).is_none());

        let face = shape.insert(face)?;
        assert!(shape.get_handle(&face.get()).as_ref() == Some(&face));

        Ok(())
    }

    #[test]
    fn add_vertex() -> anyhow::Result<()> {
        let mut shape = Shape::new().with_min_distance(MIN_DISTANCE);
        let mut other = Shape::new();

        let point = shape.insert(Point::from([0., 0., 0.]))?;
        shape.insert(Vertex { point })?;

        // Should fail, as `point` is not part of the shape.
        let point = other.insert(Point::from([1., 0., 0.]))?;
        let result = shape.insert(Vertex { point });
        assert!(matches!(result, Err(ValidationError::Structural(_))));

        // `point` is too close to the original point. `assert!` is commented,
        // because that only causes a warning to be logged right now.
        let point = shape.insert(Point::from([5e-8, 0., 0.]))?;
        let result = shape.insert(Vertex { point });
        assert!(matches!(result, Err(ValidationError::Uniqueness)));

        // `point` is farther than `MIN_DISTANCE` away from original point.
        // Should work.
        let point = shape.insert(Point::from([5e-6, 0., 0.]))?;
        shape.insert(Vertex { point })?;

        Ok(())
    }

    #[test]
    fn add_edge() -> anyhow::Result<()> {
        let mut shape = TestShape::new();
        let mut other = TestShape::new();

        let curve = other.add_curve();
        let a = Vertex::builder(&mut other).build_from_point([1., 0., 0.])?;
        let b = Vertex::builder(&mut other).build_from_point([2., 0., 0.])?;

        // Shouldn't work. Nothing has been added to `shape`.
        let err = shape
            .insert(Edge {
                curve: curve.clone(),
                vertices: Some([a.clone(), b.clone()]),
            })
            .unwrap_err();
        assert!(err.missing_curve(&curve));
        assert!(err.missing_vertex(&a));
        assert!(err.missing_vertex(&b));

        let curve = shape.add_curve();
        let a = Vertex::builder(&mut shape).build_from_point([1., 0., 0.])?;
        let b = Vertex::builder(&mut shape).build_from_point([2., 0., 0.])?;

        // Everything has been added to `shape` now. Should work!
        shape.insert(Edge {
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
            .insert(Cycle {
                edges: vec![edge.clone()],
            })
            .unwrap_err();
        assert!(err.missing_edge(&edge));

        // Referring to edge that *is* from the same shape. Should work.
        let edge = shape.add_edge()?;
        shape.insert(Cycle { edges: vec![edge] })?;

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
            .insert(Face::Face {
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
        shape.insert(Face::Face {
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
            self.insert(Curve::x_axis()).unwrap()
        }

        fn add_surface(&mut self) -> Handle<Surface> {
            self.insert(Surface::xy_plane()).unwrap()
        }

        fn add_edge(&mut self) -> anyhow::Result<Handle<Edge>> {
            let points = [(); 2].map(|()| {
                let point = self.next_point;
                self.next_point.x += Scalar::ONE;
                point
            });
            let edge = Edge::builder(&mut self.inner)
                .build_line_segment_from_points(points)?;

            Ok(edge)
        }

        fn add_cycle(&mut self) -> anyhow::Result<Handle<Cycle>> {
            let edge = self.add_edge()?;
            let cycle = self.insert(Cycle { edges: vec![edge] })?;
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
