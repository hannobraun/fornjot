use approx::AbsDiffEq;

use crate::{Aabb, Point, Scalar, Transform, Vector};

/// An n-dimensional circle
///
/// The dimensionality of the circle is defined by the const generic `D`
/// parameter.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Circle<const D: usize> {
    center: Point<D>,
    a: Vector<D>,
    b: Vector<D>,
}

impl<const D: usize> Circle<D> {
    /// Construct a circle
    ///
    /// # Panics
    ///
    /// Panics, if any of the following requirements are not met:
    ///
    /// - The circle radius (defined by the length of `a` and `b`) must not be
    ///   zero.
    /// - `a` and `b` must be of equal length.
    /// - `a` and `b` must be perpendicular to each other.
    pub fn new(
        center: impl Into<Point<D>>,
        a: impl Into<Vector<D>>,
        b: impl Into<Vector<D>>,
    ) -> Self {
        let center = center.into();
        let a = a.into();
        let b = b.into();

        assert_eq!(
            a.magnitude(),
            b.magnitude(),
            "`a` and `b` must be of equal length"
        );
        assert_ne!(
            a.magnitude(),
            Scalar::ZERO,
            "circle radius must not be zero"
        );
        // Requiring the vector to be *precisely* perpendicular is not
        // practical, because of numerical inaccuracy. This epsilon value seems
        // seems to work for now, but maybe it needs to become configurable.
        assert!(
            a.dot(&b) < Scalar::default_epsilon(),
            "`a` and `b` must be perpendicular to each other"
        );

        Self { center, a, b }
    }

    /// Construct a `Circle` from a center point and a radius
    pub fn from_center_and_radius(
        center: impl Into<Point<D>>,
        radius: impl Into<Scalar>,
    ) -> Self {
        let radius = radius.into();

        let mut a = [Scalar::ZERO; D];
        let mut b = [Scalar::ZERO; D];

        a[0] = radius;
        b[1] = radius;

        Self::new(center, a, b)
    }

    /// Access the center point of the circle
    pub fn center(&self) -> Point<D> {
        self.center
    }

    /// Access the radius of the circle
    pub fn radius(&self) -> Scalar {
        self.a().magnitude()
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
        let point = point.into();

        let center_to_point = point - self.center;
        let [a, b] = [&self.a, &self.b]
            .map(|v| center_to_point.scalar_projection_onto(v));

        let atan = Scalar::atan2(b, a);
        let coord = if atan >= Scalar::ZERO {
            atan
        } else {
            atan + Scalar::TAU
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

    /// Calculate an AABB for the circle
    pub fn aabb(&self) -> Aabb<D> {
        let center_to_min_max = Vector::from_component(self.radius());

        Aabb {
            min: self.center() - center_to_min_max,
            max: self.center() + center_to_min_max,
        }
    }
}

impl Circle<3> {
    /// # Transform the circle
    pub fn transform(&self, transform: &Transform) -> Self {
        Circle::new(
            transform.transform_point(&self.center()),
            transform.transform_vector(&self.a()),
            transform.transform_vector(&self.b()),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{FRAC_PI_2, PI};

    use crate::{Circle, Point, Vector};

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

        let circle = Circle {
            center: Point::from([1., 2., 3.]),
            a: Vector::from([1., 0., 0.]),
            b: Vector::from([0., 0., 1.]),
        };

        assert_eq!(
            circle.point_to_circle_coords([2., 2., 3.]),
            Point::from([0.]),
        );
        assert_eq!(
            circle.point_to_circle_coords([1., 2., 4.]),
            Point::from([FRAC_PI_2]),
        );
        assert_eq!(
            circle.point_to_circle_coords([0., 2., 3.]),
            Point::from([PI]),
        );
        assert_eq!(
            circle.point_to_circle_coords([1., 2., 2.]),
            Point::from([FRAC_PI_2 * 3.]),
        );
    }
}
