/// An n-dimensional hypersphere
///
/// `Hypersphere` is typically used through one of its type aliases, like
/// [`Circle`] or [`Sphere`].
///
/// [`Circle`]: crate::geometry::shapes::Circle
/// [`Sphere`]: crate::geometry::shapes::Sphere
#[derive(Default)]
pub struct Hypersphere<const D: usize> {
    /// The radius of the hypersphere
    pub radius: f32,
}

impl<const D: usize> Hypersphere<D> {
    /// Create a new hypersphere
    ///
    /// The radius is initially set to `1.0`.
    pub fn new() -> Self {
        Self { radius: 1.0 }
    }

    /// Create a new hypersphere with the given radius
    pub fn from_radius(radius: f32) -> Self {
        let mut hypersphere = Self::new();
        hypersphere.radius = radius;
        hypersphere
    }

    /// Update radius
    ///
    /// Returns a copy of `self`, with the radius replaced with `radius`.
    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }
}
