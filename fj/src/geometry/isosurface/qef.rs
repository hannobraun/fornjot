use nalgebra::{vector, MatrixXx4, Point, SVector};

pub fn find_best_point(
    planes: &[(Point<f32, 3>, SVector<f32, 3>)],
) -> Point<f32, 3> {
    // This algorithm is based on Dual Contouring of Hermite Data, section 2.3,
    // and Dual Contouring: "The Secret Sauce", section 2.

    // According to Dual Contouring: "The Secret Sauce", section 2.1, Solving
    // QEFs, we start by initializing a 4x4 matrix to zero.
    let mut m = MatrixXx4::<f32>::zeros(4);

    for plane in planes {
        let (point, normal) = plane;

        // According to Dual Contouring: "The Secret Sauce", section 2.1, we're
        // supposed to append the plane equation described by the hermite data
        // of surface-intersecting edges to the matrix.
        //
        // Compute arguments of the plane equation for this plain. This is
        // decently explained on Wikipedia:
        // https://en.wikipedia.org/wiki/Plane_(geometry)#Point%E2%80%93normal_form_and_general_form_of_the_equation_of_a_plane
        // TASK: Figure out whether this is what the paper is actually referring
        //       to.
        let a = normal.x;
        let b = normal.y;
        let c = normal.z;
        let d = -(a * point.x + b * point.y + c * point.z);

        // Append a row consisting of the plane equation values to the matrix.
        let i = m.nrows();
        m = m.insert_rows(i, 1, 0.0);
        m.set_row(i, &vector![a, b, c, d].transpose());

        // Convert matrix into upper triangular matrix using Givens rotations.
        for j in 0..m.ncols() {
            for i in (j + 1)..m.nrows() {
                let element = m[(i, j)];
                if element != 0.0 {
                    // Zero `element` in `m` using a Givens rotation. See
                    // Wikipedia for more information:
                    // https://en.wikipedia.org/wiki/Givens_rotation

                    // TASK: Create a Givens rotation matrix G(i, j, θ) where θ
                    //       is tailored to zero element aᵢⱼ.
                    // TASK: Multiply givens rotation matrix with `m`.
                }
            }
        }

        // TASK: Figure out what happens to the 5x4 matrix that is created by
        //       the first iteration. Do we throw away the last row, to make it
        //       a 4x4 matrix again, before the next loop iteration?
    }

    // TASK: Implement.
    todo!()
}

// TASK: Check out the Dual Contouring subreddit, it might be helpful:
//       https://www.reddit.com/r/dualcontouring/

#[cfg(test)]
mod tests {
    use nalgebra::{point, vector};

    use super::find_best_point;

    // TASK: Un-ignore test.
    #[test]
    #[ignore]
    fn test_perpendicular_planes() {
        let a = (point![0.5, 0.0, 0.0], vector![1.0, 0.0, 0.0]);
        let b = (point![0.0, 0.5, 0.0], vector![0.0, 1.0, 0.0]);
        let c = (point![0.0, 0.0, 0.5], vector![0.0, 0.0, 1.0]);

        let point = find_best_point(&[a, b, c]);
        assert_eq!(point, point![0.5, 0.5, 0.5]);
    }

    // TASK: Un-ignore test.
    #[test]
    #[ignore]
    fn test_parallel_planes() {
        // TASK: Implement. The parallel planes should result in a vertex that
        //       is located within the cube.
        todo!()
    }
}
