use parry3d_f64::bounding_volume::AABB;

use crate::{
    kernel::{
        edges::Edges,
        faces::{triangulate, Face, Faces},
        Shape,
    },
    math::Point, debug::DebugInfo,
};

impl Shape for fj::Difference2d {
    fn bounding_volume(&self) -> AABB {
        // This is a conservative estimate of the bounding box: It's never going
        // to be bigger than the bounding box of the original shape that another
        // is being subtracted from.
        self.a.bounding_volume()
    }

    fn faces(&self, tolerance: f64, _: &mut DebugInfo) -> Faces {
        // TASK: Carefully think about the limits of this algorithm, and make
        //       sure to panic with a `todo!` in cases that are not supported.

        let a: Vec<_> = self
            .a
            .edges()
            .cycles
            .into_iter()
            .map(|cycle| cycle.edges)
            .flatten()
            .map(|edge| edge.approx_vertices(tolerance))
            .flatten()
            .collect();
        let b: Vec<_> = self
            .b
            .edges()
            .cycles
            .into_iter()
            .map(|cycle| cycle.edges)
            .flatten()
            .map(|edge| edge.approx_vertices(tolerance))
            .flatten()
            .collect();

        let mut vertices = Vec::new();
        vertices.extend(&a);
        vertices.extend(&b);

        let mut triangles = triangulate(&vertices);

        // Now we have a full Delaunay triangulation of all vertices. We still
        // need to filter out the triangles that aren't actually part of the
        // difference.
        triangles.retain(|triangle| {
            let mut edges_of_b = 0;

            for segment in triangle.edges() {
                if b.contains(&segment.a) && b.contains(&segment.b) {
                    edges_of_b += 1;
                }
            }

            edges_of_b <= 1
        });

        Faces(vec![Face::Triangles(triangles)])
    }

    fn edges(&self) -> Edges {
        // TASK: This method assumes that `b` is fully contained within `a`. As
        //       long as this precondition exists, it should at least be
        //       checked.

        let mut a = self.a.edges();
        let mut b = self.b.edges();

        let (a, mut b) = if a.cycles.len() == 1 && b.cycles.len() == 1 {
            (a.cycles.pop().unwrap(), b.cycles.pop().unwrap())
        } else {
            // TASK: Open issue, link it in the error message.
            todo!(
                "The 2-dimensional difference operation only supports one \
                cycle in each operand."
            );
        };

        for edge in &mut b.edges {
            edge.reverse();
        }

        Edges { cycles: vec![a, b] }
    }

    fn vertices(&self) -> Vec<Point> {
        // TASK: Implement.
        todo!()
    }
}
