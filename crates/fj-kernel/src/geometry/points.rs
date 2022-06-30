/// A point that stores a local and a global form
///
/// The local form of a point is whatever representation is most appropriate in
/// the current context, which might be a curve or surface. The global form is
/// the global 3D form of the same point.
///
/// The purpose of storing both forms is to be able to losslessly convert the
/// point back to its global form. Even if this conversion can be computed on
/// the fly, such a conversion might not result in the original global form, due
/// to floating point accuracy issues. Hence, such a conversion would not be
/// lossless, which could result in bugs.
///
/// The `D` parameter defines the dimensionality of the local form.
///
/// # `LocalPoint` and [`Vertex`]
///
/// `LocalPoint` is similar to `Vertex`, but there is a key differences:
/// `Vertex` is an object in the boundary representation of a shape, while
/// `LocalPoint` can refer to any point. This distinction is important in the
/// case of approximation, for example, as points might be generated to
/// approximate a curve or surface, without those generated points referring to
/// any vertices.
///
/// [`Vertex`]: crate::objects::Vertex
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct LocalPoint<const D: usize> {
    local: fj_math::Point<D>,
    global: fj_math::Point<3>,
}

impl<const D: usize> LocalPoint<D> {
    /// Construct a new instance
    ///
    /// Both the local and the global form must be provided. The caller must
    /// guarantee that both of them match, i.e. define the same point.
    pub fn new(
        local: impl Into<fj_math::Point<D>>,
        global: impl Into<fj_math::Point<3>>,
    ) -> Self {
        Self {
            local: local.into(),
            global: global.into(),
        }
    }

    /// Access the point's local form
    pub fn local(&self) -> fj_math::Point<D> {
        self.local
    }

    /// Access the point's global form
    pub fn global(&self) -> fj_math::Point<3> {
        self.global
    }
}
