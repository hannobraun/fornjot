//! Basic triangulation algorithm
//!
//! This is a brute-force algorithm that I've come up with myself, and that's
//! designed to work with exactly the polygons I need it for right now, and not
//! more.

use rand::{seq::SliceRandom as _, thread_rng};
use thiserror::Error;
use tracing::debug;

use crate::geometry::shapes::{polygon, Polygon, Tri2};

/// Brute-force polygon triangulation algorithm
///
/// This algorithm handles the polygons that I care about right now, and is fast
/// enough doing so. It makes no guarantees beyond that (so really, it doesn't
/// make any guarantees).
///
/// The reason for this algorithm's existence is to make some forward progress
/// without having to finish the implementation of the Seidel trapezoidation
/// algorithm right now.
pub fn triangulate(mut polygon: Polygon) -> Result<Vec<Tri2>, InternalError> {
    debug!("Triangulating polygon: {}", polygon);

    let mut rng = thread_rng();

    let mut triangles = Vec::new();

    while !polygon.is_empty() {
        // Get the first point of our candidate triangle. This shouldn't panic,
        // as we just determined that the polygon isn't empty.
        let a = polygon.vertices().iter().next().unwrap();

        // Get the other two points of the candidate triangle.
        let mut neighbors_of_a: Vec<_> =
            polygon.vertices().neighbors_of(&a).into_iter().collect();

        // This shouldn't panic, as every point must have at least two
        // neighbors.
        let mut b = neighbors_of_a[0];
        let mut c = neighbors_of_a[1];

        loop {
            let triangle = Tri2::new_ccw(a, b, c);

            debug!("Candidate triangle: {}", triangle);

            let mut lowest_in_triangle = None;
            for vertex in polygon.vertices().iter() {
                if triangle.contains(vertex) {
                    debug!("Triangle contains vertex: {}", vertex);

                    if lowest_in_triangle.unwrap_or(vertex) >= vertex {
                        lowest_in_triangle = Some(vertex);
                    }
                }
            }

            if let Some(vertex) = lowest_in_triangle {
                debug!("Lowest vertex contained in triangle: {}", vertex);
            }

            // If there are vertices in the triangle, replace the last triangle
            // point with the lowest of them and try again.
            if let Some(vertex) = lowest_in_triangle {
                c = vertex;
                continue;
            }

            match polygon.triangles().remove(triangle) {
                Ok(()) => {
                    // We removed a triangle from the polygon.
                    triangles.push(triangle.into());
                }
                Err(polygon::triangles::RemoveError::OutsideOfPolygon(_)) => {
                    // We selected a triangle that is outside of the polygon.
                    // Shuffle neighbors, try again.
                    neighbors_of_a.shuffle(&mut rng);
                    b = neighbors_of_a[0];
                    c = neighbors_of_a[1];

                    continue;
                }
                Err(err) => {
                    // Other errors are a bug. Properly report this to the
                    // caller.
                    return Err(InternalError {
                        triangle,
                        polygon,
                        cause: err,
                    });
                }
            }

            debug!("Removed triangle. Updated polygon: {}", polygon);

            // If we reached this point, the triangle has successfully been
            // removed from the polygon. We can abort the inner loop.
            break;
        }
    }

    Ok(triangles)
}

#[derive(Debug, Error)]
#[error(
    "BUG - Error while removing triangle {triangle} from polygon\n{polygon}"
)]
pub struct InternalError {
    pub triangle: Tri2,
    pub polygon: Polygon,

    #[source]
    pub cause: polygon::triangles::RemoveError,
}

#[cfg(test)]
mod tests {
    use crate::geometry::shapes::{Pnt2, Polygon};

    use super::triangulate;

    #[test]
    fn triangulate_should_triangulate_polygon_with_hole() {
        let mut polygon = Polygon::new();

        // A very rough circle.
        let p0 = Pnt2::new(0.0, 0.0);
        let p1 = Pnt2::new(2.0, 0.0);
        let p2 = Pnt2::new(2.0, 2.0);
        let p3 = Pnt2::new(0.0, 2.0);
        polygon.insert_chain(&[p0, p1, p2, p3]);

        // A roughly circular hole.
        let p0 = Pnt2::new(0.5, 0.5);
        let p1 = Pnt2::new(0.5, 1.0);
        let p2 = Pnt2::new(1.0, 1.0);
        let p3 = Pnt2::new(1.0, 0.5);
        polygon.insert_chain(&[p0, p1, p2, p3]);

        println!("Original polygon: {:#?}", polygon);

        let triangles = triangulate(polygon.clone()).unwrap();
        for triangle in triangles {
            polygon.triangles().remove(triangle).unwrap();

            println!("Removed triangle: {:#?}", triangle);
            println!("Updated polygon: {:#?}", polygon);
        }

        println!("Empty polygon: {:#?}", polygon);

        // We removed all the triangles from the polygon, and if we reach that
        // point, this succeeded. This means, the algorithm didn't generate any
        // triangles that are not in the polygon.
        //
        // If the polygon is now empty, this means the algorithm also generated
        // all of the triangles that made up the polygon.
        assert!(polygon.is_empty());
    }

    #[test]
    fn triangulate_should_handle_selected_triangle_being_outside_of_polygon() {
        let mut polygon = Polygon::new();

        // Common point
        let p0 = Pnt2::new(0.0, 0.0);

        // Outer border
        let p1 = Pnt2::new(2.0, -2.0);
        let p2 = Pnt2::new(3.0, 0.0);
        let p3 = Pnt2::new(2.0, 2.0);
        polygon.insert_chain(&[p0, p1, p2, p3]);

        // Inner border
        let p1 = Pnt2::new(1.0, 0.5);
        let p2 = Pnt2::new(2.0, 0.0);
        let p3 = Pnt2::new(1.0, -0.5);
        polygon.insert_chain(&[p0, p1, p2, p3]);

        // The three "lowest" points belong to the hole. The triangle that's
        // selected first thus doesn't belong to the polygon.

        let triangles = triangulate(polygon.clone()).unwrap();
        for triangle in triangles {
            polygon.triangles().remove(triangle).unwrap();
        }

        assert!(polygon.is_empty());
    }
}
