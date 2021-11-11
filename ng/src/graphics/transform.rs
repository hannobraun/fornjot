use std::f32::consts::FRAC_PI_4;

use nalgebra::{Isometry3, Perspective3, Rotation, Translation};

use crate::geometry::bounding_volume::Aabb;

#[derive(Debug)]
pub struct Transform {
    pub rotation: Rotation<f32, 3>,
    pub translation: Translation<f32, 2>,
    pub distance: f32,
}

impl Transform {
    pub fn new(aabb: Aabb) -> Self {
        // Let's make sure we choose a distance, so that the model fills most of
        // the screen.
        //
        // To do that, first compute the model's highest point, as well as the
        // furthers point from the origin, in x and y.
        let highest_point = aabb.max.z;
        let furthest_point =
            [aabb.min.x.abs(), aabb.max.x, aabb.min.y.abs(), aabb.max.y]
                .into_iter()
                .reduce(|a, b| f32::max(a, b))
                // `reduce` can only return `None`, if there are no items in the
                // iterator. And since we're creating an array full of items
                // above, we know this can't panic.
                .unwrap();

        // The actual furthest point is not far enough. We don't want the model
        // to fill the whole screen.
        let furthest_point = furthest_point * 2.;

        // Having computed those points, figuring out how far the camera needs
        // to be from the model is just a bit of trigonometry.
        let distance_from_model = furthest_point / (FIELD_OF_VIEW / 2.).atan();

        // An finally, the distance from the origin is trivial now.
        let distance = highest_point + distance_from_model;

        Self {
            rotation: Rotation::identity(),
            translation: Translation::identity(),
            distance,
        }
    }

    pub fn to_native(&self, aspect_ratio: f32) -> NativeTransform {
        let projection = Perspective3::new(
            aspect_ratio,
            FIELD_OF_VIEW,
            0.1,    // near plane
            1000.0, // far plane
        );

        let transform = projection.to_projective() * self.view_transform();

        let mut native = [0.0; 16];
        native.copy_from_slice(transform.matrix().data.as_slice());

        native
    }

    pub fn to_normals_transform(&self) -> NativeTransform {
        let transform =
            self.view_transform().inverse().to_homogeneous().transpose();

        let mut native = [0.0; 16];
        native.copy_from_slice(transform.data.as_slice());

        native
    }

    fn view_transform(&self) -> Isometry3<f32> {
        Isometry3::from_parts(
            Translation::from([
                self.translation.x,
                self.translation.y,
                -self.distance,
            ]),
            self.rotation.into(),
        )
    }
}

pub type NativeTransform = [f32; 16];

const FIELD_OF_VIEW: f32 = FRAC_PI_4; // 45 degrees
