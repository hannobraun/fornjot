use fj_interop::mesh::Color;
use fj_math::Triangle;

use crate::builder::FaceBuilder;

use super::{Cycle, Surface};

/// A face of a shape
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Face {
    representation: Representation,
}

impl Face {
    /// Construct a new instance of `Face`
    pub fn new(
        surface: Surface,
        exteriors: impl IntoIterator<Item = Cycle>,
        interiors: impl IntoIterator<Item = Cycle>,
        color: [u8; 4],
    ) -> Self {
        let exteriors = exteriors.into_iter().collect();
        let interiors = interiors.into_iter().collect();

        Self {
            representation: Representation::BRep(FaceBRep {
                surface,
                exteriors,
                interiors,
                color,
            }),
        }
    }

    /// Contact an instance that uses triangle representation
    pub fn from_triangles(triangles: TriRep) -> Self {
        Self {
            representation: Representation::TriRep(triangles),
        }
    }

    /// Build a face using the [`FaceBuilder`] API
    pub fn builder(surface: Surface) -> FaceBuilder {
        FaceBuilder::new(surface)
    }

    /// Access this face's surface
    pub fn surface(&self) -> &Surface {
        &self.brep().surface
    }

    /// Access the cycles that bound the face on the outside
    pub fn exteriors(&self) -> impl Iterator<Item = &Cycle> + '_ {
        self.brep().exteriors.iter()
    }

    /// Access the cycles that bound the face on the inside
    ///
    /// Each of these cycles defines a hole in the face.
    pub fn interiors(&self) -> impl Iterator<Item = &Cycle> + '_ {
        self.brep().interiors.iter()
    }

    /// Access all cycles of this face
    ///
    /// This is equivalent to chaining the iterators returned by
    /// [`Face::exteriors`] and [`Face::interiors`].
    pub fn all_cycles(&self) -> impl Iterator<Item = &Cycle> + '_ {
        self.exteriors().chain(self.interiors())
    }

    /// Access the color of the face
    pub fn color(&self) -> [u8; 4] {
        self.brep().color
    }

    /// Access triangles, if this face uses triangle representation
    ///
    /// Only some faces still use triangle representation. At some point, none
    /// will. This method exists as a workaround, while the transition is still
    /// in progress.
    pub fn triangles(&self) -> Option<&TriRep> {
        if let Representation::TriRep(triangles) = &self.representation {
            return Some(triangles);
        }

        None
    }

    /// Access the boundary representation of the face
    fn brep(&self) -> &FaceBRep {
        if let Representation::BRep(face) = &self.representation {
            return face;
        }

        // No code that still uses triangle representation is calling this
        // method.
        unreachable!()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Representation {
    BRep(FaceBRep),
    TriRep(TriRep),
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct FaceBRep {
    surface: Surface,
    exteriors: Vec<Cycle>,
    interiors: Vec<Cycle>,
    color: [u8; 4],
}

type TriRep = Vec<(Triangle<3>, Color)>;
