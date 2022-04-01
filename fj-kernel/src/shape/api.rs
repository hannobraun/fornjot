use std::marker::PhantomData;

use fj_math::Scalar;

use super::{
    stores::{
        Curves, Cycles, Edges, Faces, Points, Stores, Surfaces, Vertices,
    },
    Geometry, Object, Topology, ValidationResult,
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
