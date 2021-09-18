/// A 3-dimensional sweep of a 2-dimensional sketch along a straight path
pub struct LinearSweep<Shape> {
    /// The sketch being swept
    pub sketch: Shape,

    /// The distance that the sketch is being swept through space
    pub distance: f32,
}

impl<Shape> LinearSweep<Shape> {
    /// Update sketch
    ///
    /// Returns a copy of `self` with the sketch replaced with `sketch`.
    pub fn with_sketch(mut self, sketch: Shape) -> Self {
        self.sketch = sketch;
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
