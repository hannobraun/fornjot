use std::{collections::HashSet, fmt};

use fj_math::Scalar;

use crate::objects::GlobalVertex;

pub fn validate_vertex(
    vertex: &GlobalVertex,
    vertices: &HashSet<GlobalVertex>,
    min_distance: Scalar,
) -> Result<(), UniquenessIssues> {
    for existing in vertices {
        if (existing.position - vertex.position).magnitude() < min_distance {
            return Err(UniquenessIssues {
                duplicate_vertex: Some(*existing),
            });
        }
    }

    Ok(())
}

/// Uniqueness issues found during validation
///
/// Used by [`ValidationError`].
///
/// # Implementation Note
///
/// This struct doesn't carry any actual information, currently. Information
/// about the specific uniqueness issues found can be added as required. For
/// now, this struct exists to ease the error handling code.
///
/// [`ValidationError`]: super::ValidationError
#[derive(Debug, Default, thiserror::Error)]
pub struct UniquenessIssues {
    /// Duplicate vertex found
    pub duplicate_vertex: Option<GlobalVertex>,
}

impl fmt::Display for UniquenessIssues {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Uniqueness issues found:")?;

        if let Some(duplicate_vertex) = &self.duplicate_vertex {
            writeln!(f, "- Duplicate vertex ({:?}", duplicate_vertex)?;
        }

        Ok(())
    }
}
