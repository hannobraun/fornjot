use std::fmt;

use fj_math::Point;

use crate::{
    geometry::CurveBoundary,
    storage::Handle,
    topology::{Curve, HalfEdge, Vertex},
};

/// A [`Shell`] contains two [`HalfEdge`]s that are coincident but not siblings
///
/// Coincident half-edges must reference the same curve, and have the same
/// boundaries on that curve. This provides clear, topological information,
/// which is important to handle the shell geometry in a robust way.
///
/// [`Shell`]: crate::topology::Shell
#[derive(Clone, Debug, thiserror::Error)]
pub struct CoincidentHalfEdgesAreNotSiblings {
    /// The boundaries of the half-edges
    pub boundaries: [CurveBoundary<Point<1>>; 2],

    /// The curves of the half-edges
    pub curves: [Handle<Curve>; 2],

    /// The vertices of the half-edges
    pub vertices: [CurveBoundary<Vertex>; 2],

    /// The first half-edge
    pub half_edge_a: Handle<HalfEdge>,

    /// The second half-edge
    pub half_edge_b: Handle<HalfEdge>,
}

impl fmt::Display for CoincidentHalfEdgesAreNotSiblings {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "`Shell` contains `HalfEdge`s that are coincident but are not \
            siblings",
        )?;

        {
            let [a, b] = &self.boundaries;

            if a != &b.reverse() {
                writeln!(
                    f,
                    "Boundaries don't match.\n\
                    \tHalf-edge 1 has boundary `{a:?}`\n\
                    \tHalf-edge 2 has boundary `{b:?}`\n\
                    \t(expecting same boundary, but reversed)"
                )?;
            }
        }

        {
            let [a, b] = &self.curves;

            if a.id() != b.id() {
                writeln!(
                    f,
                    "Curves don't match.\n\
                    \tHalf-edge 1 lies on {a:?}\n\
                    \tHalf-edge 2 lies on {b:?}\n\
                    \t(must be the same)"
                )?;
            }
        }

        {
            let [a, b] = &self.vertices;

            if a != &b.clone().reverse() {
                writeln!(
                    f,
                    "Vertices don't match.\n\
                    \tHalf-edge 1 is bounded by `{a:?}`\n\
                    \tHalf-edge 2 is bounded by `{b:?}`\n\
                    \t(expecting same vertices, but in reverse order)"
                )?;
            }
        }

        write!(
            f,
            "Half-edge 1: {:#?}\n\
            Half-edge 2: {:#?}",
            self.half_edge_a, self.half_edge_b,
        )?;

        Ok(())
    }
}
