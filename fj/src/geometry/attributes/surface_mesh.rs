use std::f32::consts::PI;

use nalgebra::{point, vector, Rotation3};

use crate::geometry::{
    attributes::{Edges, Vertices as _},
    shapes::{self, mesh::MeshMaker, Mesh, Triangle},
};

/// Compute a triangle mesh that approximates a shape's surface
pub trait SurfaceMesh<const D: usize> {
    /// Compute surface mesh for shape
    ///
    /// If the surface mesh can only be approximated, `n` defines the number of
    /// computational steps taken to compute the mesh.
    // TASK: Replace `n` with a tolerance value.
    fn surface_mesh(&self, n: u32) -> Mesh<D>;
}

impl<T> SurfaceMesh<3> for shapes::Toroid<T>
where
    T: Edges<2>,
{
    fn surface_mesh(&self, n: u32) -> Mesh<3> {
        let edges = self.shape.edges();

        let mut mesh = MeshMaker::new();
        let mut angle = 0.;

        for _ in 0..n {
            let next_angle = angle + PI * 2. / n as f32;

            let rot_curr = Rotation3::new(vector![0., angle, 0.]);
            let rot_next = Rotation3::new(vector![0., next_angle, 0.]);

            for edge in &edges {
                let vertices = edge.vertices();

                // An edge has exactly two vertices, so this isn't going to
                // panic.
                let v1 = vertices[0].offset;
                let v2 = vertices[1].offset;

                let v1 = point![v1.x, v1.y, 0.];
                let v2 = point![v2.x, v2.y, 0.];

                let a = rot_curr.transform_point(&v1);
                let b = rot_curr.transform_point(&v2);
                let c = rot_next.transform_point(&v2);
                let d = rot_next.transform_point(&v1);

                // TASK: Decide how to handle panics. An inner point could be on
                //       the origin, then rotating it would do nothing.
                mesh.triangle(Triangle::from_points([a, b, c]).unwrap());
                mesh.triangle(Triangle::from_points([a, c, d]).unwrap());
            }

            angle = next_angle;
        }

        mesh.make()
    }
}
