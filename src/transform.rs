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

        let m11 = 1.0 / aspect_ratio; // aspect ratio
        let m12 = 0.0;
        let m13 = 0.0;
        let m14 = 0.0;
        let m21 = 0.0;
        let m22 = 1.0;
        let m23 = 0.0;
        let m24 = 0.0;
        let m31 = 0.0;
        let m32 = 0.0;
        let m33 = 1.0;
        let m34 = -1.0; // project points into plane z = -1.0
        let m41 = 0.0;
        let m42 = 0.0;
        let m43 = 0.0;
        let m44 = 0.0;

        #[rustfmt::skip]
        let projection = Transform3D::<f32, (), ()>::new(
            m11, m12, m13, m14,
            m21, m22, m23, m24,
            m31, m32, m33, m34,
            m41, m42, m43, m44,
        );

        // To get a right-handed coordinate system, the camera is looking
        // towards the negative z axis, meaning visible points have a negative z
        // coordinate, relative to the camera. But WebGPU only displays vertices
        // whose z, in normalized device coordinates, is between 0 and 1.
        //
        // To make the vertices that should be visible actually visible, we just
        // need to negate their z coordinate.
        let projection = projection.then_scale(1.0, 1.0, -1.0);

        let transform = view.then(&projection);

        transform.to_arrays()
    }
}

pub type NativeTransform = [[f32; 4]; 4];
