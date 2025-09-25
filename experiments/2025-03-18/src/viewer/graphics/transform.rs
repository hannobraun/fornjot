use bytemuck::{Pod, Zeroable};
use nalgebra::Perspective3;

use crate::viewer::camera::Camera;

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(transparent)]
pub struct Transform {
    inner: NativeTransform,
}

impl Transform {
    pub fn identity() -> Self {
        Self::from(&fj_math::Transform::identity())
    }

    /// Compute transform used for vertices
    ///
    /// The returned transform is used for transforming vertices on the GPU.
    pub fn for_vertices(camera: &Camera, aspect_ratio: f64) -> Self {
        let field_of_view_in_y = 2.
            * ((camera.field_of_view_in_x() / 2.).tan() / aspect_ratio).atan();

        let transform = {
            let projection = Perspective3::new(
                aspect_ratio,
                field_of_view_in_y,
                camera.near_plane(),
                camera.far_plane(),
            );

            let mut array = [0.; 16];
            array.copy_from_slice(
                (projection.to_projective() * camera.camera_to_model().inner)
                    .matrix()
                    .as_slice(),
            );

            array
        };

        Self {
            inner: transform.map(|scalar| scalar as f32),
        }
    }

    /// Compute transform used for normals
    ///
    /// This method is only relevant for the graphics code. The returned
    /// transform is used for transforming normals on the GPU.
    pub fn for_normals(camera: &Camera) -> Self {
        let transform = camera.camera_to_model().inverse().transpose();

        Self::from(&transform)
    }
}

impl From<&fj_math::Transform> for Transform {
    fn from(transform: &fj_math::Transform) -> Self {
        let mut native = [0.; 16];
        native.copy_from_slice(transform.data());

        Self {
            inner: native.map(|val| val as f32),
        }
    }
}

pub type NativeTransform = [f32; 16];
