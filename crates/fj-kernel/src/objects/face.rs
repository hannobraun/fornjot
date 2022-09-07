use std::collections::{btree_set, BTreeSet};

use fj_interop::mesh::Color;
use fj_math::Triangle;

use crate::builder::FaceBuilder;

use super::{Cycle, Surface};

/// A collection of faces
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Faces {
    inner: BTreeSet<Face>,
}

impl Faces {
    /// Create an empty instance of `Faces`
    pub fn new() -> Self {
        Self::default()
    }
}

impl Extend<Face> for Faces {
    fn extend<T: IntoIterator<Item = Face>>(&mut self, iter: T) {
        self.inner.extend(iter)
    }
}

impl IntoIterator for Faces {
    type Item = Face;
    type IntoIter = btree_set::IntoIter<Face>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a> IntoIterator for &'a Faces {
    type Item = &'a Face;
    type IntoIter = btree_set::Iter<'a, Face>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

/// A face of a shape
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Face {
    representation: Representation,
}

impl Face {
    /// Build a face using [`FaceBuilder`]
    pub fn build(surface: Surface) -> FaceBuilder {
        FaceBuilder::new(surface)
    }

    /// Construct a new instance of `Face`
    ///
    /// Creates the face with no exteriors, no interiors and the default color.
    /// This can be overridden using the `with_` methods.
    pub fn new(surface: Surface) -> Self {
        Self {
            representation: Representation::BRep(BRep {
                surface,
                exteriors: Vec::new(),
                interiors: Vec::new(),
                color: Color::default(),
            }),
        }
    }

    /// Construct an instance that uses triangle representation
    ///
    /// Triangle representation is obsolete, and only still present because
    /// there is one last place in the kernel code that uses it. Don't add any
    /// more of those places!
    ///
    /// See this issue for more context:
    /// <https://github.com/hannobraun/Fornjot/issues/97>
    pub fn from_triangles(triangles: TriRep) -> Self {
        Self {
            representation: Representation::TriRep(triangles),
        }
    }

    /// Add exterior cycles to the face
    ///
    /// Consumes the face and returns the updated instance.
    ///
    /// # Panics
    ///
    /// Panics, if the added cycles are not defined in the face's surface.
    pub fn with_exteriors(
        mut self,
        exteriors: impl IntoIterator<Item = Cycle>,
    ) -> Self {
        for cycle in exteriors.into_iter() {
            assert_eq!(
                self.surface(),
                cycle.surface(),
                "Cycles that bound a face must be in face's surface"
            );

            self.brep_mut().exteriors.push(cycle);
        }

        self
    }

    /// Add interior cycles to the face
    ///
    /// Consumes the face and returns the updated instance.
    ///
    /// # Panics
    ///
    /// Panics, if the added cycles are not defined in the face's surface.
    pub fn with_interiors(
        mut self,
        interiors: impl IntoIterator<Item = Cycle>,
    ) -> Self {
        for cycle in interiors.into_iter() {
            assert_eq!(
                self.surface(),
                cycle.surface(),
                "Cycles that bound a face must be in face's surface"
            );

            self.brep_mut().interiors.push(cycle);
        }

        self
    }

    /// Update the color of the face
    ///
    /// Consumes the face and returns the updated instance.
    pub fn with_color(mut self, color: Color) -> Self {
        self.brep_mut().color = color;
        self
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
    pub fn color(&self) -> Color {
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
    fn brep(&self) -> &BRep {
        if let Representation::BRep(face) = &self.representation {
            return face;
        }

        // No code that still uses triangle representation is calling this
        // method.
        unreachable!()
    }

    /// Access the boundary representation of the face mutably
    fn brep_mut(&mut self) -> &mut BRep {
        if let Representation::BRep(face) = &mut self.representation {
            return face;
        }

        // No code that still uses triangle representation is calling this
        // method.
        unreachable!()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Representation {
    BRep(BRep),
    TriRep(TriRep),
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct BRep {
    surface: Surface,
    exteriors: Vec<Cycle>,
    interiors: Vec<Cycle>,
    color: Color,
}

type TriRep = Vec<(Triangle<3>, Color)>;
