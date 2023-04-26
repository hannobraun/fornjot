//! API for processing shapes

use fj_interop::{debug::DebugInfo, processed_shape::ProcessedShape};
use fj_kernel::{
    algorithms::{
        approx::{InvalidTolerance, Tolerance},
        triangulate::Triangulate,
    },
    services::Services,
    validate::ValidationError,
};
use fj_math::Scalar;

use crate::Shape as _;

/// Processes an [`fj::Shape`] into a [`ProcessedShape`]
pub struct ShapeProcessor {
    /// The tolerance value used for creating the triangle mesh
    pub tolerance: Option<Tolerance>,
}

impl ShapeProcessor {
    /// Process an [`fj::Shape`] into [`ProcessedShape`]
    pub fn process(&self, shape: &fj::Shape) -> Result<ProcessedShape, Error> {
        let aabb = shape.bounding_volume();

        let tolerance = match self.tolerance {
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

        let mut services = Services::new();
        let mut debug_info = DebugInfo::new();
        let shape = shape.compute_brep(&mut services, &mut debug_info);
        let mesh = (&shape, tolerance).triangulate();

        Ok(ProcessedShape {
            aabb,
            mesh,
            debug_info,
        })
    }
}

/// A shape processing error
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Error converting to shape
    #[error("Error converting to shape")]
    ToShape(#[from] Box<ValidationError>),

    /// Model has zero size
    #[error("Model has zero size")]
    Extent(#[from] InvalidTolerance),
}
