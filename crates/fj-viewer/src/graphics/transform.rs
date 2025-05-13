use bytemuck::{Pod, Zeroable};

use crate::camera::Camera;

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(transparent)]
pub struct Transform(pub [f32; 16]);

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

        let transform = camera.camera_to_model().project_to_array(
            aspect_ratio,
            field_of_view_in_y,
            camera.near_plane(),
            camera.far_plane(),
        );

        Self(transform.map(|scalar| scalar as f32))
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
        let mut native = [0.0; 16];
        native.copy_from_slice(transform.data());

        Self(native.map(|val| val as f32))
    }
}
