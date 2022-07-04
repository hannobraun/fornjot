//! Infrastructure for validating shapes
//!
//! Validation enforces various constraints about shapes and the objects that
//! constitute them. These constraints fall into 4 categories:
//!
//! - **Coherence:** Local forms of objects must be consistent with their
//!   canonical forms.
//! - **Geometric:** Comprises various object-specific constraints, for example
//!   edges or faces might not be allowed to intersect.
//! - **Structural:** All other objects that an object references must be part
//!   of the same shape.
//! - **Uniqueness:** Objects within a shape must be unique.
//!
//! Please note that not all of these validation categories are fully
//! implemented, as of this writing.
//!
//! # Implementation Note
//!
//! There is an ongoing effort to abolish [`Shape`] and replace it with a much
//! simpler data structure:
//! https://github.com/hannobraun/Fornjot/issues/697
//!
//! Once completed, this would make structural validation moot, and reduce the
//! scope of uniqueness validation.
//!
//! [`Shape`]: crate::shape::Shape

mod coherence;
mod uniqueness;

pub use self::{
    coherence::{CoherenceIssues, CoherenceMismatch},
    uniqueness::UniquenessIssues,
};

use std::{collections::HashSet, ops::Deref};

use fj_math::Scalar;

use crate::iter::ObjectIters;

/// Validate the given [`Shape`]
pub fn validate<T>(
    object: T,
    config: &ValidationConfig,
) -> Result<Validated<T>, ValidationError>
where
    T: ObjectIters,
{
    let mut vertices = HashSet::new();

    for vertex in object.global_vertex_iter() {
        uniqueness::validate_vertex(
            &vertex,
            &vertices,
            config.distinct_min_distance,
        )?;

        vertices.insert(vertex);
    }

    for edge in object.edge_iter() {
        coherence::validate_edge(&edge, config.identical_max_distance)?;
    }

    Ok(Validated(object))
}

/// Configuration required for the validation process
#[derive(Debug, Clone, Copy)]
pub struct ValidationConfig {
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

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            distinct_min_distance: Scalar::from_f64(5e-7), // 0.5 µm,

            // This value was chosen pretty arbitrarily. Seems small enough to
            // catch errors. If it turns out it's too small (because it produces
            // false positives due to floating-point accuracy issues), we can
            // adjust it.
            identical_max_distance: Scalar::from_f64(5e-14),
        }
    }
}

/// Wrapper around an object that indicates the object has been validated
///
/// Returned by implementations of `Validate`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Validated<T>(T);

impl<T> Validated<T> {
    /// Consume this instance of `Validated` and return the wrapped object
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> Deref for Validated<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// An error that can occur during a validation
#[allow(clippy::large_enum_variant)]
#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    /// Coherence validation failed
    #[error("Coherence validation failed")]
    Coherence(#[from] CoherenceIssues),

    /// Geometric validation failed
    #[error("Geometric validation failed")]
    Geometric,

    /// Uniqueness validation failed
    #[error("Uniqueness validation failed")]
    Uniqueness(#[from] UniquenessIssues),
}

#[cfg(test)]
mod tests {
    use fj_math::{Point, Scalar};

    use crate::{
        local::Local,
        objects::{Curve, Edge, GlobalVertex, Vertex, VerticesOfEdge},
        validation::{validate, ValidationConfig, ValidationError},
    };

    #[test]
    fn coherence_edge() {
        let a = Point::from([0., 0., 0.]);
        let b = Point::from([1., 0., 0.]);

        let curve = {
            let curve_local = Curve::line_from_points([[0., 0.], [1., 0.]]);
            let curve_canonical = Curve::line_from_points([a, b]);
            Local::new(curve_local, curve_canonical)
        };

        let a = GlobalVertex::from_position(a);
        let b = GlobalVertex::from_position(b);

        let deviation = Scalar::from_f64(0.25);

        let a = Vertex::new(Point::from([Scalar::ZERO + deviation]), a);
        let b = Vertex::new(Point::from([Scalar::ONE]), b);
        let vertices = VerticesOfEdge::from_vertices([a, b]);

        let edge = Edge { curve, vertices };

        let result = validate(
            edge.clone(),
            &ValidationConfig {
                identical_max_distance: deviation * 2.,
                ..ValidationConfig::default()
            },
        );
        assert!(result.is_ok());

        let result = validate(
            edge,
            &ValidationConfig {
                identical_max_distance: deviation / 2.,
                ..ValidationConfig::default()
            },
        );
        assert!(result.is_err());
    }

    #[test]
    fn uniqueness_vertex() -> anyhow::Result<()> {
        let mut shape = Vec::new();

        let deviation = Scalar::from_f64(0.25);

        let a = Point::from([0., 0., 0.]);

        let mut b = a;
        b.x += deviation;

        let config = ValidationConfig {
            distinct_min_distance: deviation * 2.,
            ..ValidationConfig::default()
        };

        // Adding a vertex should work.
        shape.push(GlobalVertex::from_position(a));
        validate(shape.clone(), &config)?;

        // Adding a second vertex that is considered identical should fail.
        shape.push(GlobalVertex::from_position(b));
        let result = validate(shape, &config);
        assert!(matches!(result, Err(ValidationError::Uniqueness(_))));

        Ok(())
    }
}
