/// # A point
pub struct Point<const N: usize> {
    pub coordinates: Vector<N>,
}

/// # A vector
pub struct Vector<const N: usize> {
    pub components: [Scalar; N],
}

/// # A scalar value
pub struct Scalar {
    pub value: f64,
}

/// # An axis-aligned bounding box
pub struct Aabb<const D: usize> {
    pub min: Point<D>,
    pub max: Point<D>,
}
