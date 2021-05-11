use crate::geometry::operations::LinearExtrude;

use super::Circle;

pub type Cylinder = LinearExtrude<Circle>;

impl Cylinder {
    pub fn new() -> Self {
        LinearExtrude {
            sketch: Circle::new(),
            height: 1.0,
        }
    }

    pub fn with_radius(mut self, radius: f32) -> Self {
        self.sketch = self.sketch.with_radius(radius);
        self
    }
}
