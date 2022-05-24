use std::fmt;

use fj_math::Point;

use crate::{
    geometry::Curve,
    shape::{Handle, LocalForm, Shape},
};

use super::{EdgeBuilder, Vertex};

/// An edge of a shape
///
/// # Equality
///
/// Please refer to [`crate::kernel::topology`] for documentation on the
/// equality of topological objects.
///
/// # Validation
///
/// An edge that is part of a [`Shape`] must be structurally sound. That means
/// the curve and any vertices that it refers to, must be part of the same
/// shape.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Edge<const D: usize> {
    /// Access the curve that defines the edge's geometry
    ///
    /// The edge can be a segment of the curve that is bounded by two vertices,
    /// or if the curve is continuous (i.e. connects to itself), the edge could
    /// be defined by the whole curve, and have no bounding vertices.
    pub curve: LocalForm<Curve<D>, Curve<3>>,

    /// Access the vertices that bound the edge on the curve
    ///
    /// If there are no such vertices, that means that both the curve and the
    /// edge are continuous (i.e. connected to themselves).
    pub vertices: Option<[LocalForm<Point<1>, Vertex>; 2]>,
}

impl Edge<3> {
    /// Construct an instance of `Edge`
    ///
    /// # Implementation Note
    ///
    /// This method accepts two `Handle<Vertex>`, and projects them onto the
    /// curve, to create the local representation that is stored in `Edge`. This
    /// could lead to incorrect results, as the caller could provide vertices
    /// that don't actually lie on `curve`.
    ///
    /// The good news is, that whole thing seems to be unnecessary. Or rather,
    /// this whole method seems to be unnecessary. I reviewed all the call
    /// sites, and in all cases, there seems to be a better way to construct the
    /// `Edge`, without using this constructor.
    ///
    /// Ideally, we'd just fix all those call sites and remove this method, to
    /// completely avoid the potential for any bugs to occur here. Problem is,
    /// some of those call sites can't be fixed easily, without building further
    /// infrastructure. Here's one such piece of infrastructure, for which an
    /// issue already exists:
    /// <https://github.com/hannobraun/Fornjot/issues/399>
    pub fn new(
        curve: Handle<Curve<3>>,
        vertices: Option<[Handle<Vertex>; 2]>,
    ) -> Self {
        let curve = LocalForm::canonical_only(curve);

        let vertices = vertices.map(|vertices| {
            vertices.map(|canonical| {
                let local = curve
                    .canonical()
                    .get()
                    .point_to_curve_coords(canonical.get().point())
                    .local();
                LocalForm::new(local, canonical)
            })
        });

        Self { curve, vertices }
    }

    /// Build an edge using the [`EdgeBuilder`] API
    pub fn builder(shape: &mut Shape) -> EdgeBuilder {
        EdgeBuilder::new(shape)
    }
}

impl<const D: usize> Edge<D> {
    /// Access the curve that the edge refers to
    ///
    /// This is a convenience method that saves the caller from dealing with the
    /// [`Handle`].
    pub fn curve(&self) -> Curve<3> {
        self.curve.canonical().get()
    }

    /// Access the vertices that the edge refers to
    ///
    /// This is a convenience method that saves the caller from dealing with the
    /// [`Handle`]s.
    pub fn vertices(&self) -> Option<[Vertex; 2]> {
        self.vertices
            .as_ref()
            .map(|[a, b]| [a.canonical().get(), b.canonical().get()])
    }
}

impl<const D: usize> fmt::Display for Edge<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.vertices() {
            Some(vertices) => {
                let [a, b] = vertices.map(|vertex| vertex.point());
                write!(f, "edge from {:?} to {:?}", a, b)?
            }
            None => write!(f, "continuous edge")?,
        }

        write!(f, " on {}", self.curve())?;

        Ok(())
    }
}
