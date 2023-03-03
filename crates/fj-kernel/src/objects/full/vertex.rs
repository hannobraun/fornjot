/// A vertex, defined in global (3D) coordinates
///
/// This struct exists to distinguish between vertices and points at the type
/// level. This is a relevant distinction, as vertices are part of a shape that
/// help define its topology.
///
/// Points, on the other hand, might be used to approximate a shape for various
/// purposes, without presenting any deeper truth about the shape's structure.
///
/// # Validation
///
/// Vertices must be unique within a shape, meaning an identical vertex must not
/// exist in the same shape. In the context of vertex uniqueness, points that
/// are close to each other are considered identical. The minimum distance
/// between distinct vertices can be configured using the respective field in
/// [`ValidationConfig`].
///
/// [`ValidationConfig`]: crate::validate::ValidationConfig
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Vertex {}

impl Vertex {
    /// Construct a `Vertex` from a position
    pub fn new() -> Self {
        Self::default()
    }
}
