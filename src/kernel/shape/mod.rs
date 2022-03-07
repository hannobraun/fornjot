pub mod cycles;
pub mod edges;
pub mod handle;
pub mod vertices;

use crate::math::{Point, Scalar};

use super::topology::{edges::Cycle, faces::Faces};

use self::{
    cycles::Cycles, edges::Edges, handle::HandleInner, vertices::Vertices,
};

/// The boundary representation of a shape
///
/// # Implementation note
///
/// The goal for `Shape` is to enforce full self-consistency, through the API it
/// provides. Steps have been made in that direction, but right now, the API is
/// still full of holes, forcing callers to just be careful for the time being.
#[derive(Clone, Debug)]
pub struct Shape {
    /// The minimum distance between two vertices
    ///
    /// Use for vertex validation, to determine whether vertices are unique.
    min_distance: Scalar,

    vertices: VerticesInner,
    edges: Edges,
    cycles: CyclesInner,

    pub faces: Faces,
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
            edges: Edges,
            cycles: CyclesInner::new(),
            faces: Faces(Vec::new()),
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

    /// Access the shape's vertices
    pub fn vertices(&mut self) -> Vertices {
        Vertices {
            min_distance: self.min_distance,
            vertices: &mut self.vertices,
        }
    }

    /// Access the shape's edges
    pub fn edges(&mut self) -> &mut Edges {
        &mut self.edges
    }

    /// Access the shape's cycles
    pub fn cycles(&mut self) -> Cycles {
        Cycles {
            cycles: &mut self.cycles,
        }
    }
}

type VerticesInner = Vec<HandleInner<Point<3>>>;
type CyclesInner = Vec<Cycle>;
