/// A sweep of a shape along a straight path
pub struct LinearSweep<T> {
    /// The shape being swept
    pub shape: T,

    /// The distance that the shape is being swept through space
    pub distance: f32,
}

impl<Shape> LinearSweep<Shape> {
    /// Update shape
    ///
    /// Returns a copy of `self` with the shape replaced with `shape`.
    pub fn with_shape(mut self, shape: Shape) -> Self {
        self.shape = shape;
        self
    }

    /// Update distance
    ///
    /// Returns a copy of `self` with the distance replaced with `distance`.
    pub fn with_distance(mut self, distance: f32) -> Self {
        self.distance = distance;
        self
    }
}

// `LinearSweep` is covered by a bunch of unit tests in `cylinder`.
