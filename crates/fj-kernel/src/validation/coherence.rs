use std::fmt;

use fj_math::{Point, Scalar};

use crate::objects::Edge;

pub fn validate_edge(
    edge: &Edge<3>,
    max_distance: impl Into<Scalar>,
) -> Result<(), CoherenceIssues> {
    let max_distance = max_distance.into();

    // Validate that the local and canonical forms of the vertices match. As a
    // side effect, this also happens to validate that the canonical forms of
    // the vertices lie on the curve.

    let mut edge_vertex_mismatches = Vec::new();

    for vertex in edge.vertices.iter() {
        let local = vertex.position();
        let local_as_global = edge.curve().point_from_curve_coords(local);
        let global = vertex.global().position();
        let distance = (local_as_global - global).magnitude();

        if distance > max_distance {
            edge_vertex_mismatches.push(CoherenceMismatch {
                local,
                local_as_global,
                global,
            });
        }
    }

    if !edge_vertex_mismatches.is_empty() {
        return Err(CoherenceIssues {
            edge_vertex_mismatches,
        });
    }

    Ok(())
}

/// Geometric issues found during validation
///
/// Used by [`ValidationError`].
///
/// [`ValidationError`]: super::ValidationError
#[derive(Debug, Default, thiserror::Error)]
pub struct CoherenceIssues {
    /// Mismatches between the local and global forms of edge vertices
    pub edge_vertex_mismatches: Vec<CoherenceMismatch<Point<1>, Point<3>>>,
}

impl fmt::Display for CoherenceIssues {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Geometric issues found:")?;

        if !self.edge_vertex_mismatches.is_empty() {
            writeln!(f, "- Edge vertex mismatches:")?;

            for mismatch in &self.edge_vertex_mismatches {
                writeln!(f, "  - {}", mismatch)?;
            }
        }

        Ok(())
    }
}

/// A mismatch between the local and global forms of an object
///
/// Used in [`CoherenceIssues`].
#[derive(Debug)]
pub struct CoherenceMismatch<Local, Global> {
    /// The local form of the object
    pub local: Local,

    /// The local form of the object, converted into the global form
    pub local_as_global: Global,

    /// The global form of the object
    pub global: Global,
}

impl<Local, Canonical> fmt::Display for CoherenceMismatch<Local, Canonical>
where
    Local: fmt::Debug,
    Canonical: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "local: {:?} (converted to global: {:?}), global: {:?},",
            self.local, self.local_as_global, self.global,
        )
    }
}
