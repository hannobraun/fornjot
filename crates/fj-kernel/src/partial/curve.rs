use fj_math::{Line, Point, Scalar, Vector};

use crate::{
    objects::{Curve, GlobalCurve, Surface},
    path::{GlobalPath, SurfacePath},
    stores::{Handle, Stores},
};

/// API for building a [`Curve`]
///
/// Also see [`Curve::builder`].
pub struct PartialCurve<'a> {
    /// The stores that the created objects are put in
    pub stores: &'a Stores,

    /// The path that defines the [`Curve`]
    ///
    /// Must be provided before calling [`PartialCurve::build`].
    pub path: Option<SurfacePath>,

    /// The surface that the [`Curve`] is defined in
    pub surface: Surface,

    /// The global form of the [`Curve`]
    ///
    /// Must be provided before calling [`PartialCurve::build`].
    pub global_form: Option<GlobalCurve>,
}

impl<'a> PartialCurve<'a> {
    /// Build a line that represents the u-axis on the surface
    pub fn build_u_axis(self) -> Curve {
        let a = Point::origin();
        let b = a + Vector::unit_u();

        self.as_line_from_points([a, b]).build()
    }

    /// Build a line that represents the v-axis on the surface
    pub fn build_v_axis(self) -> Curve {
        let a = Point::origin();
        let b = a + Vector::unit_v();

        self.as_line_from_points([a, b]).build()
    }

    /// Build a circle from the given radius
    pub fn build_circle_from_radius(self, radius: impl Into<Scalar>) -> Curve {
        let radius = radius.into();

        let path = SurfacePath::circle_from_radius(radius);
        let global_form = GlobalCurve::partial()
            .as_circle_from_radius(radius)
            .build(self.stores);

        Curve::new(self.surface, path, global_form)
    }

    /// Update partial curve as a line, from the provided points
    pub fn as_line_from_points(
        mut self,
        points: [impl Into<Point<2>>; 2],
    ) -> Self {
        let points = points.map(Into::into);

        self.path = Some(SurfacePath::Line(Line::from_points(points)));
        self.global_form =
            Some(GlobalCurve::from_path(GlobalPath::Line(Line::from_points(
                points
                    .map(|point| self.surface.point_from_surface_coords(point)),
            ))));

        self
    }

    /// Build a full [`Curve`] from the partial curve
    pub fn build(self) -> Curve {
        let path = self.path.expect("Can't build `Curve` without path");
        let global_form = self
            .global_form
            .expect("Can't build `Curve` without a global form");

        let global_form = self.stores.global_curves.insert(global_form);

        Curve::new(self.surface, path, global_form)
    }
}

/// A partial [`GlobalCurve`]
///
/// See [`crate::partial`] for more information.
#[derive(Default)]
pub struct PartialGlobalCurve {
    /// The path that defines the [`GlobalCurve`]
    ///
    /// Must be provided before [`PartialGlobalCurve::build`] is called.
    pub path: Option<GlobalPath>,
}

impl PartialGlobalCurve {
    /// Provide a path for the partial global curve
    pub fn with_path(mut self, path: GlobalPath) -> Self {
        self.path = Some(path);
        self
    }

    /// Update partial global curve to represent the x-axis
    pub fn as_x_axis(self) -> Self {
        self.with_path(GlobalPath::x_axis())
    }

    /// Update partial global curve to represent the y-axis
    pub fn as_y_axis(self) -> Self {
        self.with_path(GlobalPath::y_axis())
    }

    /// Update partial global curve to represent the z-axis
    pub fn as_z_axis(self) -> Self {
        self.with_path(GlobalPath::z_axis())
    }

    /// Update partial global curve as a circle, from the provided radius
    pub fn as_circle_from_radius(self, radius: impl Into<Scalar>) -> Self {
        self.with_path(GlobalPath::circle_from_radius(radius))
    }

    /// Update partial global curve as a line, from the provided points
    pub fn as_line_from_points(self, points: [impl Into<Point<3>>; 2]) -> Self {
        self.with_path(GlobalPath::line_from_points(points))
    }

    /// Build a full [`GlobalCurve`] from the partial global curve
    ///
    /// # Panics
    ///
    /// Panics, if no path was provided.
    pub fn build(self, stores: &Stores) -> Handle<GlobalCurve> {
        let path = self.path.expect("Can't build `GlobalCurve` without a path");
        stores.global_curves.insert(GlobalCurve::from_path(path))
    }
}
