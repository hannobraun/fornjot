use std::cmp::Ordering;

pub type Point = [Scalar; 3];

#[derive(Clone, Copy, Debug, PartialEq)]
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

impl Ord for Scalar {
    fn cmp(&self, other: &Self) -> Ordering {
        let Some(ordering) = self.value.partial_cmp(&other.value) else {
            unreachable!(
                "Failed to compare `Scalar` values `{}` and `{}`",
                self.value, other.value
            );
        };

        ordering
    }
}

impl PartialOrd for Scalar {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<f64> for Scalar {
    fn from(value: f64) -> Self {
        Self::new(value)
    }
}
