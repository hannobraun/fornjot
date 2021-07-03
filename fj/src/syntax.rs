use crate::{geometry::operations, mesh::into_mesh};

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
    fn resolution(self, resolution: f32) -> into_mesh::WithResolution<Self> {
        into_mesh::WithResolution {
            geometry: self,
            resolution,
        }
    }
}

impl<Geometry> Resolution for Geometry {}
