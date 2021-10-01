use crate::math::Point;

pub struct Triangle<const D: usize>([Point<D>; 3]);

impl<const D: usize> Triangle<D> {
    /// Create a new `Triangle`
    pub fn new([a, b, c]: [Point<D>; 3]) -> Option<Self> {
        if a == b || a == c || b == c {
            return None;
        }

        // TASK: Make sure that points form a triangle.
        // TASK: Normalize triangle.
        Some(Self([a, b, c]))
    }

    /// Return the points of the triangle
    pub fn points(&self) -> [Point<D>; 3] {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::point;

    use crate::geometry::shapes::Triangle;

    #[test]
    fn test_triangle_new() {
        let triangle =
            Triangle::new([point![0., 0.], point![0., 1.], point![1., 1.]]);
        let collapsed_points =
            Triangle::new([point![0., 0.], point![1., 1.], point![1., 1.]]);

        assert!(triangle.is_some());
        assert!(collapsed_points.is_none());
    }
}
