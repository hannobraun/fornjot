use std::collections::VecDeque;

use nalgebra::{Point3, RealField as _};

use crate::geometry::Mesh;

pub struct Circle {
    diameter: f32,
}

impl Circle {
    pub fn from_diameter(diameter: f32) -> Self {
        Self { diameter }
    }

    pub fn from_radius(radius: f32) -> Self {
        Self {
            diameter: radius * 2.0,
        }
    }

    pub fn diameter(&self) -> f32 {
        self.diameter
    }

    pub fn to_mesh(&self, n: u16) -> Option<Mesh> {
        let mut mesh = Mesh::new();

        let center = mesh.vertex(Point3::new(0.0, 0.0, 0.0));

        let radius = self.diameter / 2.0;
        let mut circumference = VecDeque::new();
        for i in 0..n {
            let angle = f32::two_pi() / n as f32 * i as f32;

            let (sin, cos) = angle.sin_cos();

            let x = cos * radius;
            let y = sin * radius;

            let index = mesh.vertex(Point3::new(x, y, 0.0));
            circumference.push_back(index);
        }

        let mut a = circumference.pop_front()?;
        let mut b = circumference.pop_front()?;

        let first = a;

        loop {
            mesh.triangle(center, a, b);

            a = b;
            b = match circumference.pop_front() {
                Some(index) => index,
                None => break,
            };
        }

        mesh.triangle(center, a, first);

        Some(mesh)
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use crate::geometry::Triangles;

    use super::Circle;

    #[test]
    fn circle_should_be_created_from_diameter_and_radius() {
        let circle = Circle::from_diameter(1.0);
        assert_eq!(circle.diameter(), 1.0);

        let circle = Circle::from_radius(1.0);
        assert_eq!(circle.diameter(), 2.0);
    }

    #[test]
    fn circle_should_convert_to_mesh() {
        let circle = Circle::from_radius(1.0);
        let mesh = circle.to_mesh(4).unwrap();

        let triangles = mesh.triangles();

        use crate::geometry::Triangle as T;
        #[rustfmt::skip]
        let expected_triangles = Triangles(vec![
            T::new([0.0, 0.0, 0.0], [ 1.0,  0.0, 0.0], [ 0.0,  1.0, 0.0]),
            T::new([0.0, 0.0, 0.0], [ 0.0,  1.0, 0.0], [-1.0,  0.0, 0.0]),
            T::new([0.0, 0.0, 0.0], [-1.0,  0.0, 0.0], [ 0.0, -1.0, 0.0]),
            T::new([0.0, 0.0, 0.0], [ 0.0, -1.0, 0.0], [ 1.0,  0.0, 0.0]),
        ]);
        assert_relative_eq!(triangles, expected_triangles);
    }
}
