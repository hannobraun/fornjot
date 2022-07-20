use fj_interop::mesh::Color;
use fj_math::Triangle;

use crate::builder::FaceBuilder;

use super::{Cycle, Surface};

/// A face of a shape
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Face {
    /// A face of a shape
    ///
    /// A face is defined by a surface, and is bounded by edges that lie in that
    /// surface.
    BRep(FaceBRep),

    /// The triangles of the face
    ///
    /// Representing faces as a collection of triangles is a temporary state.
    /// The plan is to eventually represent faces as a geometric surface,
    /// bounded by edges. While the transition is being made, this variant is
    /// still required.
    Triangles(Vec<(Triangle<3>, Color)>),
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

        Self::BRep(FaceBRep {
            surface,
            exteriors,
            interiors,
            color,
        })
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

    /// Access the boundary representation of the face
    fn brep(&self) -> &FaceBRep {
        if let Self::BRep(face) = self {
            return face;
        }

        // No code that still uses triangle representation is calling this
        // method.
        unreachable!()
    }
}

/// The boundary representation of a face
///
/// This type exists to ease the handling of faces that use boundary
/// representation. It will eventually be merged into `Face`, once
/// `Face::Triangles` can finally be removed.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct FaceBRep {
    surface: Surface,
    exteriors: Vec<Cycle>,
    interiors: Vec<Cycle>,
    color: [u8; 4],
}
