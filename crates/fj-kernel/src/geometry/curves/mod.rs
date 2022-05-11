mod circle;

pub use self::circle::Circle;

use fj_math::{Line, Point, Transform, Vector};

/// A one-dimensional shape
///
/// The word "curve" is used as an umbrella term for all one-dimensional shapes,
/// and doesn't imply that those shapes need to be curved. Straight lines are
/// included.
///
/// The nomenclature is inspired by Boundary Representation Modelling Techniques
/// by Ian Stroud. "Curve" refers to unbounded one-dimensional geometry, while
/// while edges are bounded portions of curves.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Curve {
    /// A circle
    Circle(Circle),

    /// A line
    Line(Line<3>),
}

impl Curve {
    /// Construct a `Curve` that represents the x-axis
    pub fn x_axis() -> Self {
        Self::Line(Line {
            origin: Point::origin(),
            direction: Vector::unit_x(),
        })
    }

    /// Construct a `Curve` that represents the y-axis
    pub fn y_axis() -> Self {
        Self::Line(Line {
            origin: Point::origin(),
            direction: Vector::unit_y(),
        })
    }

    /// Construct a `Curve` that represents the z-axis
    pub fn z_axis() -> Self {
        Self::Line(Line {
            origin: Point::origin(),
            direction: Vector::unit_z(),
        })
    }

    /// Access the origin of the curve's coordinate system
    pub fn origin(&self) -> Point<3> {
        match self {
            Self::Circle(curve) => curve.origin(),
            Self::Line(curve) => curve.origin,
        }
    }

    /// Create a new instance that is reversed
    #[must_use]
    pub fn reverse(self) -> Self {
        match self {
            Self::Circle(curve) => Self::Circle(curve.reverse()),
            Self::Line(curve) => Self::Line(curve.reverse()),
        }
    }

    /// Create a new instance that is transformed by `transform`
    #[must_use]
    pub fn transform(self, transform: &Transform) -> Self {
        match self {
            Self::Circle(curve) => Self::Circle(curve.transform(transform)),
            Self::Line(curve) => Self::Line(transform.transform_line(&curve)),
        }
    }

    /// Convert a point in model coordinates to curve coordinates
    ///
    /// Projects the point onto the curve before computing curve coordinate.
    /// This is done to make this method robust against floating point accuracy
    /// issues.
    ///
    /// Callers are advised to be careful about the points they pass, as the
    /// point not being on the curve, intentional or not, will never result in
    /// an error.
    pub fn point_to_curve_coords(&self, point: &Point<3>) -> Point<1> {
        match self {
            Self::Circle(curve) => curve.point_to_circle_coords(*point),
            Self::Line(curve) => curve.point_to_line_coords(*point),
        }
    }

    /// Convert a point on the curve into model coordinates
    pub fn point_curve_to_model(&self, point: &Point<1>) -> Point<3> {
        match self {
            Self::Circle(curve) => curve.point_from_circle_coords(*point),
            Self::Line(curve) => curve.point_from_line_coords(*point),
        }
    }

    /// Convert a vector on the curve into model coordinates
    pub fn vector_curve_to_model(&self, point: &Vector<1>) -> Vector<3> {
        match self {
            Self::Circle(curve) => curve.vector_from_circle_coords(*point),
            Self::Line(curve) => curve.vector_from_line_coords(*point),
        }
    }
}
