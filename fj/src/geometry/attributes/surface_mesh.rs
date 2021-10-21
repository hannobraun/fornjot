use crate::geometry::{
    attributes::Edges2,
    shapes::{Mesh, Toroid},
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
    fn surface_mesh(&self, _n: u32) -> Mesh<3> {
        // TASK: Implement.
        todo!()
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
