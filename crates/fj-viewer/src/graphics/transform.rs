use nalgebra::Perspective3;

use crate::camera::Camera;

#[derive(Clone, Copy)]
pub struct Transform {
    inner: fj_core::math::Transform,
}

impl Transform {
    pub fn identity() -> Self {
        Self::from(fj_core::math::Transform::identity())
    }

    /// Compute transform used for vertices
    ///
    /// The returned transform is used for transforming vertices on the GPU.
    pub fn for_vertices(camera: &Camera, aspect_ratio: f64) -> Self {
        let perspective = Perspective3::new(
            aspect_ratio,
            camera.field_of_view_in_y(aspect_ratio),
            camera.near_plane(),
            camera.far_plane(),
        );

        Self {
            inner: fj_core::math::Transform {
                inner: perspective.to_projective()
                    * camera.model_to_camera().inner,
            },
        }
    }

    /// Compute transform used for normals
    ///
    /// This method is only relevant for the graphics code. The returned
    /// transform is used for transforming normals on the GPU.
    pub fn for_normals(camera: &Camera) -> Self {
        let transform = camera.model_to_camera().inverse().transpose();

        Self::from(transform)
    }

    pub fn inner(&self) -> &fj_core::math::Transform {
        &self.inner
    }

    pub fn to_native(self) -> [f32; 16] {
        let mut native = [0.; 16];
        native.copy_from_slice(self.inner.data());

        native.map(|v| v as f32)
    }
}

impl From<fj_core::math::Transform> for Transform {
    fn from(transform: fj_core::math::Transform) -> Self {
        Self { inner: transform }
    }
}

pub type NativeTransform = [f32; 16];
