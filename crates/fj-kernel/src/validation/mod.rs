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
//! Once completed, this would make structural and uniqueness validation moot.
//!
//! [`Shape`]: crate::shape::Shape

mod coherence;
mod structural;

pub use self::{
    coherence::{CoherenceIssues, CoherenceMismatch},
    structural::StructuralIssues,
};

use std::{collections::HashSet, ops::Deref};

use fj_math::Scalar;

use crate::{
    objects::{Curve, Cycle, Edge, Surface, Vertex},
    shape::{Handle, Shape, UniquenessIssues},
};

/// Validate the given [`Shape`]
pub fn validate(
    shape: Shape,
    config: &ValidationConfig,
) -> Result<Validated<Shape>, ValidationError> {
    let mut curves = HashSet::new();
    let mut cycles = HashSet::new();
    let mut edges = HashSet::new();
    let mut surfaces = HashSet::new();
    let mut vertices = HashSet::new();

    for curve in shape.curves() {
        curves.insert(curve);
    }
    for vertex in shape.vertices() {
        vertices.insert(vertex);
    }
    for edge in shape.edges() {
        coherence::validate_edge(&edge.get(), config.identical_max_distance)?;
        structural::validate_edge(&edge.get(), &curves, &vertices)?;

        edges.insert(edge);
    }
    for cycle in shape.cycles() {
        structural::validate_cycle(&cycle.get(), &edges)?;

        cycles.insert(cycle);
    }
    for surface in shape.surfaces() {
        surfaces.insert(surface);
    }
    for face in shape.faces() {
        structural::validate_face(&face.get(), &cycles, &surfaces)?;
    }

    Ok(Validated(shape))
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

    /// Structural validation failed
    #[error("Structural validation failed")]
    Structural(#[from] StructuralIssues),

    /// Uniqueness validation failed
    #[error("Uniqueness validation failed")]
    Uniqueness(#[from] UniquenessIssues),
}

impl ValidationError {
    /// Indicate whether validation found a missing curve
    pub fn missing_curve(&self, curve: &Handle<Curve<3>>) -> bool {
        if let Self::Structural(StructuralIssues { missing_curve, .. }) = self {
            return missing_curve.as_ref() == Some(curve);
        }

        false
    }

    /// Indicate whether validation found a missing vertex
    pub fn missing_vertex(&self, vertex: &Handle<Vertex>) -> bool {
        if let Self::Structural(StructuralIssues {
            missing_vertices, ..
        }) = self
        {
            return missing_vertices.contains(vertex);
        }

        false
    }

    /// Indicate whether validation found a missing edge
    pub fn missing_edge(&self, edge: &Handle<Edge<3>>) -> bool {
        if let Self::Structural(StructuralIssues { missing_edges, .. }) = self {
            return missing_edges.contains(edge);
        }

        false
    }

    /// Indicate whether validation found a missing surface
    pub fn missing_surface(&self, surface: &Handle<Surface>) -> bool {
        if let Self::Structural(StructuralIssues {
            missing_surface, ..
        }) = self
        {
            return missing_surface.as_ref() == Some(surface);
        }

        false
    }

    /// Indicate whether validation found a missing cycle
    pub fn missing_cycle(&self, cycle: &Handle<Cycle<3>>) -> bool {
        if let Self::Structural(StructuralIssues { missing_cycles, .. }) = self
        {
            return missing_cycles.contains(cycle);
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use fj_math::Scalar;

    use crate::{
        objects::Edge,
        shape::{LocalForm, Shape},
        validation::ValidationConfig,
    };

    #[test]
    fn edge_coherence() -> anyhow::Result<()> {
        let mut shape = Shape::new();
        Edge::builder(&mut shape)
            .build_line_segment_from_points([[0., 0., 0.], [1., 0., 0.]])?
            .get();

        let deviation = Scalar::from_f64(0.25);

        shape
            .update()
            .update_all(|edge: &mut Edge<3>| {
                let original = edge.clone();
                *edge = Edge {
                    vertices: original.vertices.map(|vertex| {
                        LocalForm::new(
                            *vertex.local() + [deviation],
                            vertex.canonical(),
                        )
                    }),
                    ..original
                }
            })
            .validate()?;

        let result = super::validate(
            shape.clone(),
            &ValidationConfig {
                identical_max_distance: deviation * 2.,
                ..ValidationConfig::default()
            },
        );
        assert!(result.is_ok());

        let result = super::validate(
            shape,
            &ValidationConfig {
                identical_max_distance: deviation / 2.,
                ..ValidationConfig::default()
            },
        );
        assert!(result.is_err());

        Ok(())
    }
}
