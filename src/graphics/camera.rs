// TASK: Consider splitting this method into two modules: `crate::camera` and
//       `graphics::transform`.
//
//       `Camera` arguably contains code that is separate from the core concerns
//       of `graphics`. Once I add more stuff here, like ray casting, this will
//       only become more pronounced. `NativeTransform`, on the other hand, is
//       core to the responsibility of `graphics`.
//
//       I have the following idea:
//       - Convert `to_vertex_transform` and `to_normal_transform` into
//         constructors of `NativeTransform`.
//       - Move `Camera` to new `crate::camera` module.
//       - Rename `graphics::camera` to `graphics::transform`.
//       - Rename `NativeTransform` to `Transform`.

use std::f64::consts::FRAC_PI_2;

use bytemuck::{Pod, Zeroable};
use nalgebra::{Matrix4, Perspective3, TAffine, Transform, Translation};

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
    pub fn new(initial_distance: f64) -> Self {
        Self {
            rotation: Transform::identity(),
            translation: Translation::identity(),
            distance: initial_distance,
        }
    }

    /// Compute transform used for normals
    ///
    /// This method is only relevant for the graphics code. The returned
    /// transform is used for transforming normals on the GPU.
    pub fn to_normal_transform(&self) -> NativeTransform {
        let transform =
            self.view_transform().inverse().to_homogeneous().transpose();

        NativeTransform::from(&transform)
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

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(transparent)]
pub struct NativeTransform(pub [f32; 16]);

impl NativeTransform {
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
}

impl From<&Matrix4<f64>> for NativeTransform {
    fn from(matrix: &Matrix4<f64>) -> Self {
        let mut native = [0.0; 16];
        native.copy_from_slice(matrix.data.as_slice());

        Self(native.map(|val| val as f32))
    }
}

pub const NEAR_PLANE: f64 = 0.1;
pub const FAR_PLANE: f64 = 1000.0;
pub const FIELD_OF_VIEW_IN_X: f64 = FRAC_PI_2; // 90 degrees
