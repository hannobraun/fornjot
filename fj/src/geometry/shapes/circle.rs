pub struct Circle {
    radius: f32,
}

impl Circle {
    pub fn from_radius(radius: f32) -> Self {
        Self { radius }
    }

    pub fn from_diameter(diameter: f32) -> Self {
        Self {
            radius: diameter / 2.0,
        }
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn diameter(&self) -> f32 {
        self.radius * 2.0
    }
}

#[cfg(test)]
mod tests {
    use super::Circle;

    #[test]
    fn circle_should_be_created_from_diameter_and_radius() {
        let circle = Circle::from_radius(1.0);
        assert_eq!(circle.diameter(), 2.0);

        let circle = Circle::from_diameter(1.0);
        assert_eq!(circle.radius(), 0.5);
    }
}
