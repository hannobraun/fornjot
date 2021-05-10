// TASK: Create `Cylinder` using a circle primitive and linear extrusion.
pub struct Cylinder {
    pub radius: f32,
    pub height: f32,
}

impl Cylinder {
    pub fn new() -> Self {
        Self {
            radius: 1.0,
            height: 1.0,
        }
    }

    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    pub fn with_height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }
}
