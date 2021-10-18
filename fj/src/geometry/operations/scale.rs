/// Scales a shape
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Scale<T> {
    /// The shape to scale
    pub shape: T,

    // TASK: Convert this into an n-dimensional vector, to enable scaling the
    //       shape along each dimension.
    /// The factor to scale the shape by
    pub factor: f32,
}

impl<T> Scale<T> {
    pub fn with_factor(mut self, factor: f32) -> Self {
        self.factor = factor;
        self
    }
}
