use euclid::Transform3D;

pub struct Transform;

impl Transform {
    pub fn to_native(&self) -> NativeTransform {
        let transform = Transform3D::<f32, (), ()>::identity();

        transform.to_arrays()
    }
}

pub type NativeTransform = [[f32; 4]; 4];
