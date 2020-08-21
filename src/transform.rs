use euclid::Transform3D;

pub struct Transform;

impl Transform {
    pub fn to_native(&self) -> NativeTransform {
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
        let transform = projection.then_scale(1.0, 1.0, -1.0);

        transform.to_arrays()
    }
}

pub type NativeTransform = [[f32; 4]; 4];
