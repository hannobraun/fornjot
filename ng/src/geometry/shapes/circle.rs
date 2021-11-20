use std::f64::consts::PI;

use nalgebra::point;

use crate::{
    geometry::{
        bounding_volume::Aabb,
        edges::Edges,
        faces::{triangulate, Faces, Triangle},
        vertices::Vertices,
        Shape,
    },
    math::Point,
};

impl Shape for fj::Circle {
    fn aabb(&self) -> Aabb {
        Aabb {
            min: point![-self.radius, -self.radius, 0.0],
            max: point![self.radius, self.radius, 0.0],
        }
    }
}

impl Edges for fj::Circle {
    fn edge_vertices(&self, tolerance: f64) -> Vec<Vec<Point>> {
        // To approximate the circle, we use a regular polygon for which the
        // circle is the circumscribed circle. The `tolerance` parameter is the
        // maximum allowed distance between the polygon and the circle. This is
        // the same as the difference between the circumscribed circle and the
        // incircle.
        //
        // Let's figure which regular polygon we need to use, by just trying out
        // some of them until we find one whose maximum error is less than or
        // equal to the tolerance.
        let mut n = 3;
        loop {
            let incircle_radius = self.radius * (PI / n as f64).cos();
            let maximum_error = self.radius - incircle_radius;

            if maximum_error <= tolerance {
                break;
            }

            n += 1;
        }

        let mut vertices = Vec::new();
        for i in 0..n {
            let angle = 2. * PI / n as f64 * i as f64;

            let (sin, cos) = angle.sin_cos();

            let x = cos * self.radius;
            let y = sin * self.radius;

            vertices.push([x, y, 0.].into());
        }

        vec![vertices]
    }
}

impl Faces for fj::Circle {
    fn triangles(&self, tolerance: f64) -> Vec<Triangle> {
        let vertices: Vec<_> = self
            .edge_vertices(tolerance)
            .into_iter()
            .flatten()
            .collect();
        triangulate(&vertices)
    }
}

impl Vertices for fj::Circle {
    fn vertices(&self) -> Vec<Point> {
        // Circles have just a single round edge with no vertices.
        Vec::new()
    }
}
