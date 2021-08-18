use nalgebra::{matrix, vector, DMatrix, MatrixXx4, Point, SVector};

pub fn find_best_point(
    planes: &[(Point<f32, 3>, SVector<f32, 3>)],
) -> Point<f32, 3> {
    // This algorithm is based on Dual Contouring of Hermite Data, section 2.3,
    // and Dual Contouring: "The Secret Sauce", section 2.

    // According to Dual Contouring: "The Secret Sauce", section 2.1, Solving
    // QEFs, we start by initializing a 4x4 matrix to zero.
    //
    // The name `A` is chosen because I don't know a better one, and `A` is a
    // common name for matrices in the math texts I've consulted for this.
    #[allow(non_snake_case)]
    let mut A = MatrixXx4::<f32>::zeros(4);

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
        let i = A.nrows();
        A = A.insert_rows(i, 1, 0.0);
        A.set_row(i, &vector![a, b, c, d].transpose());

        // Convert matrix into upper triangular matrix using Givens rotations.
        for j in 0..A.ncols() {
            for i in (j + 1)..A.nrows() {
                let aᵢⱼ = A[(i, j)];
                let aⱼⱼ = A[(j, j)];

                if aᵢⱼ != 0.0 {
                    // Zero `aᵢⱼ` using a Givens rotation. See Wikipedia for
                    // more information:
                    // https://en.wikipedia.org/wiki/Givens_rotation
                    //
                    // I found that page a bit hard to follow at times, but was
                    // able to fill in the gaps using the equivalent page from
                    // the German Wikipedia:
                    // https://de.wikipedia.org/wiki/Givens-Rotation

                    // Create the Givens rotation matrix G(i, j, θ) where θ is
                    // tailored to zero element `aᵢⱼ`.
                    #[allow(non_snake_case)]
                    let Gᵢⱼ = givens_rotation(A.nrows(), i, j, aᵢⱼ, aⱼⱼ);

                    // Multiply with Givens rotation matrix to zero aᵢⱼ.
                    A = Gᵢⱼ * A;
                }
            }
        }

        // TASK: Figure out what happens to the 5x4 matrix that is created by
        //       the first iteration. Do we throw away the last row, to make it
        //       a 4x4 matrix again, before the next loop iteration?
    }

    #[allow(non_snake_case)]
    let Â = matrix![
        A[(0, 0)], A[(0, 1)], A[(0, 2)];
        A[(1, 0)], A[(1, 1)], A[(1, 2)];
        A[(2, 0)], A[(2, 1)], A[(2, 2)];
    ];
    #[allow(non_snake_case)]
    let B̂ = matrix![
        A[(0, 3)];
        A[(1, 3)];
        A[(2, 3)];
    ];
    let r = A[(3, 3)];

    #[allow(non_snake_case)]
    let AᵀA = Â.transpose() * Â;

    // Compute Singular Value Decomposition of AᵀA.
    // TASK: The paper talks about only the eigenvalues and eigenvectors being
    //       needed to compute the SVD, and makes that sound self-evident and
    //       straight-forward. I don't get any of that, so I just hope the
    //       following is functionally the same, and what the paper refers to is
    //       just an optimization.
    let svd = AᵀA.svd(true, true);
    // TASK: Figure out what a good epsilon value would be. `0.1` is just a
    //       randomly guessed placeholder.
    // TASK: Figure out under which circumstances this could fail, and improve
    //       the error handling.
    let x = svd.solve(&B̂, 0.1).unwrap();

    dbg!(r);

    x.into()
}

fn givens_rotation(
    size: usize,
    i: usize,
    j: usize,
    aᵢⱼ: f32,
    aⱼⱼ: f32,
) -> DMatrix<f32> {
    // This function uses the same parameter names as the Wikipedia page cited
    // above.

    let r = (aⱼⱼ * aⱼⱼ + aᵢⱼ * aᵢⱼ).sqrt() * aⱼⱼ.signum();
    let c = aⱼⱼ / r;
    let s = aᵢⱼ / r;

    #[allow(non_snake_case)]
    let mut Gᵢⱼ = DMatrix::from_element(size, size, 0.0);

    for k in 0..Gᵢⱼ.ncols() {
        for l in 0..Gᵢⱼ.nrows() {
            if k == i && l == i || k == j && l == j {
                Gᵢⱼ[(k, l)] = c;
            }
            if k == i && l == j {
                Gᵢⱼ[(k, l)] = s;
            }
            if k == j && l == i {
                Gᵢⱼ[(k, l)] = -s;
            }
            if k == l && k != i && k != j {
                Gᵢⱼ[(k, l)] = 1.0;
            }

            // In all other cases, the element stays `0.0`.
        }
    }

    Gᵢⱼ
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
