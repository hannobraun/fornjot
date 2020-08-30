use euclid::{Angle, Transform3D, Vector3D};

pub struct Transform {
    pub angle_x: Angle<f32>,
    pub angle_z: Angle<f32>,
    pub distance: f32,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            angle_x: Angle::frac_pi_4(),
            angle_z: Angle::frac_pi_4(),
            distance: 4.0,
        }
    }

    pub fn to_native(&self, aspect_ratio: f32) -> NativeTransform {
        let view = Transform3D::<f32, (), ()>::identity()
            .then_rotate(1.0, 0.0, 0.0, self.angle_x)
            .then_rotate(0.0, 0.0, 1.0, self.angle_z)
            .then_translate(Vector3D::new(0.0, 0.0, -self.distance));

        // Create perspective projection, which projects points into a plane
        // coplanar with the x-y plane, that has the given distance from the
        // origin (our camera), in negative direction along the z axis.
        let distance = 1.0;
        let mut projection = Transform3D::<f32, (), ()>::perspective(distance);

        // The `perspective` method sets `m44` to `1.0`. This is a bug, it
        // should be `0.0`.
        //
        // See https://github.com/servo/euclid/pull/465
        projection.m44 = 0.0;

        // To get a right-handed coordinate system, the camera is looking
        // towards the negative z axis, meaning visible points have a negative z
        // coordinate, relative to the camera. But WebGPU only displays vertices
        // whose z, in normalized device coordinates, is between 0 and 1.
        //
        // To make the vertices that should be visible actually visible, we just
        // need to negate their z coordinate.
        let projection = projection.then_scale(1.0, 1.0, -1.0);

        let projection = projection.then_scale(1.0 / aspect_ratio, 1.0, 1.0);

        let transform = view.then(&projection);

        transform.to_arrays()
    }
}

pub type NativeTransform = [[f32; 4]; 4];
