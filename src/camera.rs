use nalgebra::{TAffine, Transform, Translation};

use crate::{geometry::bounding_volume::Aabb, graphics::FIELD_OF_VIEW_IN_X};

/// The camera abstraction
///
/// Please note that the metaphor we're using (which influences how mouse input
/// is handled, for example) is not that of a camera freely flying through a
/// static scene. Instead, the camera is static, and the model is freely
/// translated and rotated.
#[derive(Debug)]
pub struct Camera {
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

    pub fn new(aabb: &Aabb) -> Self {
        let initial_distance = {
            // Let's make sure we choose a distance, so that the model fills
            // most of the screen.
            //
            // To do that, first compute the model's highest point, as well as
            // the furthest point from the origin, in x and y.
            let highest_point = aabb.max.z;
            let furthest_point =
                [aabb.min.x.abs(), aabb.max.x, aabb.min.y.abs(), aabb.max.y]
                    .into_iter()
                    .reduce(|a, b| f64::max(a, b))
                    // `reduce` can only return `None`, if there are no items in the
                    // iterator. And since we're creating an array full of items
                    // above, we know this can't panic.
                    .unwrap();

            // The actual furthest point is not far enough. We don't want the model
            // to fill the whole screen.
            let furthest_point = furthest_point * 2.;

            // Having computed those points, figuring out how far the camera needs
            // to be from the model is just a bit of trigonometry.
            let distance_from_model =
                furthest_point / (FIELD_OF_VIEW_IN_X / 2.).atan();

            // An finally, the distance from the origin is trivial now.
            highest_point + distance_from_model
        };

        Self {
            rotation: Transform::identity(),
            translation: Translation::identity(),
            distance: initial_distance,
        }
    }

    pub fn near_plane(&self) -> f64 {
        Self::INITIAL_NEAR_PLANE
    }

    pub fn far_plane(&self) -> f64 {
        Self::INITIAL_FAR_PLANE
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
