use std::cmp::Ordering;

pub type Point = [Scalar; 3];

#[derive(Clone, Copy, PartialEq)]
pub struct Scalar {
    value: f64,
}

impl Scalar {
    /// # Create a new instance of [`Scalar`]
    ///
    /// ## Panics
    ///
    /// Panics, if `value` is NaN or infinite.
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

impl Eq for Scalar {}

impl PartialOrd for Scalar {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}
