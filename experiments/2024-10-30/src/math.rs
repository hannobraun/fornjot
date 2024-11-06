pub type Point = [Scalar; 3];

#[derive(Clone, Copy)]
pub struct Scalar {
    value: f64,
}

impl Scalar {
    pub fn new(value: f64) -> Self {
        Self { value }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}
