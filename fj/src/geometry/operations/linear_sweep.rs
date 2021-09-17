/// A 3-dimensional sweep of a 2-dimensional sketch along a straight path
pub struct LinearSweep<Sketch> {
    /// The sketch being swept
    pub sketch: Sketch,

    /// The distance that the sketch is being swept through space
    pub distance: f32,
}

impl<Sketch> LinearSweep<Sketch> {
    /// Update sketch
    ///
    /// Returns a copy of `self` with the sketch replaced with `sketch`.
    pub fn with_sketch(mut self, sketch: Sketch) -> Self {
        self.sketch = sketch;
        self
    }

    /// Update height
    ///
    /// Returns a copy of `self` with the height replaced with `height`.
    pub fn with_distance(mut self, distance: f32) -> Self {
        self.distance = distance;
        self
    }
}

// `Sweep` is covered by a bunch of unit tests in `cylinder`.
