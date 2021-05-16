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

#[cfg(test)]
mod tests {
    use crate::geometry::attributes::Distance as _;

    use super::Cylinder;

    #[test]
    fn distance() {
        let cylinder = Cylinder::new().with_radius(0.5).with_height(1.0);

        assert_eq!(cylinder.distance([0.0, 0.0, 0.0]), -0.5);
        assert_eq!(cylinder.distance([0.25, 0.0, 0.0]), -0.25);
        assert_eq!(cylinder.distance([0.0, 0.25, 0.0]), -0.25);
        assert_eq!(cylinder.distance([0.0, 0.0, 0.25]), -0.25);

        assert_eq!(cylinder.distance([1.0, 0.0, 0.0]), 0.5);
        assert_eq!(cylinder.distance([0.0, 1.0, 0.0]), 0.5);
        assert_eq!(cylinder.distance([0.0, 0.0, 1.0]), 0.5);
        assert_eq!(cylinder.distance([0.0, 0.0, -1.0]), 0.5);
    }
}
