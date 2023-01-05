use approx::AbsDiffEq;

use crate::{Point, Scalar, Vector};

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
        let vector = (point.into() - self.center).to_uv();
        let atan = Scalar::atan2(vector.v, vector.u);
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

/// Calculated geometry that is useful when dealing with an arc
pub struct ArcCircleData {
    /// Start point of the arc
    pub start: Point<2>,
    /// End point of the arc
    pub end: Point<2>,
    /// Center of the circle the arc is constructed on
    pub center: Point<2>,
    /// Radius of the circle the arc is constructed on
    pub radius: Scalar,
    /// Angle of `start` relative to `center`, in radians
    ///
    /// Guaranteed to be less than `end_angle`.
    pub start_angle: Scalar,
    /// Angle of `end` relative to `center`, in radians
    ///
    /// Guaranteed to be greater than `end_angle`.
    pub end_angle: Scalar,
    /// True if `start` and `end` were switched to ensure `end_angle` > `start_angle`
    pub flipped_construction: bool,
}

impl ArcCircleData {
    /// Constructs an [`ArcCircleData`] from two endpoints and the associated angle.
    pub fn from_endpoints_and_angle(
        p0: impl Into<Point<2>>,
        p1: impl Into<Point<2>>,
        angle: Scalar,
    ) -> Self {
        use num_traits::Float;

        let (p0, p1) = (p0.into(), p1.into());

        let flipped_construction = angle <= Scalar::ZERO;
        let angle_rad = angle.abs();

        let [p0, p1] = if flipped_construction {
            [p1, p0]
        } else {
            [p0, p1]
        };
        let [[x0, y0], [x1, y1]] = [p0, p1].map(|p| p.coords.components);
        // https://math.stackexchange.com/questions/27535/how-to-find-center-of-an-arc-given-start-point-end-point-radius-and-arc-direc
        // distance between endpoints
        let d = ((x1 - x0).powi(2) + (y1 - y0).powi(2)).sqrt();
        // radius
        let r = d / (2. * (angle_rad.into_f64() / 2.).sin());
        // distance from center to midpoint between endpoints
        let h = (r.powi(2) - (d.powi(2) / 4.)).sqrt();
        // (u, v) is the unit normal in the direction of p1 - p0
        let u = (x1 - x0) / d;
        let v = (y1 - y0) / d;
        // (cx, cy) is the center of the circle
        let cx = ((x0 + x1) / 2.) - h * v;
        let cy = ((y0 + y1) / 2.) + h * u;
        let start_angle = (y0 - cy).atan2(x0 - cx);
        let end_angle = (y1 - cy).atan2(x1 - cx);
        Self {
            start: p0,
            end: p1,
            center: Point::from([cx, cy]),
            radius: r,
            start_angle,
            end_angle,
            flipped_construction,
        }
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
