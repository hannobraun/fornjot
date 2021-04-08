use parry2d::shape::Triangle;

use super::PolygonInner;

pub struct Triangles<'r>(pub(super) &'r mut PolygonInner);

impl Triangles<'_> {
    pub fn remove(
        &mut self,
        triangle: Triangle,
    ) -> Result<(), TriangleNotPresent> {
        // TASK: Convert to update `self.edges` in addition to `self.chains`.
        //       Once this is done correctly, the other methods can be updated
        //       to use `self.edges`, and `self.chains` can be removed.

        // ---

        // Create a structure that gives us each point of the triangle together
        // with the two other points.
        let triangle = [
            (triangle.a, [triangle.b, triangle.c]),
            (triangle.b, [triangle.a, triangle.c]),
            (triangle.c, [triangle.a, triangle.b]),
        ];

        for chain in &mut self.0.chains {
            // Need to query a copy of the chain, else our removals will falsify
            // further queries.
            let chain_copy = chain.clone();

            for &(vertex, [a, b]) in &triangle {
                if let Some(neighbors) = chain_copy.neighbors_of(vertex) {
                    if neighbors.contains(a) && neighbors.contains(b) {
                        chain.remove(vertex);
                    }
                }
            }

            if chain.len() < chain_copy.len() {
                // We removed vertices from the chain.
                //
                // Due to the assumptions made by `Polygon` (no edges that
                // overlap, and no vertices shared between chains), we can
                // assume that we're done and will find nothing more.
                return Ok(());
            }
        }

        Err(TriangleNotPresent)
    }
}

#[derive(Debug)]
pub struct TriangleNotPresent;
