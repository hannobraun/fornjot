//! Basic triangulation algorithm
//!
//! This is a brute-force algorithm that I've come up with myself, and that's
//! designed to work with exactly the polygons I need it for right now, and not
//! more.

use std::collections::{BTreeMap, BTreeSet};

use decorum::R32;
use nalgebra::Point2;
use parry2d::shape::Triangle;

use crate::geometry::shapes::Polygon;

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

        let p_a = Point2::new(a.0.into_inner(), a.1.into_inner());
        let p_b = Point2::new(b.0.into_inner(), b.1.into_inner());
        let p_c = Point2::new(c.0.into_inner(), c.1.into_inner());

        // Make sure triangles face the right way.
        // TASK: Factor this operation into a method on `Segment`, submit to
        //       Parry.
        let c_is_left_of_a_b = (p_b.x - p_a.x) * (p_c.y - p_a.y)
            - (p_b.y - p_a.y) * (p_c.x - p_a.x)
            > 0.0;
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

struct Neighbors(BTreeMap<(R32, R32), BTreeSet<(R32, R32)>>);

impl Neighbors {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn insert(&mut self, a: Point2<f32>, b: Point2<f32>) {
        let a = a.map(|value| R32::from_inner(value));
        let b = b.map(|value| R32::from_inner(value));

        let a = (a.x, a.y);
        let b = (b.x, b.y);

        self.0.entry(a).or_insert(BTreeSet::new()).insert(b);
        self.0.entry(b).or_insert(BTreeSet::new()).insert(a);
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn first(&self) -> (R32, R32) {
        *self.0.keys().next().unwrap()
    }

    pub fn of(
        &self,
        point: (R32, R32),
    ) -> impl Iterator<Item = (R32, R32)> + '_ {
        self.0.get(&point).unwrap().iter().map(|&point| point)
    }
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
