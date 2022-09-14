//! Paths through 2D and 3D space
//!
//! See [`SurfacePath`] and [`GlobalPath`].
//!
//! # Implementation Note
//!
//! This is a bit of an in-between module. It is closely associated with curves
//! ([`Curve`]/[`GlobalCurve`]) and [`Surface`]s, but paths are not really
//! objects themselves, as logically speaking, they are owned and not referenced
//! (practically speaking, all objects are owned and not referenced, but that is
//! an implementation detail; see [#1021] for context on where things are
//! going).
//!
//! On the other hand, the types in this module don't follow the general style
//! of types in `fj-math`.
//!
//! We'll see how it shakes out. Maybe it will stay its own thing, maybe it will
//! move to `fj-math`, maybe something else entirely will happen.
//!
//! [`Curve`]: crate::objects::Curve
//! [`GlobalCurve`]: crate::objects::GlobalCurve
//! [`Surface`]: crate::objects::Surface
//! [#1021]: https://github.com/hannobraun/Fornjot/issues/1021

use fj_math::{Circle, Line, Point, Scalar, Vector};

/// A path through surface (2D) space
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum SurfacePath {
    /// A circle
    Circle(Circle<2>),

    /// A line
    Line(Line<2>),
}

impl SurfacePath {
    /// Build a circle from the given radius
    pub fn circle_from_radius(radius: impl Into<Scalar>) -> Self {
        let radius = radius.into();

        SurfacePath::Circle(Circle::from_center_and_radius(
            Point::origin(),
            radius,
        ))
    }

    /// Construct a line from two points
    pub fn line_from_points(points: [impl Into<Point<2>>; 2]) -> Self {
        Self::Line(Line::from_points(points))
    }

    /// Convert a point on the path into global coordinates
    pub fn point_from_path_coords(
        &self,
        point: impl Into<Point<1>>,
    ) -> Point<2> {
        match self {
            Self::Circle(circle) => circle.point_from_circle_coords(point),
            Self::Line(line) => line.point_from_line_coords(point),
        }
    }
}

/// A path through global (3D) space
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum GlobalPath {
    /// A circle
    Circle(Circle<3>),

    /// A line
    Line(Line<3>),
}

impl GlobalPath {
    /// Construct a `GlobalPath` that represents the x-axis
    pub fn x_axis() -> Self {
        Self::Line(Line::from_origin_and_direction(
            Point::origin(),
            Vector::unit_x(),
        ))
    }

    /// Construct a `GlobalPath` that represents the y-axis
    pub fn y_axis() -> Self {
        Self::Line(Line::from_origin_and_direction(
            Point::origin(),
            Vector::unit_y(),
        ))
    }

    /// Construct a `GlobalPath` that represents the z-axis
    pub fn z_axis() -> Self {
        Self::Line(Line::from_origin_and_direction(
            Point::origin(),
            Vector::unit_z(),
        ))
    }

    /// Build a circle from the given radius
    pub fn circle_from_radius(radius: impl Into<Scalar>) -> Self {
        let radius = radius.into();

        GlobalPath::Circle(Circle::from_center_and_radius(
            Point::origin(),
            radius,
        ))
    }

    /// Construct a line from two points
    pub fn line_from_points(points: [impl Into<Point<3>>; 2]) -> Self {
        Self::Line(Line::from_points(points))
    }

    /// Access the origin of the path's coordinate system
    pub fn origin(&self) -> Point<3> {
        match self {
            Self::Circle(circle) => circle.center() + circle.a(),
            Self::Line(line) => line.origin(),
        }
    }

    /// Convert a point on the path into global coordinates
    pub fn point_from_path_coords(
        &self,
        point: impl Into<Point<1>>,
    ) -> Point<3> {
        match self {
            Self::Circle(circle) => circle.point_from_circle_coords(point),
            Self::Line(line) => line.point_from_line_coords(point),
        }
    }

    /// Convert a vector on the path into global coordinates
    pub fn vector_from_path_coords(
        &self,
        vector: impl Into<Vector<1>>,
    ) -> Vector<3> {
        match self {
            Self::Circle(circle) => circle.vector_from_circle_coords(vector),
            Self::Line(line) => line.vector_from_line_coords(vector),
        }
    }
}
