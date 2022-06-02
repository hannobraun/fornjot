use std::fmt;

use fj_math::Scalar;

use crate::{
    shape::{stores::Store, Handle},
    topology::Vertex,
};

pub fn validate_vertex(
    vertex: &Vertex,
    handle: Option<&Handle<Vertex>>,
    min_distance: Scalar,
    vertices: &Store<Vertex>,
) -> Result<(), UniquenessIssues> {
    for existing in vertices.iter() {
        if Some(&existing) == handle {
            continue;
        }

        let distance = (existing.get().point() - vertex.point()).magnitude();
        if distance < min_distance {
            return Err(UniquenessIssues {
                duplicate_vertex: Some(DuplicateVertex {
                    existing,
                    new: vertex.clone(),
                    distance,
                }),
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
    pub duplicate_vertex: Option<DuplicateVertex>,
}

impl fmt::Display for UniquenessIssues {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Uniqueness issues found:")?;

        if let Some(duplicate_vertex) = &self.duplicate_vertex {
            writeln!(f, "- Duplicate vertex ({})", duplicate_vertex)?;
        }

        Ok(())
    }
}

/// A duplicate vertex
///
/// Used in [`UniquenessIssues`].
#[derive(Debug)]
pub struct DuplicateVertex {
    /// The existing vertex
    pub existing: Handle<Vertex>,

    /// The new vertex
    pub new: Vertex,

    /// The distance between the vertices
    pub distance: Scalar,
}

impl fmt::Display for DuplicateVertex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "existing: {:?}, new: {:?}, distance: {}",
            self.existing, self.new, self.distance
        )
    }
}
