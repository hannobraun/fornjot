use nalgebra::{MatrixXx4, Point, SVector};

pub fn find_best_point(
    planes: &[(Point<f32, 3>, SVector<f32, 3>)],
) -> Point<f32, 3> {
    // This algorithm is based on Dual Contouring of Hermite Data, section 2.3,
    // and Dual Contouring: "The Secret Sauce", section 2.

    // According to Dual Contouring: "The Secret Sauce", section 2.1, Solving
    // QEFs, we start by initializing a 4x4 matrix to zero.
    let _m = MatrixXx4::<f32>::from_element(4, 0.0);

    for plane in planes {
        let (point, normal) = plane;

        // According to Dual Contouring: "The Secret Sauce", section 2.1, we're
        // supposed to append the plane equation described by the hermite data
        // of surface-intersecting edges to the matrix.
        //
        // Compute arguments of the plane equation for this plain. This is
        // decently explained on Wikipedia:
        // https://en.wikipedia.org/wiki/Plane_(geometry)#Point%E2%80%93normal_form_and_general_form_of_the_equation_of_a_plane
        // TASK: Figure out that this is what the paper is actually referring
        //       to.
        let a = normal.x;
        let b = normal.y;
        let c = normal.z;
        let _d = -(a * point.x + b * point.y + c * point.z);

        // TASK: Append `a`, `b`, `c`, `d` as a row to the matrix.
        // TASK: Perform Givens rotations on the 5x4 matrix to bring it into
        //       upper triangular form. I have no idea how that works.
        // TASK: Figure out what happens to the 5x4 matrix after that. Do we
        //       throw away the last row, to make it a 4x4 matrix again, before
        //       the next loop iteration?
    }

    // TASK: Implement.
    todo!()
}

// TASK: Check out the Dual Contouring subreddit, it might be helpful:
//       https://www.reddit.com/r/dualcontouring/

#[cfg(test)]
mod tests {
    // TASK: Add unit test with orthogonal planes (sharp edge).
    // TASK: Add unit test for parallel planes (resulting vertex must be within
    //       cube).
}
