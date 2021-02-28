use nalgebra::Point2;
use parry2d::shape::{Segment, Triangle};

use crate::geometry::shapes::Polygon;

use super::neighbors::Neighbors;

pub fn triangulate(polygon: &Polygon) -> Vec<Triangle> {
    let mut neighbors = Neighbors::new();
    for edge in polygon.edges() {
        neighbors.insert(edge.a, edge.b);
    }

    assert!(!neighbors.is_empty());

    let mut triangles = Vec::new();

    loop {
        // Get the first point of our candidate triangle. This shouldn't panic,
        // as we wouldn't be here, if there wasn't at least one item in
        // `neighbors`.
        let a = neighbors.first();

        // Get the other two points of the candidate triangle. This shouldn't
        // panic, as every point must have two neighbors.
        let mut neighbors_of_a = neighbors.of(a);
        let b = neighbors_of_a.next().unwrap();
        let c = neighbors_of_a.next().unwrap();
        drop(neighbors_of_a);

        let p_a: Point2<f32> = a.into();
        let p_b: Point2<f32> = b.into();
        let p_c: Point2<f32> = c.into();

        // Make sure triangles face the right way.
        let a_b = Segment::new(p_a, p_b);
        let a_c = p_c - p_a;
        let c_is_left_of_a_b = a_b.scaled_normal().dot(&a_c) < 0.0;
        if c_is_left_of_a_b {
            triangles.push(Triangle::new(p_a, p_b, p_c));
        } else {
            triangles.push(Triangle::new(p_a, p_c, p_b));
        }

        // Insert the new connection between `b` and `c`.
        neighbors.0.get_mut(&b).unwrap().insert(c);
        neighbors.0.get_mut(&c).unwrap().insert(b);

        // The connections from `a` to its neighbors are on the outside of the
        // triangle. Remove them.
        // TASK: Factor this operation into a method on a `Neighbors` struct.
        neighbors.0.remove(&a);
        for neighbor_set in neighbors.0.values_mut() {
            neighbor_set.remove(&a);
        }

        // Remove any points that don't have enough neighbors left. This will
        // only happen on the last triangle.
        loop {
            let mut to_remove = Vec::new();

            for (point, neighbor_set) in neighbors.0.iter_mut() {
                if neighbor_set.len() < 2 {
                    to_remove.push(*point);
                }
            }

            for point in &to_remove {
                neighbors.0.remove(point);
                for neighbor_set in neighbors.0.values_mut() {
                    neighbor_set.remove(point);
                }
            }

            if to_remove.is_empty() {
                break;
            }
        }

        if neighbors.is_empty() {
            break;
        }
    }

    triangles
}

#[cfg(test)]
mod tests {
    use nalgebra::Point2;
    use parry2d::shape::Triangle;

    use crate::geometry::shapes::{Polygon, VertexChain};

    use super::triangulate;

    #[test]
    fn triangulate_should_triangulate_a_circle_like_polygon() {
        // A very rough circle.
        let p0 = Point2::new(0.0, 0.0);
        let p1 = Point2::new(0.0, 1.0);
        let p2 = Point2::new(1.0, 0.0);
        let p3 = Point2::new(1.0, 1.0);

        let mut chain = VertexChain::new();
        chain.insert(p0);
        chain.insert(p1);
        chain.insert(p2);
        chain.insert(p3);

        let mut polygon = Polygon::new();
        polygon.insert_chain(chain);

        let triangles = triangulate(&polygon);

        let expected =
            vec![Triangle::new(p0, p3, p1), Triangle::new(p1, p2, p3)];
        assert_eq!(triangles, expected);
    }
}
