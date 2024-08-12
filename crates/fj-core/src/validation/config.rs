use fj_math::Scalar;

use crate::algorithms::approx::Tolerance;

/// Configuration required for the validation process
#[derive(Debug, Clone, Copy)]
pub struct ValidationConfig {
    /// Panic on first validation error, instead of storing it
    ///
    /// Validation errors are usually stored in the validation layer, and only
    /// cause a panic if the validation layer is dropped with unhandled errors.
    ///
    /// This provides flexibility in handling validation errors, but can also be
    /// helpful in understanding them, as experience has shown that the first
    /// validation error often does not provide a full picture of what's wrong.
    ///
    /// However, it can be helpful to get an immediate panic on a validation
    /// error, to get the code that caused it into a stack trace. This is what
    /// happens, if this option is set to `true`.
    ///
    /// Defaults to `false`.
    pub panic_on_error: bool,

    /// The tolerance value used for intermediate geometry representation
    pub tolerance: Tolerance,

    /// The minimum distance between distinct objects
    ///
    /// Objects whose distance is less than the value defined in this field, are
    /// considered identical.
    pub distinct_min_distance: Scalar,

    /// The maximum distance between identical objects
    ///
    /// Objects that are considered identical might still have a distance
    /// between them, due to inaccuracies of the numerical representation. If
    /// that distance is less than the one defined in this field, can not be
    /// considered identical.
    pub identical_max_distance: Scalar,
}

impl ValidationConfig {
    /// Compute validation config from a tolerance value
    pub fn from_tolerance(tolerance: impl Into<Tolerance>) -> Self {
        let tolerance = tolerance.into();

        // This value was chosen pretty arbitrarily. Seems small enough to catch
        // errors. If it turns out it's too small (because it produces false
        // positives due to floating-point accuracy issues), we can adjust it.
        let identical_max_distance = Scalar::from_f64(5e-14);

        Self {
            panic_on_error: false,
            tolerance,
            distinct_min_distance: Scalar::from_f64(5e-7), // 0.5 µm,
            identical_max_distance,
        }
    }
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self::from_tolerance(0.001)
    }
}
