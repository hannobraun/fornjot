use fj_math::{Point, Scalar, Vector};

/// # A kind of curve geometry
///
/// Curve geometry is always defined locally on a surface.
pub trait CurveGeometry {
    /// # Convert a point on the curve into a vector on the surface
    fn local_point_to_vector(&self, point: Point<1>) -> Vector<2>;

    /// # Compute the increment of approximation from the given point
    fn increment_at(
        &self,
        point: Point<1>,
        tolerance: Tolerance,
        size_hint: SizeHint,
    ) -> Increment<1>;
}

/// # A kind of surface geometry
pub trait SurfaceGeometry {
    /// # Convert a point on the surface into a vector in space
    fn local_point_to_vector(&self, point: Point<2>) -> Vector<3>;

    /// # Compute the increment of approximation from the given point
    fn increment_at(
        &self,
        point: Point<2>,
        tolerance: Tolerance,
        size_hint: SizeHint,
    ) -> Increment<2>;
}

/// # A tolerance
///
/// Must be non-zero and positive.
pub struct Tolerance {
    /// # The tolerance value
    ///
    /// This should not be public. It should be private, and constructing a
    /// `Tolerance` should ensure that the value is valid.
    pub value: Scalar,
}

/// # A size hint
///
/// Must be zero or positive.
pub struct SizeHint {
    /// # The size hint value
    ///
    /// This should not be public. It should be private, and constructing a
    /// `SizeHint` should ensure that the value is valid.
    pub value: Scalar,
}

/// # An increment of approximation of a curve or surface
pub struct Increment<const D: usize> {
    pub value: Vector<D>,
}

pub struct Plane {
    pub u: Vector<3>,
    pub v: Vector<3>,
}

impl SurfaceGeometry for Plane {
    fn local_point_to_vector(&self, point: Point<2>) -> Vector<3> {
        self.u * point.u + self.v * point.v
    }

    fn increment_at(
        &self,
        _: Point<2>,
        _: Tolerance,
        size_hint: SizeHint,
    ) -> Increment<2> {
        Increment {
            value: Vector::from_component(size_hint.value),
        }
    }
}
