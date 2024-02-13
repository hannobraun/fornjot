use std::{error::Error as _, fmt, mem};

use fj_core::{
    algorithms::{
        approx::{InvalidTolerance, Tolerance},
        bounding_volume::BoundingVolume,
        triangulate::Triangulate,
    },
    validate::ValidationErrors,
    Instance,
};
use fj_interop::Model;
use fj_math::{Aabb, Point, Scalar};
use tracing_subscriber::prelude::*;

use crate::Args;

/// Export or display a model, according to CLI arguments
///
/// This function is intended to be called by applications that define a model
/// and want to provide a standardized CLI interface for dealing with that
/// model.
///
/// This function is used by Fornjot's own testing infrastructure, but is useful
/// beyond that, when using Fornjot directly to define a model.
pub fn handle_model<M>(model: &M, core: Instance) -> Result
where
    for<'r> (&'r M, Tolerance): Triangulate,
    M: BoundingVolume<3>,
{
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args = Args::parse();

    if args.ignore_validation {
        mem::forget(core);
    } else {
        core.services.drop_and_validate()?;
    }

    let aabb = model.aabb().unwrap_or(Aabb {
        min: Point::origin(),
        max: Point::origin(),
    });

    let tolerance = match args.tolerance {
        None => {
            // Compute a reasonable default for the tolerance value. To do
            // this, we just look at the smallest non-zero extent of the
            // bounding box and divide that by some value.

            let mut min_extent = Scalar::MAX;
            for extent in aabb.size().components {
                if extent > Scalar::ZERO && extent < min_extent {
                    min_extent = extent;
                }
            }

            let tolerance = min_extent / Scalar::from_f64(1000.);
            Tolerance::from_scalar(tolerance)?
        }
        Some(user_defined_tolerance) => user_defined_tolerance,
    };

    let mesh = (model, tolerance).triangulate();

    if let Some(path) = args.export {
        crate::export::export(&mesh, &path)?;
        return Ok(());
    }

    let model = Model { mesh, aabb };

    crate::window::display(model, false)?;

    Ok(())
}

/// Return value of [`handle_model`]
pub type Result = std::result::Result<(), Error>;

/// Error returned by [`handle_model`]
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
