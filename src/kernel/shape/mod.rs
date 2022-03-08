pub mod curves;
pub mod cycles;
pub mod edges;
pub mod faces;
pub mod handle;
pub mod surfaces;
pub mod vertices;

use crate::math::Scalar;

use super::topology::{
    edges::{Cycle, Edge},
    faces::Face,
    vertices::Vertex,
};

use self::{
    curves::Curves, cycles::Cycles, edges::Edges, faces::Faces,
    handle::Storage, surfaces::Surfaces, vertices::Vertices,
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

    /// Access the shape's curves
    pub fn curves(&mut self) -> Curves {
        Curves
    }

    /// Access the shape's surfaces
    pub fn surfaces(&mut self) -> Surfaces {
        Surfaces
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
            curves: Curves,
            edges: &mut self.edges,
        }
    }

    /// Access the shape's cycles
    pub fn cycles(&mut self) -> Cycles {
        Cycles {
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

type VerticesInner = Vec<Storage<Vertex>>;
type EdgesInner = Vec<Storage<Edge>>;
type CyclesInner = Vec<Storage<Cycle>>;
type FacesInner = Vec<Storage<Face>>;
