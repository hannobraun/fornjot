use nalgebra::{
    Isometry3, Perspective3, RealField as _, Rotation, Translation,
};

#[derive(Debug)]
pub struct Transform {
    pub rotation: Rotation<f32, 3>,
    pub distance: f32,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            rotation: Rotation::identity(),
            distance: 400.0,
        }
    }

    pub fn to_native(&self, aspect_ratio: f32) -> NativeTransform {
        let projection = Perspective3::new(
            aspect_ratio,
            f32::frac_pi_4(), // field of view; 45 degrees
            1.0,              // near plane
            1000.0,           // far plane
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
            Translation::from([0.0, 0.0, -self.distance]),
            self.rotation.into(),
        )
    }
}

pub type NativeTransform = [f32; 16];
