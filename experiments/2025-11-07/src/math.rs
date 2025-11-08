/// # A point
pub struct Point<const N: usize> {
    pub coordinates: [Scalar; N],
}

/// # A scalar value
pub struct Scalar {
    pub value: f64,
}
