use crate::{Point, Scalar, Vector};

/// An n-dimensional circle
///
/// The dimensionality of the circle is defined by the const generic `D`
/// parameter.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Circle<const D: usize> {
    /// The center point of the circle
    pub center: Point<D>,

    /// A vector from the center to the starting point of the circle
    ///
    /// The length of this vector defines the circle radius. Please also refer
    /// to the documentation of `b`.
    pub a: Vector<D>,

    /// A second vector that defines the plane of the circle
    ///
    /// The vector must be of equal length to `a` (the circle radius) and must
    /// be perpendicular to it. Code working with circles might assume that
    /// these conditions are met.
    pub b: Vector<D>,
}

impl<const D: usize> Circle<D> {
    /// Construct a circle
    pub fn new(
        center: impl Into<Point<D>>,
        a: impl Into<Vector<D>>,
        b: impl Into<Vector<D>>,
    ) -> Self {
        let center = center.into();
        let a = a.into();
        let b = b.into();

        Self { center, a, b }
    }

    /// Access the center point of the circle
    pub fn center(&self) -> Point<D> {
        self.center
    }

    /// Access the vector that defines the starting point of the circle
    ///
    /// The point where this vector points from the circle center, is the zero
    /// coordinate of the circle's coordinate system. The length of the vector
    /// defines the circle's radius.
    ///
    /// Please also refer to [`Self::b`].
    pub fn a(&self) -> Vector<D> {
        self.a
    }

    /// Access the vector that defines the plane of the circle
    ///
    /// Also defines the direction of the circle's coordinate system. The length
    /// is equal to the circle's radius, and this vector is perpendicular to
    /// [`Self::a`].
    pub fn b(&self) -> Vector<D> {
        self.b
    }

    /// Create a new instance that is reversed
    #[must_use]
    pub fn reverse(mut self) -> Self {
        self.b = -self.b;
        self
    }

    /// Convert a `D`-dimensional point to circle coordinates
    ///
    /// Converts the provided point into circle coordinates between `0.`
    /// (inclusive) and `PI * 2.` (exclusive).
    ///
    /// Projects the point onto the circle before computing circle coordinate,
    /// ignoring the radius. This is done to make this method robust against
    /// floating point accuracy issues.
    ///
    /// Callers are advised to be careful about the points they pass, as the
    /// point not being on the curve, intentional or not, will not result in an
    /// error.
    pub fn point_to_circle_coords(
        &self,
        point: impl Into<Point<D>>,
    ) -> Point<1> {
        let vector = (point.into() - self.center).to_uv();
        let atan = Scalar::atan2(vector.v, vector.u);
        let coord = if atan >= Scalar::ZERO {
            atan
        } else {
            atan + Scalar::PI * 2.
        };
        Point::from([coord])
    }

    /// Convert a point in circle coordinates into a `D`-dimensional point
    pub fn point_from_circle_coords(
        &self,
        point: impl Into<Point<1>>,
    ) -> Point<D> {
        self.center + self.vector_from_circle_coords(point.into().coords)
    }

    /// Convert a vector in circle coordinates into a `D`-dimensional point
    pub fn vector_from_circle_coords(
        &self,
        vector: impl Into<Vector<1>>,
    ) -> Vector<D> {
        let angle = vector.into().t;
        let (sin, cos) = angle.sin_cos();

        self.a * cos + self.b * sin
    }
}

impl<const D: usize> approx::AbsDiffEq for Circle<D> {
    type Epsilon = <Scalar as approx::AbsDiffEq>::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        Scalar::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.center.abs_diff_eq(&other.center, epsilon)
            && self.a.abs_diff_eq(&other.a, epsilon)
            && self.b.abs_diff_eq(&other.b, epsilon)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_PI_2, PI};

    use crate::{Point, Vector};

    use super::Circle;

    #[test]
    fn point_to_circle_coords() {
        let circle = Circle {
            center: Point::from([1., 2., 3.]),
            a: Vector::from([1., 0., 0.]),
            b: Vector::from([0., 1., 0.]),
        };

        assert_eq!(
            circle.point_to_circle_coords([2., 2., 3.]),
            Point::from([0.]),
        );
        assert_eq!(
            circle.point_to_circle_coords([1., 3., 3.]),
            Point::from([FRAC_PI_2]),
        );
        assert_eq!(
            circle.point_to_circle_coords([0., 2., 3.]),
            Point::from([PI]),
        );
        assert_eq!(
            circle.point_to_circle_coords([1., 1., 3.]),
            Point::from([FRAC_PI_2 * 3.]),
        );
    }
}
