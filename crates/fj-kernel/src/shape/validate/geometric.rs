use std::fmt;

/// Geometric issues found during validation
///
/// Used by [`ValidationError`].
///
/// [`ValidationError`]: super::ValidationError
#[derive(Debug, Default, thiserror::Error)]
pub struct GeometricIssues;

impl fmt::Display for GeometricIssues {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Geometric issues found")?;
        Ok(())
    }
}
