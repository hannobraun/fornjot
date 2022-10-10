use fj_math::{Point, Scalar, Vector};

use crate::{
    objects::{Curve, GlobalCurve, Stores, Surface},
    path::SurfacePath,
    storage::{Handle, HandleWrapper},
};

/// A partial [`Curve`]
///
/// See [`crate::partial`] for more information.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct PartialCurve {
    /// The path that defines the [`Curve`]
    ///
    /// Must be provided before calling [`PartialCurve::build`].
    pub path: Option<SurfacePath>,

    /// The surface that the [`Curve`] is defined in
    ///
    /// Must be provided before calling [`PartialCurve::build`].
    pub surface: Option<Handle<Surface>>,

    /// The global form of the [`Curve`]
    ///
    /// Will be computed from `path` and `surface` in [`PartialCurve::build`],
    /// if not provided.
    pub global_form: Option<HandleWrapper<GlobalCurve>>,
}

impl PartialCurve {
    /// Provide a path for the partial curve
    pub fn with_path(mut self, path: Option<SurfacePath>) -> Self {
        if let Some(path) = path {
            self.path = Some(path);
        }
        self
    }

    /// Provide a surface for the partial curve
    pub fn with_surface(mut self, surface: Option<Handle<Surface>>) -> Self {
        if let Some(surface) = surface {
            self.surface = Some(surface);
        }
        self
    }

    /// Provide a global form for the partial curve
    pub fn with_global_form(
        mut self,
        global_form: Option<impl Into<HandleWrapper<GlobalCurve>>>,
    ) -> Self {
        if let Some(global_form) = global_form {
            self.global_form = Some(global_form.into());
        }
        self
    }

    /// Update partial curve to represent the u-axis
    pub fn as_u_axis(self) -> Self {
        let a = Point::origin();
        let b = a + Vector::unit_u();

        self.as_line_from_points([a, b])
    }

    /// Update partial curve to represent the v-axis
    pub fn as_v_axis(self) -> Self {
        let a = Point::origin();
        let b = a + Vector::unit_v();

        self.as_line_from_points([a, b])
    }

    /// Update partial curve as a circle, from the provided radius
    pub fn as_circle_from_radius(self, radius: impl Into<Scalar>) -> Self {
        self.with_path(Some(SurfacePath::circle_from_radius(radius)))
    }

    /// Update partial curve as a line, from the provided points
    pub fn as_line_from_points(self, points: [impl Into<Point<2>>; 2]) -> Self {
        self.with_path(Some(SurfacePath::line_from_points(points)))
    }

    /// Build a full [`Curve`] from the partial curve
    pub fn build(self, stores: &Stores) -> Handle<Curve> {
        let path = self.path.expect("Can't build `Curve` without path");
        let surface =
            self.surface.expect("Can't build `Curve` without surface");

        let global_form = self
            .global_form
            .unwrap_or_else(|| GlobalCurve::new(stores).into());

        Curve::new(surface, path, global_form, stores)
    }
}

impl From<&Handle<Curve>> for PartialCurve {
    fn from(curve: &Handle<Curve>) -> Self {
        Self {
            path: Some(curve.path()),
            surface: Some(curve.surface().clone()),
            global_form: Some(curve.global_form().clone().into()),
        }
    }
}
