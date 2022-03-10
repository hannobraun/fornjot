pub mod cycles;
pub mod edges;
pub mod faces;
pub mod geometry;
pub mod handle;
pub mod vertices;

use crate::math::Scalar;

use super::topology::{
    edges::{Cycle, Edge},
    faces::Face,
    vertices::Vertex,
};

use self::{
    cycles::Cycles,
    edges::Edges,
    faces::Faces,
    geometry::Geometry,
    handle::{Handle, Storage},
    vertices::Vertices,
};

/// The boundary representation of a shape
#[derive(Clone, Debug)]
pub struct Shape {
    /// The minimum distance between two vertices
    ///
    /// Use for vertex validation, to determine whether vertices are unique.
    min_distance: Scalar,

    vertices: VerticesInner,
    edges: EdgesInner,
    cycles: CyclesInner,
    faces: FacesInner,
}

impl Shape {
    /// Construct a new shape
    pub fn new() -> Self {
        Self {
            // This should really come from `Self::DEFAULT_MIN_DISTANCE`, or a
            // similarly named constant. Unfortunately `Scalar::from_f64` can't
            // be `const` yet.
            min_distance: Scalar::from_f64(5e-7), // 0.5 µm

            vertices: VerticesInner::new(),
            edges: EdgesInner::new(),
            cycles: CyclesInner::new(),
            faces: FacesInner::new(),
        }
    }

    /// Override the minimum distance for this shape
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

    /// Access the shape's geometry
    pub fn geometry(&mut self) -> Geometry {
        Geometry
    }

    /// Access the shape's vertices
    pub fn vertices(&mut self) -> Vertices {
        Vertices {
            min_distance: self.min_distance,
            vertices: &mut self.vertices,
        }
    }

    /// Access the shape's edges
    pub fn edges(&mut self) -> Edges {
        Edges {
            geometry: Geometry,
            vertices: &mut self.vertices,
            edges: &mut self.edges,
        }
    }

    /// Access the shape's cycles
    pub fn cycles(&mut self) -> Cycles {
        Cycles {
            edges: &mut self.edges,
            cycles: &mut self.cycles,
        }
    }

    /// Access the shape's faces
    pub fn faces(&mut self) -> Faces {
        Faces {
            faces: &mut self.faces,
        }
    }
}

/// Returned by the various `add_` methods of the [`Shape`] API
pub type ValidationResult<T> = Result<Handle<T>, ValidationError>;

/// An error that can occur during a validation
#[derive(Debug)]
pub enum ValidationError {
    /// Structural validation failed
    ///
    /// Structural validation verifies, that all the object that an object
    /// refers to are already part of the shape.
    #[allow(unused)]
    Structural,

    /// Uniqueness validation failed
    ///
    /// Uniqueness validation checks, that an object is unique. Uniqueness is
    /// only required for topological objects, as there's no harm in geometric
    /// objects being duplicated.
    #[allow(unused)]
    Uniqueness,

    /// Geometric validation failed
    ///
    /// Geometric validation checks, that various geometric constraints of an
    /// object are upheld. For example, edges or faces might not be allowed to
    /// intersect.
    #[allow(unused)]
    Geometric,
}

type VerticesInner = Vec<Storage<Vertex>>;
type EdgesInner = Vec<Storage<Edge>>;
type CyclesInner = Vec<Storage<Cycle>>;
type FacesInner = Vec<Storage<Face>>;
