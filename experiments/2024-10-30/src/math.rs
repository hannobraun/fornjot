pub type Point = [Scalar; 3];

#[derive(Clone, Copy)]
pub struct Scalar {
    value: f64,
}

impl Scalar {
    pub fn new(value: f64) -> Self {
        if value.is_nan() {
            panic!("`Scalar` value must not be NaN");
        }
        if value.is_infinite() {
            panic!("`Scalar` value must not be infinite. Value: `{value}`");
        }

        Self { value }
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}
