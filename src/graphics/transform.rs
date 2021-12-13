use bytemuck::{Pod, Zeroable};
use nalgebra::{Matrix4, Perspective3};

use crate::camera::Camera;

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(transparent)]
pub struct Transform(pub [f32; 16]);

impl Transform {
    pub fn identity() -> Self {
        Self::from(&Matrix4::identity())
    }

    /// Compute transform used for vertices
    ///
    /// The returned transform is used for transforming vertices on the GPU.
    pub fn for_vertices(camera: &Camera, aspect_ratio: f64) -> Self {
        let field_of_view_in_y = camera.field_of_view_in_x() / aspect_ratio;

        let projection = Perspective3::new(
            aspect_ratio,
            field_of_view_in_y,
            camera.near_plane(),
            camera.far_plane(),
        );

        let transform = projection.to_projective() * camera.camera_to_model();

        Self::from(transform.matrix())
    }

    /// Compute transform used for normals
    ///
    /// This method is only relevant for the graphics code. The returned
    /// transform is used for transforming normals on the GPU.
    pub fn for_normals(camera: &Camera) -> Self {
        let transform = camera
            .camera_to_model()
            .inverse()
            .to_homogeneous()
            .transpose();

        Self::from(&transform)
    }
}

impl From<&Matrix4<f64>> for Transform {
    fn from(matrix: &Matrix4<f64>) -> Self {
        let mut native = [0.0; 16];
        native.copy_from_slice(matrix.data.as_slice());

        Self(native.map(|val| val as f32))
    }
}
