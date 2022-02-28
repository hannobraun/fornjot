use crate::{
    kernel::geometry::{self, Curve},
    math::{self, Transform},
};

/// The vertices of a shape
#[derive(Clone)]
pub struct Vertices(pub Vec<Vertex<3>>);

/// A vertex
///
/// This struct exists to distinguish between vertices and points at the type
/// level. This is a relevant distinction, as vertices are part of a shape that
/// help define its topology.
///
/// Points, on the other hand, might be used to approximate a shape for various
/// purposes, without presenting any deeper truth about the shape's structure.
///
/// # Uniqueness
///
/// You **MUST NOT** construct a new instance of `Vertex` that represents an
/// already existing vertex. If there already exists a vertex and you need a
/// `Vertex` instance to refer to it, acquire one by copying or converting the
/// existing `Vertex` instance.
///
/// Every time you create a `Vertex` instance, you might do so using a point you
/// have computed. When doing this for an existing vertex, you run the risk of
/// computing a slightly different point, due to floating point accuracy issues.
/// The resulting `Vertex` will then no longer be equal to the existing `Vertex`
/// instance that refers to the same vertex, which will cause bugs.
///
/// This can be prevented outright by never creating a new `Vertex` instance
/// for an existing vertex. Hence why this is strictly forbidden.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Vertex<const D: usize>(geometry::Point<D>);

impl<const D: usize> Vertex<D> {
    /// Create a vertex at the given location
    ///
    /// You **MUST NOT** use this method to construct a new instance of `Vertex`
    /// that represents an already existing vertex. See documentation of
    /// [`Vertex`] for more information.
    pub fn new(point: impl Into<geometry::Point<D>>) -> Self {
        Self(point.into())
    }

    /// Access the location of this vertex
    pub fn point(&self) -> &math::Point<D> {
        &self.0
    }

    /// Convert the vertex to its canonical form
    pub fn to_canonical(self) -> Vertex<3> {
        Vertex(geometry::Point::new(self.0.canonical(), self.0.canonical()))
    }
}

impl Vertex<1> {
    /// Create a transformed vertex
    ///
    /// You **MUST NOT** use this method to construct a new instance of `Vertex`
    /// that represents an already existing vertex. See documentation of
    /// [`Vertex`] for more information.
    ///
    /// This is a 3D transformation that transforms the canonical form of the
    /// vertex, but leaves the location untouched. Since `self` is a
    /// 1-dimensional vertex, transforming the location is not possible.
    ///
    /// And, presumably, also not necessary, as this is likely part of a larger
    /// transformation that also transforms the curve the vertex is on. Making
    /// sure this is the case, is the responsibility of the caller.
    #[must_use]
    pub fn transform(mut self, transform: &Transform) -> Self {
        self.0 = geometry::Point::new(
            self.0.native(),
            transform.transform_point(&self.0.canonical()),
        );
        self
    }
}

impl Vertex<3> {
    /// Convert the vertex to a 1-dimensional vertex
    ///
    /// Uses to provided curve to convert the vertex into a 1-dimensional vertex
    /// in the curve's coordinate system.
    pub fn to_1d(self, curve: &Curve) -> Vertex<1> {
        let location = curve.point_model_to_curve(&self.0);

        Vertex(geometry::Point::new(location, self.0.canonical()))
    }
}
