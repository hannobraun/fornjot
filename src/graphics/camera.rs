// TASK: Rename `graphics::camera` to `graphics::transform`.

use std::f64::consts::FRAC_PI_2;

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
        let field_of_view_y = FIELD_OF_VIEW_IN_X / aspect_ratio;

        let projection = Perspective3::new(
            aspect_ratio,
            field_of_view_y,
            NEAR_PLANE,
            FAR_PLANE,
        );

        let transform = projection.to_projective() * camera.view_transform();

        Self::from(transform.matrix())
    }

    /// Compute transform used for normals
    ///
    /// This method is only relevant for the graphics code. The returned
    /// transform is used for transforming normals on the GPU.
    pub fn for_normals(camera: &Camera) -> Self {
        let transform = camera
            .view_transform()
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

pub const NEAR_PLANE: f64 = 0.1;
pub const FAR_PLANE: f64 = 1000.0;
pub const FIELD_OF_VIEW_IN_X: f64 = FRAC_PI_2; // 90 degrees
