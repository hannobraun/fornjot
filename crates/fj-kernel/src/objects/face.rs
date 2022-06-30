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
    Face(FaceBRep),

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
        let exteriors = CyclesInFace::new(exteriors);
        let interiors = CyclesInFace::new(interiors);

        Self::Face(FaceBRep {
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

    /// Access the boundary representation of the face
    pub fn brep(&self) -> &FaceBRep {
        match self {
            Self::Face(face) => face,
            _ => {
                // No code that still uses triangle representation is calling
                // this method.
                unreachable!()
            }
        }
    }

    /// Access the surface that the face refers to
    ///
    /// This is a convenience method that saves the caller from dealing with the
    /// [`Handle`].
    pub fn surface(&self) -> Surface {
        self.brep().surface()
    }

    /// Access the exterior cycles that the face refers to
    ///
    /// This is a convenience method that saves the caller from dealing with the
    /// [`Handle`]s.
    pub fn exteriors(&self) -> impl Iterator<Item = Cycle> + '_ {
        self.brep().exteriors()
    }

    /// Access the interior cycles that the face refers to
    ///
    /// This is a convenience method that saves the caller from dealing with the
    /// [`Handle`]s.
    pub fn interiors(&self) -> impl Iterator<Item = Cycle> + '_ {
        self.brep().interiors()
    }

    /// Access all cycles that the face refers to
    ///
    /// This is equivalent to chaining the iterators returned by
    /// [`Face::exteriors`] and [`Face::interiors`].
    pub fn all_cycles(&self) -> impl Iterator<Item = Cycle> + '_ {
        self.exteriors().chain(self.interiors())
    }

    /// Access the color of the face
    pub fn color(&self) -> [u8; 4] {
        self.brep().color
    }
}

/// The boundary representation of a face
///
/// This type exists to ease the handling of faces that use boundary
/// representation. It will eventually be merged into `Face`, once
/// `Face::Triangles` can finally be removed.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct FaceBRep {
    /// The surface that defines this face
    pub surface: Surface,

    /// The cycles that bound the face on the outside
    ///
    /// # Implementation Note
    ///
    /// Since these cycles bound the face, the edges they consist of must
    /// lie in the surface. The data we're using here is 3-dimensional
    /// though, so no such limitation is enforced.
    ///
    /// It might be less error-prone to specify the cycles in surface
    /// coordinates.
    pub exteriors: CyclesInFace,

    /// The cycles that bound the face on the inside
    ///
    /// Each of these cycles defines a hole in the face.
    ///
    /// # Implementation note
    ///
    /// See note on `exterior` field.
    pub interiors: CyclesInFace,

    /// The color of the face
    pub color: [u8; 4],
}

impl FaceBRep {
    /// Access the surface that the face refers to
    ///
    /// This is a convenience method that saves the caller from dealing with the
    /// [`Handle`].
    pub fn surface(&self) -> Surface {
        self.surface
    }

    /// Access the exterior cycles that the face refers to
    ///
    /// This is a convenience method that saves the caller from dealing with the
    /// [`Handle`]s.
    pub fn exteriors(&self) -> impl Iterator<Item = Cycle> + '_ {
        self.exteriors.as_local()
    }

    /// Access the interior cycles that the face refers to
    ///
    /// This is a convenience method that saves the caller from dealing with the
    /// [`Handle`]s.
    pub fn interiors(&self) -> impl Iterator<Item = Cycle> + '_ {
        self.interiors.as_local()
    }

    /// Access all cycles that the face refers to
    ///
    /// This is equivalent to chaining the iterators returned by
    /// [`Face::exteriors`] and [`Face::interiors`].
    pub fn all_cycles(&self) -> impl Iterator<Item = Cycle> + '_ {
        self.exteriors().chain(self.interiors())
    }
}

/// A list of cycles, as they are stored in `Face`
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct CyclesInFace(Vec<Cycle>);

impl CyclesInFace {
    /// Create a new instance of `CyclesInFace`
    pub fn new(cycles: impl IntoIterator<Item = Cycle>) -> Self {
        Self(cycles.into_iter().collect())
    }

    /// Access an iterator over the canonical forms of the cycles
    pub fn as_local(&self) -> impl Iterator<Item = Cycle> + '_ {
        self.0.iter().cloned()
    }
}
