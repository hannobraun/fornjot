use nalgebra::Perspective3;

use crate::viewer::camera::Camera;

#[derive(Clone, Copy)]
pub struct Transform {
    inner: fj_math::Transform,
}

impl Transform {
    pub fn identity() -> Self {
        Self::from(fj_math::Transform::identity())
    }

    /// Compute transform used for vertices
    ///
    /// The returned transform is used for transforming vertices on the GPU.
    pub fn for_vertices(camera: &Camera, aspect_ratio: f64) -> Self {
        let field_of_view_in_y = 2.
            * ((camera.field_of_view_in_x() / 2.).tan() / aspect_ratio).atan();

        let transform = {
            let perspective = Perspective3::new(
                aspect_ratio,
                field_of_view_in_y,
                camera.near_plane(),
                camera.far_plane(),
            );

            fj_math::Transform {
                inner: perspective.to_projective()
                    * camera.camera_to_model().inner,
            }
        };

        Self { inner: transform }
    }

    /// Compute transform used for normals
    ///
    /// This method is only relevant for the graphics code. The returned
    /// transform is used for transforming normals on the GPU.
    pub fn for_normals(camera: &Camera) -> Self {
        let transform = camera.camera_to_model().inverse().transpose();

        Self::from(transform)
    }

    pub fn to_native(self) -> [f32; 16] {
        let mut native = [0.; 16];
        native.copy_from_slice(self.inner.data());

        native.map(|v| v as f32)
    }
}

impl From<fj_math::Transform> for Transform {
    fn from(transform: fj_math::Transform) -> Self {
        Self { inner: transform }
    }
}

pub type NativeTransform = [f32; 16];
