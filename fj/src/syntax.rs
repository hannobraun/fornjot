pub trait Rotate {
    fn rotate(self, axis: [f64; 3], angle: f64) -> crate::Transform;
}

impl<T> Rotate for T
where
    T: Into<crate::Shape3d>,
{
    fn rotate(self, axis: [f64; 3], angle: f64) -> crate::Transform {
        let shape = self.into();
        crate::Transform {
            shape: Box::new(shape),
            axis,
            angle,
            offset: [0.; 3],
        }
    }
}

pub trait Sweep {
    fn sweep(self, length: f64) -> crate::Sweep;
}

impl<T> Sweep for T
where
    T: Into<crate::Shape2d>,
{
    fn sweep(self, length: f64) -> crate::Sweep {
        let shape = self.into();
        crate::Sweep { shape, length }
    }
}
