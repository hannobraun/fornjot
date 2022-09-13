use fj_math::{Line, Point, Vector};

use crate::objects::{Curve, CurveKind, GlobalCurve, Surface};

/// API for building a [`Curve`]
pub struct CurveBuilder {
    surface: Surface,
}

impl CurveBuilder {
    /// Construct a new instance of [`CurveBuilder`]
    ///
    /// Also see [`Curve::build`].
    pub fn new(surface: Surface) -> Self {
        Self { surface }
    }

    /// Create a line that represents the u-axis on the surface
    pub fn u_axis(&self) -> Curve {
        let a = Point::origin();
        let b = a + Vector::unit_u();

        self.line_from_points([a, b])
    }

    /// Create a line that represents the v-axis on the surface
    pub fn v_axis(&self) -> Curve {
        let a = Point::origin();
        let b = a + Vector::unit_v();

        self.line_from_points([a, b])
    }

    /// Create a line from the given points
    pub fn line_from_points(&self, points: [impl Into<Point<2>>; 2]) -> Curve {
        let points = points.map(Into::into);

        let local = Line::from_points(points);
        let global = Line::from_points(
            points.map(|point| self.surface.point_from_surface_coords(point)),
        );

        Curve::new(
            self.surface,
            CurveKind::Line(local),
            GlobalCurve::from_path(CurveKind::Line(global)),
        )
    }
}

/// API for building a [`GlobalCurve`]
pub struct GlobalCurveBuilder;

impl GlobalCurveBuilder {
    /// Create a line that represents the x-axis
    pub fn x_axis(&self) -> GlobalCurve {
        GlobalCurve::from_path(CurveKind::x_axis())
    }

    /// Create a line that represents the y-axis
    pub fn y_axis(&self) -> GlobalCurve {
        GlobalCurve::from_path(CurveKind::y_axis())
    }

    /// Create a line that represents the z-axis
    pub fn z_axis(&self) -> GlobalCurve {
        GlobalCurve::from_path(CurveKind::z_axis())
    }

    /// Create a line from the given points
    pub fn line_from_points(
        &self,
        points: [impl Into<Point<3>>; 2],
    ) -> GlobalCurve {
        let line = Line::from_points(points);
        GlobalCurve::from_path(CurveKind::Line(line))
    }
}
