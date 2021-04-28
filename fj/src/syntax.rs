use crate::geometry::{operations::linear_extrude::LinearExtrude, Difference};

pub trait MakeDifference<A, B> {
    fn difference(self) -> Difference<A, B>;
}

impl<A, B> MakeDifference<A, B> for (A, B) {
    fn difference(self) -> Difference<A, B> {
        Difference {
            a: self.0,
            b: self.1,
        }
    }
}

pub trait MakeLinearExtrude<Sketch> {
    fn linear_extrude(self, height: f32) -> LinearExtrude<Sketch>;
}

impl<Sketch> MakeLinearExtrude<Sketch> for Sketch {
    fn linear_extrude(self, height: f32) -> LinearExtrude<Sketch> {
        LinearExtrude {
            sketch: self,
            height,
        }
    }
}
