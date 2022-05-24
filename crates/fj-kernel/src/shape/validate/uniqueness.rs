use std::fmt;

use fj_math::Scalar;

use crate::{
    shape::{stores::Stores, Handle},
    topology::Vertex,
};

pub fn validate_vertex(
    vertex: &Vertex,
    handle: Option<&Handle<Vertex>>,
    min_distance: Scalar,
    stores: &Stores,
) -> Result<(), UniquenessIssues> {
    for existing in stores.vertices.iter() {
        if Some(&existing) == handle {
            continue;
        }

        let distance = (existing.get().point() - vertex.point()).magnitude();
        if distance < min_distance {
            return Err(UniquenessIssues);
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
pub struct UniquenessIssues;

impl fmt::Display for UniquenessIssues {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Uniqueness issues found")?;
        Ok(())
    }
}
