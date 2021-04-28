use crate::geometry::operations;

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

pub trait LinearExtrude<Sketch> {
    fn linear_extrude(self, height: f32) -> operations::LinearExtrude<Sketch>;
}

impl<Sketch> LinearExtrude<Sketch> for Sketch {
    fn linear_extrude(self, height: f32) -> operations::LinearExtrude<Sketch> {
        operations::LinearExtrude {
            sketch: self,
            height,
        }
    }
}
