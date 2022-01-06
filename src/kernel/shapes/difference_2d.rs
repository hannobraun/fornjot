use parry3d_f64::bounding_volume::AABB;

use crate::{
    debug::DebugInfo,
    kernel::{
        topology::{
            edges::Edges,
            faces::{Face, Faces},
        },
        Shape,
    },
    math::Point,
};

impl Shape for fj::Difference2d {
    fn bounding_volume(&self) -> AABB {
        // This is a conservative estimate of the bounding box: It's never going
        // to be bigger than the bounding box of the original shape that another
        // is being subtracted from.
        self.a.bounding_volume()
    }

    fn faces(&self, tolerance: f64, debug_info: &mut DebugInfo) -> Faces {
        // TASK: This method assumes that `b` is fully contained within `a`. As
        //       long as this precondition exists, it should at least be
        //       checked.

        let mut a = self.a.faces(tolerance, debug_info);
        let mut b = self.b.faces(tolerance, debug_info);

        let (a, b) = if a.0.len() == 1 && b.0.len() == 1 {
            // Can't panic. We just checked that length of `a` and `b` is 1.
            (a.0.pop().unwrap(), b.0.pop().unwrap())
        } else {
            // TASK: Open issue, link it in the error message.
            todo!(
                "The 2-dimensional difference operation only supports one face \
                in each operand."
            );
        };

        let (a, b) = match (a, b) {
            (Face::Face { edges: a }, Face::Face { edges: b }) => (a, b),
            _ => {
                // None of the 2D types still use the triangles representation.
                unreachable!()
            }
        };

        let mut edges = a;
        edges.cycles.extend(b.cycles);

        Faces(vec![Face::Face { edges }])
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
