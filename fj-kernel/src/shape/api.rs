use std::marker::PhantomData;

use fj_math::Scalar;

use super::{
    stores::{
        Curves, Cycles, Edges, Faces, Points, Stores, Surfaces, Vertices,
    },
    Geometry, Handle, Object, Topology, ValidationResult,
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

    /// Access the shape's geometry
    pub fn geometry(&mut self) -> Geometry {
        Geometry {
            points: &mut self.stores.points,
            curves: &mut self.stores.curves,
            surfaces: &mut self.stores.surfaces,

            faces: &mut self.stores.faces,
        }
    }

    /// Access the shape's topology
    pub fn topology(&mut self) -> Topology {
        Topology {
            stores: self.stores.clone(),
            _lifetime: PhantomData,
        }
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
        geometry::{Curve, Surface},
        shape::Shape,
        topology::{Cycle, Edge, Face, Vertex},
    };

    #[test]

    fn get_handle() -> anyhow::Result<()> {
        let mut shape = Shape::new();

        let point = Point::from([1., 0., 0.]);
        let curve = Curve::x_axis();
        let surface = Surface::x_y_plane();

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
}
