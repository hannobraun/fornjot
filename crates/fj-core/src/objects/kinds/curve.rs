/// A curve
///
/// `Curve` represents a curve in space, but holds no data to define that curve.
/// It is referenced by [`HalfEdge`], which defines the curve in the coordinates
/// of its surface.
///
/// `Curve` exists to allow identifying which [`HalfEdge`]s are supposed to be
/// coincident in global space.
///
/// # Equality
///
/// `Curve` contains no data and exists purely to be referenced via a `Handle`,
/// where `Handle::id` can be used to compare different instances of `Curve`.
///
/// If `Curve` had `Eq`/`PartialEq` implementations, it containing no data would
/// mean that all instances of `Curve` would be considered equal. This would be
/// very error-prone.
///
/// If you need to reference a `Curve` from a struct that needs to derive
/// `Eq`/`Ord`/..., you can use `HandleWrapper<Curve>` to do that. It will use
/// `Handle::id` to provide those `Eq`/`Ord`/... implementations.
///
/// [`HalfEdge`]: crate::objects::HalfEdge
#[derive(Clone, Debug, Default, Hash)]
pub struct Curve {}

impl Curve {
    /// Create a new instance
    pub fn new() -> Self {
        Self::default()
    }
}
