use fj_math::{Circle, Line, Point, Transform, Vector};

use crate::builder::{CurveBuilder, GlobalCurveBuilder};

use super::Surface;

/// A curve, defined in local surface coordinates
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Curve {
    kind: CurveKind<2>,
    global: GlobalCurve,
}

impl Curve {
    /// Build a curve using [`CurveBuilder`]
    pub fn build(surface: Surface) -> CurveBuilder {
        CurveBuilder::new(surface)
    }

    /// Construct a new instance of `Curve`
    pub fn new(kind: CurveKind<2>, global: GlobalCurve) -> Self {
        Self { kind, global }
    }

    /// Access the kind of this curve
    pub fn kind(&self) -> &CurveKind<2> {
        &self.kind
    }

    /// Access the global form of this curve
    pub fn global(&self) -> &GlobalCurve {
        &self.global
    }
}

/// A curve, defined in global (3D) coordinates
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct GlobalCurve {
    kind: CurveKind<3>,
}

impl GlobalCurve {
    /// Build a curve using [`GlobalCurveBuilder`]
    pub fn build() -> GlobalCurveBuilder {
        GlobalCurveBuilder
    }

    /// Construct a `GlobalCurve` from a [`CurveKind<3>`]
    pub fn from_kind(kind: CurveKind<3>) -> Self {
        Self { kind }
    }

    /// Access the kind of this curve
    pub fn kind(&self) -> &CurveKind<3> {
        &self.kind
    }
}

/// A one-dimensional shape
///
/// The word "curve" is used as an umbrella term for all one-dimensional shapes,
/// and doesn't imply that those shapes need to be curved. Straight lines are
/// included.
///
/// The nomenclature is inspired by Boundary Representation Modelling Techniques
/// by Ian Stroud. "Curve" refers to unbounded one-dimensional geometry, while
/// while edges are bounded portions of curves.
///
/// The `D` parameter defines the dimensions in which the curve is defined.
/// Typically, only `2` or `3` make sense, which means the curve is defined on
/// a surface or in a space, respectively.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum CurveKind<const D: usize> {
    /// A circle
    Circle(Circle<D>),

    /// A line
    Line(Line<D>),
}

impl<const D: usize> CurveKind<D> {
    /// Construct a line from two points
    pub fn line_from_points(points: [impl Into<Point<D>>; 2]) -> Self {
        Self::Line(Line::from_points(points))
    }

    /// Access the origin of the curve's coordinate system
    pub fn origin(&self) -> Point<D> {
        match self {
            Self::Circle(curve) => curve.center(),
            Self::Line(curve) => curve.origin(),
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

    /// Convert a point on the curve into model coordinates
    pub fn point_from_curve_coords(
        &self,
        point: impl Into<Point<1>>,
    ) -> Point<D> {
        match self {
            Self::Circle(curve) => curve.point_from_circle_coords(point),
            Self::Line(curve) => curve.point_from_line_coords(point),
        }
    }

    /// Convert a vector on the curve into model coordinates
    pub fn vector_from_curve_coords(
        &self,
        point: impl Into<Vector<1>>,
    ) -> Vector<D> {
        match self {
            Self::Circle(curve) => curve.vector_from_circle_coords(point),
            Self::Line(curve) => curve.vector_from_line_coords(point),
        }
    }
}

impl CurveKind<3> {
    /// Construct a `Curve` that represents the x-axis
    pub fn x_axis() -> Self {
        Self::Line(Line::from_origin_and_direction(
            Point::origin(),
            Vector::unit_x(),
        ))
    }

    /// Construct a `Curve` that represents the y-axis
    pub fn y_axis() -> Self {
        Self::Line(Line::from_origin_and_direction(
            Point::origin(),
            Vector::unit_y(),
        ))
    }

    /// Construct a `Curve` that represents the z-axis
    pub fn z_axis() -> Self {
        Self::Line(Line::from_origin_and_direction(
            Point::origin(),
            Vector::unit_z(),
        ))
    }

    /// Transform the surface
    #[must_use]
    pub fn transform(self, transform: &Transform) -> Self {
        match self {
            CurveKind::Circle(curve) => {
                CurveKind::Circle(transform.transform_circle(&curve))
            }
            CurveKind::Line(curve) => {
                CurveKind::Line(transform.transform_line(&curve))
            }
        }
    }
}
