pub trait Rotate {
    /// Create a rotation
    ///
    /// Create a rotation that rotates `shape` by `angle` around an axis defined
    /// by `axis`.
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

pub trait Translate {
    /// Create a translation
    ///
    /// Create a translation that translates `shape` by `offset`.
    fn translate(self, offset: [f64; 3]) -> crate::Transform;
}

impl<T> Translate for T
where
    T: Into<crate::Shape3d>,
{
    fn translate(self, offset: [f64; 3]) -> crate::Transform {
        let shape = self.into();
        crate::Transform {
            shape: Box::new(shape),
            axis: [1., 0., 0.],
            angle: 0.,
            offset,
        }
    }
}
