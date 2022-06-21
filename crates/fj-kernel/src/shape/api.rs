use fj_math::Scalar;

use crate::{
    objects::{Curve, Cycle, Edge, Face, Surface, Vertex},
    validation::ValidationError,
};

use super::{
    stores::{Store, Stores},
    Handle, Iter, Mapping, Object, Update, ValidationResult,
};

/// The boundary representation of a shape
#[derive(Clone, Debug)]
pub struct Shape {
    distinct_min_distance: Scalar,

    stores: Stores,
}

impl Shape {
    /// Construct a new shape
    pub fn new() -> Self {
        Self {
            // This should really come from `Self::DEFAULT_MIN_DISTANCE`, or a
            // similarly named constant. Unfortunately `Scalar::from_f64` can't
            // be `const` yet.
            distinct_min_distance: Scalar::from_f64(5e-7), // 0.5 µm

            stores: Stores {
                curves: Store::new(),
                surfaces: Store::new(),

                vertices: Store::new(),
                edges: Store::new(),
                cycles: Store::new(),
                faces: Store::new(),
            },
        }
    }

    /// Assign a label to the shape
    ///
    /// The assigned label will be part of the `Debug` representation of
    /// `Handle`s, making it way easier to understand which `Handle`s belong to
    /// which `Shape`.
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        let label = label.into();

        self.stores.curves.label = Some(label.clone());
        self.stores.surfaces.label = Some(label.clone());

        self.stores.vertices.label = Some(label.clone());
        self.stores.edges.label = Some(label.clone());
        self.stores.cycles.label = Some(label.clone());
        self.stores.faces.label = Some(label);

        self
    }

    /// Override the minimum distance between distinct objects
    ///
    /// Used for vertex validation, to determine whether vertices are unique.
    pub fn with_distinct_min_distance(
        mut self,
        distinct_min_distance: impl Into<Scalar>,
    ) -> Self {
        self.distinct_min_distance = distinct_min_distance.into();
        self
    }

    /// Insert an object into the shape
    ///
    /// Validates the object, and returns an error if it is not valid. See the
    /// documentation of each object for validation requirements.
    pub fn insert<T: Object>(&mut self, object: T) -> ValidationResult<T> {
        object.validate(None, self.distinct_min_distance, &self.stores)?;
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

    /// Merge the provided shape into this one
    ///
    /// Returns a [`Mapping`] that maps each object from the merged shape to the
    /// merged objects in this shape.
    pub fn merge_shape(
        &mut self,
        other: &Shape,
    ) -> Result<Mapping, ValidationError> {
        let mut mapping = Mapping::new();

        for object in other.curves() {
            object.get().merge_into(Some(object), self, &mut mapping)?;
        }
        for object in other.surfaces() {
            object.get().merge_into(Some(object), self, &mut mapping)?;
        }
        for object in other.vertices() {
            object.get().merge_into(Some(object), self, &mut mapping)?;
        }
        for object in other.edges() {
            object.get().merge_into(Some(object), self, &mut mapping)?;
        }
        for object in other.cycles() {
            object.get().merge_into(Some(object), self, &mut mapping)?;
        }
        for object in other.faces() {
            object.get().merge_into(Some(object), self, &mut mapping)?;
        }

        Ok(mapping)
    }

    /// Update objects in the shape
    ///
    /// Returns [`Update`], and API that can be used to update objects in the
    /// shape.
    pub fn update(&mut self) -> Update {
        Update::new(self.distinct_min_distance, &mut self.stores)
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
    pub fn vertices(&self) -> Iter<Vertex> {
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
    use fj_math::Point;

    use crate::{
        objects::{Curve, Cycle, Edge, Face, Surface, Vertex, VerticesOfEdge},
        shape::{LocalForm, Shape},
        validation::ValidationError,
    };

    #[test]

    fn get_handle() -> anyhow::Result<()> {
        let mut shape = Shape::new();

        let point = Point::from([1., 0., 0.]);
        let curve = Curve::x_axis();
        let surface = Surface::xy_plane();

        assert!(shape.get_handle(&curve).is_none());
        assert!(shape.get_handle(&surface).is_none());

        let curve = shape.insert(curve)?;
        let surface = shape.insert(surface)?;

        assert!(shape.get_handle(&curve.get()).as_ref() == Some(&curve));
        assert!(shape.get_handle(&surface.get()).as_ref() == Some(&surface));

        let vertex = Vertex { point };
        let edge = Edge {
            curve: LocalForm::canonical_only(curve),
            vertices: VerticesOfEdge::none(),
        };

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
        let mut shape = Shape::new();

        let point = Point::from([0., 0., 0.]);

        // Adding a vertex should work.
        shape.insert(Vertex { point })?;

        // Adding a second vertex with the same point should fail.
        let result = shape.insert(Vertex { point });
        assert!(matches!(result, Err(ValidationError::Uniqueness(_))));

        Ok(())
    }

    #[test]
    fn add_edge_uniqueness() -> anyhow::Result<()> {
        let mut shape = Shape::new();

        let a = Vertex::builder(&mut shape).build_from_point([0., 0., 0.])?;
        let b = Vertex::builder(&mut shape).build_from_point([1., 0., 0.])?;

        Edge::builder(&mut shape)
            .build_line_segment_from_vertices([a.clone(), b.clone()])?;

        // Should fail. An edge with the same vertices has already been added.
        let result = Edge::builder(&mut shape)
            .build_line_segment_from_vertices([a.clone(), b.clone()]);
        assert!(matches!(result, Err(ValidationError::Uniqueness(_))));

        // Should fail. An edge with the same vertices has already been added,
        // just the order is different.
        let result =
            Edge::builder(&mut shape).build_line_segment_from_vertices([b, a]);
        assert!(matches!(result, Err(ValidationError::Uniqueness(_))));

        Ok(())
    }
}
