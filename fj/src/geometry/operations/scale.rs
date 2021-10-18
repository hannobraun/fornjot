/// Scales a shape
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Scale<T> {
    pub shape: T,
    pub factor: f32,
}
