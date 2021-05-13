pub struct Sphere {
    pub radius: f32,
}

impl Sphere {
    pub fn new() -> Self {
        Self { radius: 1.0 }
    }

    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }
}
