use fj_math::{Line, Point, Scalar, Vector};

use crate::{
    objects::{Curve, GlobalCurve, Surface},
    path::{GlobalPath, SurfacePath},
};

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

    /// Build a line that represents the u-axis on the surface
    pub fn u_axis(&self) -> Curve {
        let a = Point::origin();
        let b = a + Vector::unit_u();

        self.line_from_points([a, b])
    }

    /// Build a line that represents the v-axis on the surface
    pub fn v_axis(&self) -> Curve {
        let a = Point::origin();
        let b = a + Vector::unit_v();

        self.line_from_points([a, b])
    }

    /// Build a circle from the given radius
    pub fn circle_from_radius(&self, radius: impl Into<Scalar>) -> Curve {
        let radius = radius.into();

        let path = SurfacePath::circle_from_radius(radius);
        let global_form = GlobalCurve::build().circle_from_radius(radius);

        Curve::new(self.surface, path, global_form)
    }

    /// Build a line from the given points
    pub fn line_from_points(&self, points: [impl Into<Point<2>>; 2]) -> Curve {
        let points = points.map(Into::into);

        let local = Line::from_points(points);
        let global = Line::from_points(
            points.map(|point| self.surface.point_from_surface_coords(point)),
        );

        Curve::new(
            self.surface,
            SurfacePath::Line(local),
            GlobalCurve::from_path(GlobalPath::Line(global)),
        )
    }
}

/// API for building a [`GlobalCurve`]
pub struct GlobalCurveBuilder;

impl GlobalCurveBuilder {
    /// Build a line that represents the x-axis
    pub fn x_axis(&self) -> GlobalCurve {
        GlobalCurve::from_path(GlobalPath::x_axis())
    }

    /// Build a line that represents the y-axis
    pub fn y_axis(&self) -> GlobalCurve {
        GlobalCurve::from_path(GlobalPath::y_axis())
    }

    /// Build a line that represents the z-axis
    pub fn z_axis(&self) -> GlobalCurve {
        GlobalCurve::from_path(GlobalPath::z_axis())
    }

    /// Build a circle from the given radius
    pub fn circle_from_radius(&self, radius: impl Into<Scalar>) -> GlobalCurve {
        let path = GlobalPath::circle_from_radius(radius);
        GlobalCurve::from_path(path)
    }

    /// Create a line from the given points
    pub fn line_from_points(
        &self,
        points: [impl Into<Point<3>>; 2],
    ) -> GlobalCurve {
        let line = Line::from_points(points);
        GlobalCurve::from_path(GlobalPath::Line(line))
    }
}
