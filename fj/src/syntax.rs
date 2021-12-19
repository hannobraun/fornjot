pub trait Sweep {
    fn sweep(self, length: f64) -> crate::Sweep;
}

impl<T> Sweep for T
where
    T: Into<crate::Shape2d>,
{
    fn sweep(self, length: f64) -> crate::Sweep {
        let shape = self.into();
        let sweep = crate::Sweep { shape, length };
        sweep.into()
    }
}
