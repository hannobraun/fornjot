use crate::objects::{Curve, Cycle, Edge, Face, Surface, Vertex};

use super::{
    stores::{Store, Stores},
    Handle, Iter, Object,
};

/// The boundary representation of a shape
#[derive(Clone, Debug)]
pub struct Shape {
    stores: Stores,
}

impl Shape {
    /// Construct a new shape
    pub fn new() -> Self {
        Self {
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

    /// Insert an object into the shape
    ///
    /// Validates the object, and returns an error if it is not valid. See the
    /// documentation of each object for validation requirements.
    pub fn insert<T: Object>(&mut self, object: T) -> Handle<T> {
        self.stores.get::<T>().insert(object)
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
    pub fn get_handle_or_insert<T: Object>(&mut self, object: T) -> Handle<T> {
        if let Some(handle) = self.get_handle(&object) {
            return handle;
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
    pub fn merge<T: Object>(&mut self, object: T) -> Handle<T> {
        object.merge_into(self)
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
    };

    #[test]

    fn get_handle() {
        let mut shape = Shape::new();

        let point = Point::from([1., 0., 0.]);
        let curve = Curve::x_axis();
        let surface = Surface::xy_plane();

        assert!(shape.get_handle(&curve).is_none());
        assert!(shape.get_handle(&surface).is_none());

        let curve = shape.insert(curve);
        let surface = shape.insert(surface);

        assert!(shape.get_handle(&curve.get()).as_ref() == Some(&curve));
        assert!(shape.get_handle(&surface.get()).as_ref() == Some(&surface));

        let vertex = Vertex { point };
        let edge = Edge {
            curve: LocalForm::canonical_only(curve.get()),
            vertices: VerticesOfEdge::none(),
        };

        assert!(shape.get_handle(&vertex).is_none());
        assert!(shape.get_handle(&edge).is_none());

        let vertex = shape.insert(vertex);
        let edge = shape.insert(edge);

        assert!(shape.get_handle(&vertex.get()).as_ref() == Some(&vertex));
        assert!(shape.get_handle(&edge.get()).as_ref() == Some(&edge));

        let cycle = Cycle::new(vec![edge.get()]);
        assert!(shape.get_handle(&cycle).is_none());

        let cycle = shape.insert(cycle);
        assert!(shape.get_handle(&cycle.get()).as_ref() == Some(&cycle));

        let face =
            Face::new(surface.get(), Vec::new(), Vec::new(), [0, 0, 0, 0]);
        assert!(shape.get_handle(&face).is_none());

        let face = shape.insert(face);
        assert!(shape.get_handle(&face.get()).as_ref() == Some(&face));
    }
}
