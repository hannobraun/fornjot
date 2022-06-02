use std::fmt;

use fj_math::{Point, Scalar};

use crate::{
    shape::{stores::Store, Handle},
    topology::Vertex,
};

pub fn validate_point(
    point: Point<3>,
    handle: Option<&Handle<Point<3>>>,
    min_distance: Scalar,
    points: &Store<Point<3>>,
) -> Result<(), UniquenessIssues> {
    for existing in points.iter() {
        if Some(&existing) == handle {
            continue;
        }

        let distance = (existing.get() - point).magnitude();
        if distance < min_distance {
            return Err(UniquenessIssues {
                duplicate_point: Some(DuplicatePoint {
                    existing,
                    new: point,
                    distance,
                }),
            });
        }
    }

    Ok(())
}

pub fn validate_vertex(
    _vertex: &Vertex,
    _handle: Option<&Handle<Vertex>>,
    _min_distance: Scalar,
    _vertices: &Store<Vertex>,
) -> Result<(), UniquenessIssues> {
    // This function is a placeholder. It has been replaced by `validate_point`
    // temporarily, but will soon be extended again, to make sure that vertices
    // don't share the same points.

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
    /// Duplicate point found
    pub duplicate_point: Option<DuplicatePoint>,
}

impl fmt::Display for UniquenessIssues {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Uniqueness issues found:")?;

        if let Some(duplicate_point) = &self.duplicate_point {
            writeln!(f, "- Duplicate point ({})", duplicate_point)?;
        }

        Ok(())
    }
}

/// A duplicate point
///
/// Used in [`UniquenessIssues`].
#[derive(Debug)]
pub struct DuplicatePoint {
    /// The existing point
    pub existing: Handle<Point<3>>,

    /// The new point
    pub new: Point<3>,

    /// The distance between the vertices
    pub distance: Scalar,
}

impl fmt::Display for DuplicatePoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "existing: {:?}, new: {:?}, distance: {}",
            self.existing, self.new, self.distance
        )
    }
}
