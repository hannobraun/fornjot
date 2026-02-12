use fj_math::Point;

/// # A point in an approximation with both local and global representation
///
/// When creating an approximation, you often need to deal with local
/// coordinates, either 1-dimensional on a curve or 2-dimensional on a surface.
/// These local coordinates may be converted to global 3D coordinates later, or
/// may correspond to 3D points that already exist.
///
/// Either way, storing a local point together with its corresponding global
/// point is often advantageous or even necessary, and that's what this struct
/// provides.
///
/// `ApproxPoint` is generic over the dimension of its local point. Typically,
/// only `ApproxPoint<1>` and `ApproxPoint<2>` would be used.
///
/// `ApproxPoint<2>` [implements `spade::HasPosition`][`HasPosition`] and may be
/// used together with [`spade`] for a Delaunay triangulation.
///
/// [`HasPosition`]: #impl-HasPosition-for-ApproxPoint<2>
#[derive(Clone, Copy, Debug)]
pub struct ApproxPoint<const D: usize> {
    /// # The local form of the approximation point
    pub local: Point<D>,

    /// # The global form of the approximation point
    pub global: Point<3>,
}

impl spade::HasPosition for ApproxPoint<2> {
    type Scalar = f64;

    fn position(&self) -> spade::Point2<Self::Scalar> {
        let [x, y] = self.local.coords.components.map(|s| s.into_f64());
        spade::Point2 { x, y }
    }
}
