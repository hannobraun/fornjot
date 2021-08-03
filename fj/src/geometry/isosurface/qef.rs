use nalgebra::{Matrix4, Point, SVector};

pub fn find_best_point(
    planes: &[(Point<f32, 3>, SVector<f32, 3>)],
) -> Point<f32, 3> {
    // According to Dual Contouring: "The Secret Sauce", section 2.1, Solving
    // QEFs, we start by initializing a 4x4 matrix to zero.
    let _m = Matrix4::<f32>::zeros();

    for _plane in planes {
        // TASK: Append plane equation of `plane` to the 4x4 matrix, creating a
        //       5x4 matrix. I'm not sure how exactly that's supposed to work
        //       yet, but that's what the paper says.
        // TASK: Perform Givens rotations on the 5x4 matrix to bring it into
        //       upper triangular form. I have no idea how that works.
        // TASK: Figure out what happens to the 5x4 matrix after that. Do we
        //       throw away the last row, to make it a 4x4 matrix again, before
        //       the next loop iteration?
    }

    // TASK: Implement.
    todo!()
}

// TASK: Add unit test. It can be `#[ignore]`d while the function is not
//       implemented yet.
