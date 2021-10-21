use std::f32::consts::PI;

use nalgebra::{point, vector, Rotation3};

use crate::{
    geometry::{
        attributes::{Edges2, Vertices as _},
        shapes::{Mesh, Toroid},
    },
    Triangle,
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

impl<T> SurfaceMesh<3> for Toroid<T>
where
    T: Edges2<2>,
{
    fn surface_mesh(&self, n: u32) -> Mesh<3> {
        let edges = self.shape.edges();

        let mut mesh = Mesh::new();
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

        mesh
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::point;

    use crate::geometry::{
        attributes::Edges2,
        shapes::{Edge2, Quad, Toroid},
    };

    use super::SurfaceMesh;

    // TASK: Fix and un-ignore this test.
    #[test]
    #[ignore]
    fn triangle_mesh_for_toroid() {
        struct Square;

        impl Edges2<2> for Square {
            fn edges(&self) -> Vec<Edge2<2>> {
                vec![
                    [point![1., 0.], point![2., 0.]].into(),
                    [point![2., 0.], point![2., 1.]].into(),
                    [point![2., 1.], point![1., 1.]].into(),
                    [point![1., 1.], point![1., 0.]].into(),
                ]
            }
        }

        let toroid = Toroid::from_shape(Square);
        let mut mesh = toroid.surface_mesh(4);
        mesh.round();

        println!("Triangles:");
        for triangle in mesh.triangle_vertices() {
            println!("{}", triangle);
        }

        // TASK: This is probably describing the toroid in the wrong
        //       orientation.
        #[rustfmt::skip]
        let quads = [
            // Inner shell
            [[ 1.,  0., 0.], [ 1.,  0., 1.], [ 0.,  1., 1.], [ 0.,  1., 0.]],
            [[ 0.,  1., 0.], [ 0.,  1., 1.], [-1.,  0., 1.], [-1.,  0., 0.]],
            [[-1.,  0., 0.], [-1.,  0., 1.], [ 0., -1., 1.], [ 0., -1., 0.]],
            [[ 0., -1., 0.], [ 0., -1., 1.], [ 1.,  0., 1.], [ 1.,  0., 0.]],

            // Outer shell
            [[ 2.,  0., 0.], [ 2.,  0., 1.], [ 0.,  2., 1.], [ 0.,  2., 0.]],
            [[ 0.,  2., 0.], [ 0.,  2., 1.], [-2.,  0., 1.], [-2.,  0., 0.]],
            [[-2.,  0., 0.], [-2.,  0., 1.], [ 0., -2., 1.], [ 0., -2., 0.]],
            [[ 0., -2., 0.], [ 0., -2., 1.], [ 2.,  0., 1.], [ 2.,  0., 0.]],
        ];

        for quad in quads {
            println!("Checking {:?}...", quad);
            let quad = Quad::from_points(quad).unwrap();
            assert!(mesh.contains_quad(&quad));
        }
    }
}
