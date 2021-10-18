use crate::geometry::operations::Scale;

/// An n-dimensional hypersphere
///
/// `Hypersphere` is typically used through one of its type aliases, like
/// [`Circle`] or [`Sphere`].
///
/// [`Circle`]: crate::geometry::shapes::Circle
/// [`Sphere`]: crate::geometry::shapes::Sphere
#[derive(Default)]
pub struct Hypersphere<const D: usize>;

impl<const D: usize> Hypersphere<D> {
    // TASK: Update documentation.
    /// Create a new hypersphere
    ///
    /// The radius is initially set to `1.0`.
    pub fn new() -> Self {
        Self
    }

    /// Create a new hypersphere with the given radius
    pub fn from_radius(radius: f32) -> Scale<Self> {
        Scale {
            shape: Self::new(),
            factor: radius,
        }
    }
}
