pub type Point = [Scalar; 3];

#[derive(Clone, Copy)]
pub struct Scalar {
    pub inner: f64,
}

impl Scalar {
    pub fn new(value: f64) -> Self {
        Self { inner: value }
    }

    pub fn value(&self) -> f64 {
        self.inner
    }
}
