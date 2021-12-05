use std::f64::consts::FRAC_PI_2;

use nalgebra::{TAffine, Transform, Translation};
use parry3d_f64::{
    bounding_volume::AABB,
    query::{Ray, RayCast as _},
};
use winit::dpi::PhysicalPosition;

use crate::{
    geometry::faces::Faces,
    math::{Point, Vector},
    window::Window,
};

/// The camera abstraction
///
/// Please note that the metaphor we're using (which influences how mouse input
/// is handled, for example) is not that of a camera freely flying through a
/// static scene. Instead, the camera is static, and the model is freely
/// translated and rotated.
#[derive(Debug)]
pub struct Camera {
    /// The distance to the near plane
    near_plane: f64,

    /// The rotational part of the transform
    ///
    /// This is not an `nalgebra::Rotation`, as rotations happen around a center
    /// point, which means they must include a translational component.
    pub rotation: Transform<f64, TAffine, 3>,

    pub translation: Translation<f64, 2>,
    pub distance: f64,
}

impl Camera {
    const INITIAL_NEAR_PLANE: f64 = 0.1;
    const INITIAL_FAR_PLANE: f64 = 1000.0;
    const INITIAL_FIELD_OF_VIEW_IN_X: f64 = FRAC_PI_2; // 90 degrees

    pub fn new(aabb: &AABB) -> Self {
        let initial_distance = {
            // Let's make sure we choose a distance, so that the model fills
            // most of the screen.
            //
            // To do that, first compute the model's highest point, as well as
            // the furthest point from the origin, in x and y.
            let highest_point = aabb.maxs.z;
            let furthest_point = [
                aabb.mins.x.abs(),
                aabb.maxs.x,
                aabb.mins.y.abs(),
                aabb.maxs.y,
            ]
            .into_iter()
            .reduce(|a, b| f64::max(a, b))
            // `reduce` can only return `None`, if there are no items in
            // the iterator. And since we're creating an array full of
            // items above, we know this can't panic.
            .unwrap();

            // The actual furthest point is not far enough. We don't want the
            // model to fill the whole screen.
            let furthest_point = furthest_point * 2.;

            // Having computed those points, figuring out how far the camera
            // needs to be from the model is just a bit of trigonometry.
            let distance_from_model =
                furthest_point / (Self::INITIAL_FIELD_OF_VIEW_IN_X / 2.).atan();

            // An finally, the distance from the origin is trivial now.
            highest_point + distance_from_model
        };

        Self {
            near_plane: Self::INITIAL_NEAR_PLANE,

            rotation: Transform::identity(),
            translation: Translation::identity(),
            distance: initial_distance,
        }
    }

    /// Update the distances of the near and far planes
    ///
    /// Sets those distances such, that they envelope the model, so everything
    /// is always shown, no matter how close the camera is to it.
    pub fn update_planes(&mut self, _aabb: &AABB) {
        // TASK: Implement.
    }

    pub fn near_plane(&self) -> f64 {
        self.near_plane
    }

    pub fn far_plane(&self) -> f64 {
        Self::INITIAL_FAR_PLANE
    }

    pub fn field_of_view_in_x(&self) -> f64 {
        Self::INITIAL_FIELD_OF_VIEW_IN_X
    }

    pub fn position(&self) -> Point {
        self.view_transform()
            .inverse_transform_point(&Point::origin())
    }

    /// Transform the position of the cursor on the near plane to model space
    pub fn cursor_to_model_space(
        &self,
        cursor: PhysicalPosition<f64>,
        window: &Window,
    ) -> Point {
        let width = window.width() as f64;
        let height = window.height() as f64;
        let aspect_ratio = width / height;

        // Cursor position in normalized coordinates (-1 to +1) with
        // aspect ratio taken into account.
        let x = cursor.x / width * 2. - 1.;
        let y = -(cursor.y / height * 2. - 1.) / aspect_ratio;

        // Cursor position in camera space.
        let f = (self.field_of_view_in_x() / 2.).tan() * self.near_plane();
        let cursor =
            Point::origin() + Vector::new(x * f, y * f, -self.near_plane());

        self.view_transform().inverse_transform_point(&cursor)
    }

    /// Compute the point on the model, that the cursor currently points to
    pub fn focus_point(
        &self,
        window: &Window,
        cursor: Option<PhysicalPosition<f64>>,
        faces: &Faces,
    ) -> Option<Point> {
        let cursor = cursor?;

        // Transform camera and cursor positions to model space.
        let origin = self.position();
        let cursor = self.cursor_to_model_space(cursor, window);
        let dir = (cursor - origin).normalize();

        let ray = Ray { origin, dir };

        let mut min_t = None;

        for triangle in &faces.0 {
            let t = triangle.cast_local_ray(&ray, f64::INFINITY, true);

            if let Some(t) = t {
                if t <= min_t.unwrap_or(t) {
                    min_t = Some(t);
                }
            }
        }

        min_t.map(|t| ray.point_at(t))
    }

    pub fn view_transform(&self) -> Transform<f64, TAffine, 3> {
        // Using a mutable variable cleanly takes care of any type inference
        // problems that this operation would otherwise have.
        let mut transform = Transform::identity();

        transform *= Translation::from([
            self.translation.x,
            self.translation.y,
            -self.distance,
        ]);
        transform *= self.rotation;

        transform
    }
}
