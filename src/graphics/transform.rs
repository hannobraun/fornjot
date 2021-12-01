use std::f64::consts::FRAC_PI_4;

use bytemuck::{Pod, Zeroable};
use nalgebra::{Isometry3, Matrix4, Perspective3, Rotation, Translation};

#[derive(Debug)]
pub struct Transform {
    pub rotation: Rotation<f64, 3>,
    pub translation: Translation<f64, 2>,
    pub distance: f64,
}

impl Transform {
    pub fn new(initial_distance: f64) -> Self {
        Self {
            rotation: Rotation::identity(),
            translation: Translation::identity(),
            distance: initial_distance,
        }
    }

    pub fn to_native(&self, aspect_ratio: f64) -> NativeTransform {
        let projection = Perspective3::new(
            aspect_ratio,
            FIELD_OF_VIEW,
            NEAR_PLANE,
            FAR_PLANE,
        );

        let transform = projection.to_projective() * self.view_transform();

        NativeTransform::from_matrix(transform.matrix())
    }

    pub fn to_normals_transform(&self) -> NativeTransform {
        let transform =
            self.view_transform().inverse().to_homogeneous().transpose();

        NativeTransform::from_matrix(&transform)
    }

    pub fn view_transform(&self) -> Isometry3<f64> {
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

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(transparent)]
pub struct NativeTransform(pub [f32; 16]);

impl NativeTransform {
    pub fn identity() -> Self {
        Self::from_matrix(&Matrix4::identity())
    }

    pub fn from_matrix(transform: &Matrix4<f64>) -> Self {
        let mut native = [0.0; 16];
        native.copy_from_slice(transform.data.as_slice());

        Self(native.map(|val| val as f32))
    }
}

pub const NEAR_PLANE: f64 = 0.1;
pub const FAR_PLANE: f64 = 1000.0;
pub const FIELD_OF_VIEW: f64 = FRAC_PI_4; // 45 degrees
