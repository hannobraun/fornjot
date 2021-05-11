pub struct LinearExtrude<Sketch> {
    pub sketch: Sketch,
    pub height: f32,
}

impl<Sketch> LinearExtrude<Sketch> {
    pub fn with_sketch(mut self, sketch: Sketch) -> Self {
        self.sketch = sketch;
        self
    }

    pub fn with_height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }
}
