//! Paths through space

use fj_math::{Circle, Line, Point, Vector};

/// A path through global (3D) space
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum GlobalPath {
    /// A circle
    Circle(Circle<3>),

    /// A line
    Line(Line<3>),
}

impl GlobalPath {
    /// Construct a line from two points
    pub fn line_from_points(points: [impl Into<Point<3>>; 2]) -> Self {
        Self::Line(Line::from_points(points))
    }

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
