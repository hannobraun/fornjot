use fj_math::{Point, Scalar};

use crate::{
    geometry::{Curve, Surface},
    topology::{Cycle, Edge, Face, Vertex},
};

use super::{
    stores::{Store, Stores},
    Handle, Iter, Mapping, Object, Update, ValidationError, ValidationResult,
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
                points: Store::new(),
                curves: Store::new(),
                surfaces: Store::new(),

                vertices: Store::new(),
                edges: Store::new(),
                cycles: Store::new(),
                faces: Store::new(),
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
    pub fn insert<T: Object>(&mut self, object: T) -> ValidationResult<T> {
        object.validate(None, self.min_distance, &self.stores)?;
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
    pub fn get_handle<T: Object>(&self, object: &T) -> Option<Handle<T>> {
        self.stores
            .get::<T>()
            .iter()
            .find(|obj| &obj.get() == object)
    }

    /// Get handle of an identical object, if it exists, or add the object
    ///
    /// In any case, returns a handle that refers to an object that is identical
    /// to the provided object.
    pub fn get_handle_or_insert<T: Object>(
        &mut self,
        object: T,
    ) -> ValidationResult<T> {
        if let Some(handle) = self.get_handle(&object) {
            return Ok(handle);
        }

        self.insert(object)
    }

    /// Merge the provided object into the shape
    ///
    /// The provided object is inserted into the shape. Each objects it
    /// references is either also inserted, or, if the shape already contains an
    /// object that is identical, the referencing object will reference the
    /// already present object.
    ///
    /// This is done recursively.
    pub fn merge<T: Object>(&mut self, object: T) -> ValidationResult<T> {
        object.merge_into(None, self, &mut Mapping::new())
    }

    /// Update objects in the shape
    ///
    /// Returns [`Update`], and API that can be used to update objects in the
    /// shape.
    pub fn update(&mut self) -> Update {
        Update::new(self.min_distance, &mut self.stores)
    }

    /// Clone the shape
    ///
    /// Returns a [`Mapping`] that maps each object from the original shape to
    /// the respective object in the cloned shape.
    pub fn clone_shape(&self) -> (Shape, Mapping) {
        self.clone_shape_inner()
            .expect("Clone of valid shape can't be invalid")
    }

    fn clone_shape_inner(&self) -> Result<(Shape, Mapping), ValidationError> {
        let mut target = Shape::new();
        let mut mapping = Mapping::new();

        for original in self.points() {
            let cloned = target.merge(original.get())?;
            mapping.points.insert(original, cloned);
        }
        for original in self.curves() {
            let cloned = target.merge(original.get())?;
            mapping.curves.insert(original, cloned);
        }
        for original in self.surfaces() {
            let cloned = target.merge(original.get())?;
            mapping.surfaces.insert(original, cloned);
        }
        for original in self.vertices() {
            let cloned = target.merge(original.get())?;
            mapping.vertices.insert(original, cloned);
        }
        for original in self.edges() {
            let cloned = target.merge(original.get())?;
            mapping.edges.insert(original, cloned);
        }
        for original in self.cycles() {
            let cloned = target.merge(original.get())?;
            mapping.cycles.insert(original, cloned);
        }
        for original in self.faces() {
            let cloned = target.merge(original.get())?;
            mapping.faces.insert(original, cloned);
        }

        Ok((target, mapping))
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
    pub fn curves(&self) -> Iter<Curve<3>> {
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
    pub fn vertices(&self) -> Iter<Vertex<3>> {
        self.stores.vertices.iter()
    }

    /// Access iterator over all edges
    ///
    /// The caller must not make any assumptions about the order of edges.
    pub fn edges(&self) -> Iter<Edge<3>> {
        self.stores.edges.iter()
    }

    /// Access an iterator over all cycles
    ///
    /// The caller must not make any assumptions about the order of cycles.
    pub fn cycles(&self) -> Iter<Cycle<3>> {
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

        let vertex = Vertex::new(point);
        let edge = Edge::new(curve, None);

        assert!(shape.get_handle(&vertex).is_none());
        assert!(shape.get_handle(&edge).is_none());

        let vertex = shape.insert(vertex)?;
        let edge = shape.insert(edge)?;

        assert!(shape.get_handle(&vertex.get()).as_ref() == Some(&vertex));
        assert!(shape.get_handle(&edge.get()).as_ref() == Some(&edge));

        let cycle = Cycle::new(vec![edge]);
        assert!(shape.get_handle(&cycle).is_none());

        let cycle = shape.insert(cycle)?;
        assert!(shape.get_handle(&cycle.get()).as_ref() == Some(&cycle));

        let face = Face::new(surface, Vec::new(), Vec::new(), [0, 0, 0, 0]);
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
        shape.insert(Vertex::new(point))?;

        // Should fail, as `point` is not part of the shape.
        let point = other.insert(Point::from([1., 0., 0.]))?;
        let result = shape.insert(Vertex::new(point));
        assert!(matches!(result, Err(ValidationError::Structural(_))));

        // `point` is too close to the original point. `assert!` is commented,
        // because that only causes a warning to be logged right now.
        let point = shape.insert(Point::from([5e-8, 0., 0.]))?;
        let result = shape.insert(Vertex::new(point));
        assert!(matches!(result, Err(ValidationError::Uniqueness)));

        // `point` is farther than `MIN_DISTANCE` away from original point.
        // Should work.
        let point = shape.insert(Point::from([5e-6, 0., 0.]))?;
        shape.insert(Vertex::new(point))?;

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
            .insert(Edge::new(curve.clone(), Some([a.clone(), b.clone()])))
            .unwrap_err();
        assert!(err.missing_curve(&curve));
        assert!(err.missing_vertex(&a));
        assert!(err.missing_vertex(&b));

        let curve = shape.add_curve();
        let a = Vertex::builder(&mut shape).build_from_point([1., 0., 0.])?;
        let b = Vertex::builder(&mut shape).build_from_point([2., 0., 0.])?;

        // Everything has been added to `shape` now. Should work!
        shape.insert(Edge::new(curve, Some([a, b])))?;

        Ok(())
    }

    #[test]
    fn add_cycle() -> anyhow::Result<()> {
        let mut shape = TestShape::new();
        let mut other = TestShape::new();

        // Trying to refer to edge that is not from the same shape. Should fail.
        let edge = other.add_edge()?;
        let err = shape.insert(Cycle::new(vec![edge.clone()])).unwrap_err();
        assert!(err.missing_edge(&edge));

        // Referring to edge that *is* from the same shape. Should work.
        let edge = shape.add_edge()?;
        shape.insert(Cycle::new(vec![edge]))?;

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
            .insert(Face::new(
                surface.clone(),
                vec![cycle.clone()],
                Vec::new(),
                [255, 0, 0, 255],
            ))
            .unwrap_err();
        assert!(err.missing_surface(&surface));
        assert!(err.missing_cycle(&cycle));

        let surface = shape.add_surface();
        let cycle = shape.add_cycle()?;

        // Everything has been added to `shape` now. Should work!
        shape.insert(Face::new(
            surface,
            vec![cycle],
            Vec::new(),
            [255, 0, 0, 255],
        ))?;

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

        fn add_curve(&mut self) -> Handle<Curve<3>> {
            self.insert(Curve::x_axis()).unwrap()
        }

        fn add_surface(&mut self) -> Handle<Surface> {
            self.insert(Surface::xy_plane()).unwrap()
        }

        fn add_edge(&mut self) -> anyhow::Result<Handle<Edge<3>>> {
            let points = [(); 2].map(|()| {
                let point = self.next_point;
                self.next_point.x += Scalar::ONE;
                point
            });
            let edge = Edge::builder(&mut self.inner)
                .build_line_segment_from_points(points)?;

            Ok(edge)
        }

        fn add_cycle(&mut self) -> anyhow::Result<Handle<Cycle<3>>> {
            let edge = self.add_edge()?;
            let cycle = self.insert(Cycle::new(vec![edge]))?;
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
