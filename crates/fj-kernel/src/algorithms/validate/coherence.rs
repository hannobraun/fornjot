use std::fmt;

use fj_math::{Point, Scalar};

use crate::objects::Vertex;

pub fn validate_vertex(
    vertex: &Vertex,
    max_distance: impl Into<Scalar>,
) -> Result<(), CoherenceIssues> {
    let max_distance = max_distance.into();

    // Validate that the local and global forms of the vertex match. As a side
    // effect, this also happens to validate that the global form of the vertex
    // lies on the curve.

    let mut edge_vertex_mismatches = Vec::new();

    let local = vertex.position();
    let local_as_global = vertex
        .curve()
        .global()
        .kind()
        .point_from_curve_coords(local);
    let global = vertex.global().position();
    let distance = (local_as_global - global).magnitude();

    if distance > max_distance {
        edge_vertex_mismatches.push(CoherenceMismatch {
            local,
            local_as_global,
            global,
        });
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
    pub edge_vertex_mismatches: Vec<CoherenceMismatch>,
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
#[derive(Debug, Default, thiserror::Error)]
pub struct CoherenceMismatch {
    /// The local form of the object
    pub local: Point<1>,

    /// The local form of the object, converted into the global form
    pub local_as_global: Point<3>,

    /// The global form of the object
    pub global: Point<3>,
}

impl fmt::Display for CoherenceMismatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "local: {:?} (converted to global: {:?}), global: {:?},",
            self.local, self.local_as_global, self.global,
        )
    }
}
