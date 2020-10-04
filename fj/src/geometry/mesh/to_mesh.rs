use std::{collections::VecDeque, f32::consts::PI};

use nalgebra::{Point3, RealField as _};

use crate::geometry::{Circle, Mesh, Triangle};

pub trait ToMesh {
    fn to_mesh(self, tolerance: f32) -> Mesh;
}

impl ToMesh for &Circle {
    fn to_mesh(self, tolerance: f32) -> Mesh {
        // To approximate the circle, we use a regular polygon for which the
        // cirle is the circumscribed circle. The `tolerance` parameter is the
        // maximum allowed distance between the polygon and the circle. This is
        // the same as the difference between the circumscribed circle and the
        // in circle.
        //
        // Let's figure which regular polygon we need to use, by just trying out
        // some of them until we find one whose maximum error is less than or
        // equal to the tolerance.
        let mut n = 3;
        loop {
            let incircle_radius = self.radius() * (PI / n as f32).cos();
            let maximum_error = self.radius() - incircle_radius;

            println!("{}, {}", tolerance, maximum_error);

            if maximum_error <= tolerance {
                break;
            }

            n += 1;
        }

        let mut mesh = Mesh::new();

        let center = mesh.vertex(Point3::new(0.0, 0.0, 0.0));

        let mut circumference = VecDeque::new();
        for i in 0..n {
            let angle = f32::two_pi() / n as f32 * i as f32;

            let (sin, cos) = angle.sin_cos();

            let x = cos * self.radius();
            let y = sin * self.radius();

            let index = mesh.vertex(Point3::new(x, y, 0.0));
            circumference.push_back(index);
        }

        // We know that `n` is at least 3, so the following can't panic.
        let mut a = circumference.pop_front().unwrap();
        let mut b = circumference.pop_front().unwrap();

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

        mesh
    }
}

impl ToMesh for &Triangle {
    fn to_mesh(self, _tolerance: f32) -> Mesh {
        let mut mesh = Mesh::new();

        let i0 = mesh.vertex(self.a);
        let i1 = mesh.vertex(self.b);
        let i2 = mesh.vertex(self.c);

        mesh.triangle(i0, i1, i2);

        mesh
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;

    use crate::geometry::{Circle, Triangle, Triangles};

    use super::ToMesh as _;

    #[test]
    fn circle_should_convert_to_mesh() {
        // If we approximate the circle using a triangle whose points are on the
        // circle, the maximum error (distance between circle and triangle) is
        // 0.5. The maximum error for a square is roughly 0.3, so choosing a
        // tolerance between those two should give us a square.
        let tolerance = 0.4;

        let circle = Circle::from_radius(1.0);
        let mesh = circle.to_mesh(tolerance);

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

    #[test]
    fn test() {
        let triangle =
            Triangle::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);

        let mesh = triangle.to_mesh(0.0);
        let triangles = mesh.triangles();

        assert_eq!(triangles.0, vec![triangle]);
    }
}
