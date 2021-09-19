/// A sweep of a shape along a path
///
/// This struct is generic over `Path`, which can be any type that describes a
/// path through space. Examples of this would be a vector, or a type describing
/// some kind of curve.
///
/// As of this writing, a lot of operations are implemented for `Sweep<T, f32>`,
/// and just sweep 2-dimensional shapes along the z-axis, with the `f32`
/// representing the distance.
pub struct Sweep<T, Path> {
    /// The shape being swept
    pub shape: T,

    /// The distance that the shape is being swept through space
    pub path: Path,
}

impl<T, Path> Sweep<T, Path> {
    /// Update shape
    ///
    /// Returns a copy of `self` with the shape replaced with `shape`.
    pub fn with_shape(mut self, shape: T) -> Self {
        self.shape = shape;
        self
    }

    /// Update distance
    ///
    /// Returns a copy of `self` with the distance replaced with `distance`.
    pub fn with_path(mut self, path: Path) -> Self {
        self.path = path;
        self
    }
}

// `LinearSweep` is covered by a bunch of unit tests in `cylinder`.
