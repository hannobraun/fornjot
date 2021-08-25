use nalgebra::{Matrix3, Matrix3x1, Point, SVector};

pub fn place_surface_vertex(
    planes: &[(Point<f32, 3>, SVector<f32, 3>)],
) -> Point<f32, 3> {
    place_at_plane_intersection(planes)
}

#[allow(non_snake_case)]
fn place_at_plane_intersection(
    planes: &[(Point<f32, 3>, SVector<f32, 3>)],
) -> Point<f32, 3> {
    // Based on the approach from https://www.mattkeeter.com/projects/qef/.

    let mut AᵀA = Matrix3::zeros();
    let mut AᵀB = Matrix3x1::zeros();

    for (point, normal) in planes {
        AᵀA += normal * normal.transpose();
        AᵀB += normal * (normal.dot(&point.coords));
    }

    // TASK: Improve error handling.
    let result = AᵀA.svd(true, true).solve(&AᵀB, 0.1).unwrap();

    result.into()
}

// TASK: Check out the Dual Contouring subreddit, it might be helpful:
//       https://www.reddit.com/r/dualcontouring/

#[cfg(test)]
mod tests {
    use nalgebra::{point, vector};

    use super::place_surface_vertex;

    #[test]
    fn test_perpendicular_planes() {
        let a = (point![0.5, 0.0, 0.0], vector![1.0, 0.0, 0.0]);
        let b = (point![0.0, 0.5, 0.0], vector![0.0, 1.0, 0.0]);
        let c = (point![0.0, 0.0, 0.5], vector![0.0, 0.0, 1.0]);

        let point = place_surface_vertex(&[a, b, c]);
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
