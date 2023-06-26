use std::ops::Deref;

use fj_core::{
    algorithms::{
        approx::{InvalidTolerance, Tolerance},
        bounding_volume::BoundingVolume,
        triangulate::Triangulate,
    },
    services::Services,
};
use fj_interop::model::Model;
use fj_math::{Aabb, Point, Scalar};

use crate::Args;

/// Export or display a model, according to CLI arguments
///
/// This function is intended to be called by applications that define a model
/// and want to provide a standardized CLI interface for dealing with that
/// model.
///
/// This function is used by Fornjot's own testing infrastructure, but is useful
/// beyond that, when using Fornjot directly to define a model.
pub fn handle_model<M>(
    model: impl Deref<Target = M>,
    services: Services,
) -> Result
where
    for<'r> (&'r M, Tolerance): Triangulate,
    M: BoundingVolume<3>,
{
    let args = Args::parse();

    // Dropping `Services` will cause a panic, if there are any unhandled
    // validation errors. It would be better to return an error, but this will
    // do for now.
    drop(services);

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

    let mesh = (model.deref(), tolerance).triangulate();

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
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Error displaying model
    #[error("Error displaying model")]
    Display(#[from] crate::window::Error),

    /// Error exporting model
    #[error("Error exporting model")]
    Export(#[from] crate::export::Error),

    /// Invalid tolerance
    #[error(transparent)]
    Tolerance(#[from] InvalidTolerance),
}
