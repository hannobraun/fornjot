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

mod coherence;
mod uniqueness;

pub use self::{
    coherence::{CoherenceIssues, VertexCoherenceMismatch},
    uniqueness::UniquenessIssues,
};

use std::{collections::HashSet, ops::Deref};

use fj_math::Scalar;

use crate::iter::ObjectIters;

/// Validate an object
pub trait Validate: Sized {
    /// Validate the object using default configuration
    ///
    /// The following calls are equivalent:
    /// ``` rust
    /// # use fj_kernel::{
    /// #     algorithms::validate::{Validate, ValidationConfig},
    /// #     objects::GlobalVertex,
    /// # };
    /// # let object = GlobalVertex::from_position([0., 0., 0.]);
    /// object.validate();
    /// object.validate_with_config(&ValidationConfig::default());
    /// ```
    fn validate(self) -> Result<Validated<Self>, ValidationError> {
        self.validate_with_config(&ValidationConfig::default())
    }

    /// Validate the object
    fn validate_with_config(
        self,
        config: &ValidationConfig,
    ) -> Result<Validated<Self>, ValidationError>;
}

impl<T> Validate for T
where
    T: for<'r> ObjectIters<'r>,
{
    fn validate_with_config(
        self,
        config: &ValidationConfig,
    ) -> Result<Validated<Self>, ValidationError> {
        let mut global_vertices = HashSet::new();

        for curve in self.curve_iter() {
            coherence::validate_curve(curve, config.identical_max_distance)?;
        }

        for global_vertex in self.global_vertex_iter() {
            uniqueness::validate_vertex(
                global_vertex,
                &global_vertices,
                config.distinct_min_distance,
            )?;

            global_vertices.insert(*global_vertex);
        }
        for vertex in self.vertex_iter() {
            coherence::validate_vertex(vertex, config.identical_max_distance)?;
        }

        Ok(Validated(self))
    }
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
    use fj_math::{Line, Point, Scalar};

    use crate::{
        algorithms::validate::{Validate, ValidationConfig, ValidationError},
        objects::{
            Curve, CurveKind, Edge, GlobalCurve, GlobalVertex, Surface,
            SurfaceVertex, Vertex, VerticesOfEdge,
        },
    };

    #[test]
    fn coherence_curve() {
        let line_global = Line::from_points([[0., 0., 0.], [1., 0., 0.]]);
        let global_curve = GlobalCurve::from_kind(CurveKind::Line(line_global));

        let line_surface = Line::from_points([[0., 0.], [2., 0.]]);
        let curve = Curve::new(
            Surface::xy_plane(),
            CurveKind::Line(line_surface),
            global_curve,
        );

        let result = curve.validate();
        assert!(result.is_err());
    }

    #[test]
    fn coherence_edge() {
        let surface = Surface::xy_plane();

        let points_surface = [[0., 0.], [1., 0.]];
        let points_global = [[0., 0., 0.], [1., 0., 0.]];

        let curve = {
            let curve_local = CurveKind::line_from_points(points_surface);
            let curve_global = GlobalCurve::from_kind(
                CurveKind::line_from_points(points_global),
            );
            Curve::new(surface, curve_local, curve_global)
        };

        let [a_global, b_global] =
            points_global.map(GlobalVertex::from_position);

        let [a_surface, b_surface] = {
            // Can be cleaned up, once `zip` is stable:
            // https://doc.rust-lang.org/std/primitive.array.html#method.zip
            let [a_surface, b_surface] = points_surface;
            [(a_surface, a_global), (b_surface, b_global)].map(
                |(point_surface, vertex_global)| {
                    SurfaceVertex::new(point_surface, surface, vertex_global)
                },
            )
        };

        let deviation = Scalar::from_f64(0.25);

        let a = Vertex::new(
            Point::from([Scalar::ZERO + deviation]),
            curve,
            a_surface,
            a_global,
        );
        let b =
            Vertex::new(Point::from([Scalar::ONE]), curve, b_surface, b_global);
        let vertices = VerticesOfEdge::from_vertices([a, b]);

        let edge = Edge::from_curve_and_vertices(curve, vertices);

        let result = edge.validate_with_config(&ValidationConfig {
            identical_max_distance: deviation * 2.,
            ..ValidationConfig::default()
        });
        assert!(result.is_ok());

        let result = edge.validate_with_config(&ValidationConfig {
            identical_max_distance: deviation / 2.,
            ..ValidationConfig::default()
        });
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
        shape.clone().validate_with_config(&config)?;

        // Adding a second vertex that is considered identical should fail.
        shape.push(GlobalVertex::from_position(b));
        let result = shape.validate_with_config(&config);
        assert!(matches!(result, Err(ValidationError::Uniqueness(_))));

        Ok(())
    }
}
