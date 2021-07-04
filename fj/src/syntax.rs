use crate::{geometry::operations, model};

pub trait Difference<A, B> {
    fn difference(self) -> operations::Difference<A, B>;
}

impl<A, B> Difference<A, B> for (A, B) {
    fn difference(self) -> operations::Difference<A, B> {
        operations::Difference {
            a: self.0,
            b: self.1,
        }
    }
}

pub trait Sweep<Sketch> {
    fn sweep(self, distance: f32) -> operations::Sweep<Sketch>;
}

impl<Sketch> Sweep<Sketch> for Sketch {
    fn sweep(self, distance: f32) -> operations::Sweep<Sketch> {
        operations::Sweep {
            sketch: self,
            distance,
        }
    }
}

pub trait Resolution: Sized {
    fn resolution(self, resolution: f32) -> model::WithResolution<Self> {
        model::WithResolution {
            geometry: self,
            resolution,
        }
    }
}

impl<Geometry> Resolution for Geometry {}
