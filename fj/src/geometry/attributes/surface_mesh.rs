use crate::geometry::{
    attributes::Edges,
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
    T: Edges<2>,
{
    fn surface_mesh(&self, _n: u32) -> Mesh<3> {
        // TASK: Implement.
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::{FRAC_PI_2, PI};

    use nalgebra::{vector, Isometry2};

    use crate::{
        geometry::{
            attributes::Edges,
            operations,
            shapes::{Edge, Quad, Toroid},
        },
        math,
        syntax::Transform as _,
    };

    use super::SurfaceMesh;

    #[test]
    #[ignore]
    fn triangle_mesh_for_toroid() {
        struct Square;

        impl Edges<2> for Square {
            fn edges(&self) -> Vec<operations::Transform<Edge, 2>> {
                vec![
                    Edge::new().transform(
                        math::Transform::identity()
                            * Isometry2::new(vector![1., 0.], FRAC_PI_2),
                    ),
                    Edge::new().transform(
                        math::Transform::identity()
                            * Isometry2::new(vector![2., 0.], FRAC_PI_2),
                    ),
                    Edge::new().transform(
                        math::Transform::identity()
                            * Isometry2::new(vector![2., 1.], PI),
                    ),
                    Edge::new().transform(
                        math::Transform::identity()
                            * Isometry2::new(vector![1., 1.], FRAC_PI_2 * 3.),
                    ),
                ]
            }
        }

        let toroid = Toroid::from_shape(Square);
        let mesh = toroid.surface_mesh(4);

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
            let quad = Quad::new(quad).unwrap();
            assert!(mesh.contains_quad(&quad));
        }
    }
}
