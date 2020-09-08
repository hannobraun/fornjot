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
        let n = 0.1; // distance of near plane
        let f = 100.0; // distance of far plane

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
        let m33 = -f / (f - n); // normalize z between near/far planes
        let m34 = -1.0;

        let m41 = 0.0;
        let m42 = 0.0;
        let m43 = -f * n / (f - n); // normalize z between near/far planes
        let m44 = 0.0;

        // The resulting projection matrix has the following attributes:
        // - Projects points on the plane defined by `z = -n`.
        // - Normalizes z, with `z = -n` becoming 0, `z = -f` becoming 1.
        #[rustfmt::skip]
        let projection = Transform3D::<f32, (), ()>::new(
            m11, m12, m13, m14,
            m21, m22, m23, m24,
            m31, m32, m33, m34,
            m41, m42, m43, m44,
        );

        self.view_transform().then(&projection).to_arrays()
    }

    pub fn to_normals_transform(&self) -> NativeTransform {
        self.view_transform()
            .inverse()
            .expect("view transform was not invertible")
            .to_arrays_transposed()
    }

    fn view_transform(&self) -> Transform3D<f32, (), ()> {
        Transform3D::identity()
            .then(&self.rotation)
            .then_translate(Vector3D::new(0.0, 0.0, -self.distance))
    }
}

pub type NativeTransform = [[f32; 4]; 4];
