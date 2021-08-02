use nalgebra::{Matrix4, Point, SVector};

pub fn find_best_point(
    _plains: &[(Point<f32, 3>, SVector<f32, 3>)],
) -> Point<f32, 3> {
    // According to Dual Contouring: "The Secret Sauce", section 2.1, Solving
    // QEFs, we start by initializing a 4x4 matrix to zero.
    let _m = Matrix4::<f32>::zeros();

    // TASK: Implement.
    todo!()
}
