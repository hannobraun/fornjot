use euclid::{Transform3D, Vector3D};

pub struct Transform {
    pub rotation: Transform3D<f32, (), ()>,
    pub distance: f32,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            rotation: Transform3D::identity(),
            distance: 4.0,
        }
    }

    pub fn to_native(&self, aspect_ratio: f32) -> NativeTransform {
        let view = Transform3D::<f32, (), ()>::identity()
            .then(&self.rotation)
            .then_translate(Vector3D::new(0.0, 0.0, -self.distance));

        let projection = perspective(1.0);

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

/// Creates a perspective projection matrix
///
/// The matrix projects points into a plane defined by `z = -d`.
///
/// After points are projected into this plane, any of them that have
/// coordinates outside the [-1.0, 1.0] range will be clipped by the hardware.
/// Therefore, the distance of the plane (which is equal to the parameter `d`)
/// defines a field of view that gets narrower distance gets smaller, or wider
/// as the distance gets bigger.
fn perspective(d: f32) -> Transform3D<f32, (), ()> {
    let mut t = Transform3D::perspective(d);

    // The `perspective` method sets `m44` to `1.0`. I believe this is a bug,
    // and that the value should be `0.0`.
    //
    // See https://github.com/servo/euclid/pull/465
    t.m44 = 0.0;

    t
}
