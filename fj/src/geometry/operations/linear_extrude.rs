pub struct LinearExtrude<Sketch> {
    pub sketch: Sketch,
    pub height: f32,
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
