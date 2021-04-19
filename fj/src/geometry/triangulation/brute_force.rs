//! Basic triangulation algorithm
//!
//! This is a brute-force algorithm that I've come up with myself, and that's
//! designed to work with exactly the polygons I need it for right now, and not
//! more.

use crate::geometry::shapes::{Polygon, Tri2};

/// Brute-force polygon triangulation algorithm
///
/// This algorithm handles the polygons that I care about right now, and is fast
/// enough doing so. It makes no guarantees beyond that (so really, it doesn't
/// make any guarantees).
///
/// The reason for this algorithm's existence is to make some forward progress
/// without having to finish the implementation of the Seidel trapezoidation
/// algorithm right now.
pub fn triangulate(mut polygon: Polygon) -> Vec<Tri2> {
    let mut triangles = Vec::new();

    while !polygon.is_empty() {
        // Get the first point of our candidate triangle. This shouldn't panic,
        // as we wouldn't be here, if there wasn't at least one item in
        // `neighbors`.
        let a = polygon.first_vertex().unwrap();

        // Get the other two points of the candidate triangle. This shouldn't
        // panic, as every point must have two neighbors.
        let mut neighbors_of_a = polygon.neighbors_of(&a).unwrap().into_iter();
        let b = neighbors_of_a.next().unwrap();
        let mut c = neighbors_of_a.next().unwrap();
        drop(neighbors_of_a);

        loop {
            let triangle = Tri2::new_ccw(a, b, c);

            let mut lowest_in_triangle = None;
            for vertex in polygon.vertices().iter() {
                if triangle.contains(vertex) {
                    if lowest_in_triangle.unwrap_or(vertex) >= vertex {
                        lowest_in_triangle = Some(vertex);
                    }
                }
            }

            // If there are vertices in the triangle, replace the last triangle
            // point with the lowest of them and try again.
            if let Some(vertex) = lowest_in_triangle {
                c = vertex;
                continue;
            }

            polygon.triangles().remove(triangle).unwrap();
            triangles.push(triangle.into());

            // If we reached this point, the triangle has successfully been
            // removed from the polygon. We can abort the inner loop.
            break;
        }
    }

    triangles
}

#[cfg(test)]
mod tests {
    use crate::geometry::shapes::{Pnt2, Polygon};

    use super::triangulate;

    #[test]
    fn triangulate_should_triangulate_a_polygon() {
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

        let triangles = triangulate(polygon.clone());
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
}
