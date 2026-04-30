use std::{collections::BTreeSet, fmt};

use crate::{
    math::Line,
    new::{
        topology::{HalfEdge, Handle, Solid, Topology},
        validation::{ValidationCheck, ValidationConfig},
    },
};

/// # A solid contains coincident half-edges that are no siblings
///
/// [`HalfEdge`]s are considered siblings, if they reference the same [`Edge`].
/// Coincident half-edges within the same shell must be siblings, for the solid
/// to be valid.
///
/// [`Edge`]: crate::new::topology::Edge
pub struct CoincidentNonSiblingHalfEdges {
    /// # The coincident, non-sibling half-edges
    pub half_edges: [Handle<HalfEdge>; 2],
}

impl ValidationCheck<Solid> for CoincidentNonSiblingHalfEdges {
    fn check<'r>(
        solid: &'r Solid,
        topology: &Topology,
        config: &ValidationConfig,
    ) -> impl Iterator<Item = Self> + 'r {
        let half_edges: Vec<_> = solid
            .boundary
            .iter()
            .flat_map(|&half_face| &topology.half_faces[half_face].boundary)
            .collect();

        let mut visited_pairs = BTreeSet::new();
        let mut coincident_non_siblings = Vec::new();

        // Doing this without a spatial data structure is very suboptimal, but
        // should do for the time being.
        for &&handle_a in &half_edges {
            'b: for &&handle_b in &half_edges {
                let mut handles = [handle_a, handle_b];
                handles.sort();

                if visited_pairs.contains(&handles) {
                    continue;
                } else {
                    visited_pairs.insert(handles);
                }

                let [a, b] = [handle_a, handle_b]
                    .map(|half_edge| &topology.half_edges[half_edge]);

                if a == b {
                    // No need to check a half-edge against itself.
                    //
                    // This comparison is actually redundant, as it's also
                    // covered by the next one, but let's be clear and explicit
                    // about what's happening.
                    continue;
                }
                if a.edge == b.edge {
                    // Half-edges are siblings. This validation check does not
                    // apply.
                    continue;
                }

                // Again, it's very suboptimal to do this O(n^2) comparison of
                // everything against everything, but it should do for now.
                for a in a.boundary_and_approx(topology) {
                    for &[b1, b2] in
                        b.boundary_and_approx(topology).array_windows()
                    {
                        let (line, [t_min, t_max]) =
                            Line::from_points([b1, b2]);
                        let t =
                            line.point_to_line_coords(a).clamp(t_min, t_max);

                        let distance =
                            (a - line.point_from_line_coords(t)).magnitude();

                        if distance > config.non_coincident_distance {
                            // We have found one point that shows this pair of
                            // half-edges is not coincident. Move on to another
                            // pair.
                            continue 'b;
                        }
                    }
                }

                // If we mad it here, we found no points where the half-edges
                // are far enough apart to not be considered coincident.
                coincident_non_siblings.push([handle_a, handle_b]);
            }
        }

        coincident_non_siblings
            .into_iter()
            .map(|half_edges| Self { half_edges })
    }
}

impl fmt::Display for CoincidentNonSiblingHalfEdges {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "Coincident half-edges are not siblings: {:?}",
            self.half_edges,
        )?;
        Ok(())
    }
}
