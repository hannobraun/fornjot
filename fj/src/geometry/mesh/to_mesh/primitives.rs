use std::f32::consts::PI;

use nalgebra::Point3;

use crate::geometry::{
    shapes::Polygon, triangulation::brute_force::triangulate, Boundary as _,
    Circle, Mesh, Triangle3,
};

use super::ToMesh;

impl ToMesh for &Circle {
    fn to_mesh(self, tolerance: f32, mesh: &mut Mesh) {
        // To approximate the circle, we use a regular polygon for which the
        // circle is the circumscribed circle. The `tolerance` parameter is the
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

            if maximum_error <= tolerance {
                break;
            }

            n += 1;
        }

        let mut circumference = Vec::new();
        for i in 0..n {
            let p = self.boundary(1.0 / n as f32 * i as f32);
            circumference.push(p);
        }

        let mut polygon = Polygon::new();
        polygon.insert_chain(circumference);

        polygon.to_mesh(tolerance, mesh);
    }
}

impl ToMesh for &Polygon {
    fn to_mesh(self, _tolerance: f32, mesh: &mut Mesh) {
        let triangles = triangulate(self);

        for triangle in triangles {
            let a_x: f32 = triangle.a.x.into();
            let a_y: f32 = triangle.a.y.into();
            let b_x: f32 = triangle.b.x.into();
            let b_y: f32 = triangle.b.y.into();
            let c_x: f32 = triangle.c.x.into();
            let c_y: f32 = triangle.c.y.into();

            let a = mesh.vertex(Point3::new(a_x, a_y, 0.0));
            let b = mesh.vertex(Point3::new(b_x, b_y, 0.0));
            let c = mesh.vertex(Point3::new(c_x, c_y, 0.0));

            mesh.triangle(a, b, c);
        }
    }
}

impl ToMesh for &Triangle3 {
    fn to_mesh(self, _tolerance: f32, mesh: &mut Mesh) {
        let i0 = mesh.vertex(self.a);
        let i1 = mesh.vertex(self.b);
        let i2 = mesh.vertex(self.c);

        mesh.triangle(i0, i1, i2);
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::{Mesh, Triangle3};

    use crate::geometry::ToMesh as _;

    #[test]
    fn triangle_should_convert_to_mesh() {
        let triangle =
            Triangle3::new([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]);

        let mut mesh = Mesh::new();
        triangle.to_mesh(0.0, &mut mesh);

        let triangles = mesh.triangles();

        assert_eq!(triangles.0, vec![triangle]);
    }
}
