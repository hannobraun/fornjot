use std::{error::Error as _, fmt};

use fj_core::{
    algorithms::approx::InvalidTolerance, validate::ValidationErrors,
};

/// Return value of [`Instance::process_model`]
///
/// [`Instance::process_model`]: crate::Instance::process_model
pub type Result = std::result::Result<(), Error>;

/// Error returned by [`Instance::process_model`]
///
/// [`Instance::process_model`]: crate::Instance::process_model
#[derive(thiserror::Error)]
pub enum Error {
    /// Failed to set up logger
    #[error("Failed to set up logger")]
    Tracing(#[from] tracing::subscriber::SetGlobalDefaultError),

    /// Error displaying model
    #[error("Error displaying model")]
    Display(#[from] crate::window::Error),

    /// Error exporting model
    #[error("Error exporting model")]
    Export(#[from] crate::export::Error),

    /// Invalid tolerance
    #[error(transparent)]
    Tolerance(#[from] InvalidTolerance),

    /// Unhandled validation errors
    #[error(transparent)]
    Validation(#[from] ValidationErrors),
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // When returning an error from Rust's `main` function, the runtime uses
        // the error's `Debug` implementation to display it, not the `Display`
        // one. This is unfortunate, and forces us to override `Debug` here.

        // We should be able to replace this with `Report`, once it is stable:
        // https://doc.rust-lang.org/std/error/struct.Report.html

        write!(f, "{self}")?;

        let mut source = self.source();

        if source.is_some() {
            write!(f, "\n\nCaused by:")?;
        }

        let mut i = 0;
        while let Some(s) = source {
            write!(f, "\n    {i}: {s}")?;
            source = s.source();
            i += 1;
        }

        Ok(())
    }
}
