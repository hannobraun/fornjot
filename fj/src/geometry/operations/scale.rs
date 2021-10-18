/// Scales a shape
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Scale<T> {
    /// The shape to scale
    pub shape: T,

    /// The factor to scale the shape by
    pub factor: f32,
}
