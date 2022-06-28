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
mod structural;
mod uniqueness;

pub use self::{
    coherence::{CoherenceIssues, CoherenceMismatch},
    structural::StructuralIssues,
    uniqueness::UniquenessIssues,
};

use std::{collections::HashSet, ops::Deref};

use fj_math::Scalar;

use crate::{
    iter::ObjectIters,
    objects::{Curve, Cycle, Edge, Surface, Vertex},
};

/// Validate the given [`Shape`]
pub fn validate<T>(
    object: T,
    config: &ValidationConfig,
) -> Result<Validated<T>, ValidationError>
where
    T: ObjectIters,
{
    let mut curves = HashSet::new();
    let mut cycles = HashSet::new();
    let mut edges = HashSet::new();
    let mut surfaces = HashSet::new();
    let mut vertices = HashSet::new();

    for curve in object.curve_iter() {
        curves.insert(curve);
    }

    for vertex in object.vertex_iter() {
        uniqueness::validate_vertex(&vertex, &vertices)?;

        vertices.insert(vertex);
    }

    for edge in object.edge_iter() {
        coherence::validate_edge(&edge, config.identical_max_distance)?;
        structural::validate_edge(&edge, &curves, &vertices)?;

        edges.insert(edge);
    }

    for cycle in object.cycle_iter() {
        structural::validate_cycle(&cycle, &edges)?;

        cycles.insert(cycle);
    }

    for surface in object.surface_iter() {
        surfaces.insert(surface);
    }

    for face in object.face_iter() {
        structural::validate_face(&face, &cycles, &surfaces)?;
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

    /// Structural validation failed
    #[error("Structural validation failed")]
    Structural(#[from] StructuralIssues),

    /// Uniqueness validation failed
    #[error("Uniqueness validation failed")]
    Uniqueness(#[from] UniquenessIssues),
}

impl ValidationError {
    /// Indicate whether validation found a missing curve
    pub fn missing_curve(&self, curve: &Curve<3>) -> bool {
        if let Self::Structural(StructuralIssues { missing_curve, .. }) = self {
            return missing_curve.as_ref() == Some(curve);
        }

        false
    }

    /// Indicate whether validation found a missing vertex
    pub fn missing_vertex(&self, vertex: &Vertex) -> bool {
        if let Self::Structural(StructuralIssues {
            missing_vertices, ..
        }) = self
        {
            return missing_vertices.contains(vertex);
        }

        false
    }

    /// Indicate whether validation found a missing edge
    pub fn missing_edge(&self, edge: &Edge<3>) -> bool {
        if let Self::Structural(StructuralIssues { missing_edges, .. }) = self {
            return missing_edges.contains(edge);
        }

        false
    }

    /// Indicate whether validation found a missing surface
    pub fn missing_surface(&self, surface: &Surface) -> bool {
        if let Self::Structural(StructuralIssues {
            missing_surface, ..
        }) = self
        {
            return missing_surface.as_ref() == Some(surface);
        }

        false
    }

    /// Indicate whether validation found a missing cycle
    pub fn missing_cycle(&self, cycle: &Cycle<3>) -> bool {
        if let Self::Structural(StructuralIssues { missing_cycles, .. }) = self
        {
            return missing_cycles.contains(cycle);
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use fj_math::{Point, Scalar};

    use crate::{
        objects::{Curve, Cycle, Edge, Face, Surface, Vertex, VerticesOfEdge},
        shape::{LocalForm, Shape},
        validation::{validate, ValidationConfig, ValidationError},
    };

    #[test]
    fn coherence_edge() {
        let a = Point::from([0., 0., 0.]);
        let b = Point::from([1., 0., 0.]);

        let curve = {
            let curve = Curve::line_from_points([a, b]);
            LocalForm::canonical_only(curve)
        };

        let a = Vertex { point: a };
        let b = Vertex { point: b };

        let deviation = Scalar::from_f64(0.25);

        let a = LocalForm::new(Point::from([Scalar::ZERO + deviation]), a);
        let b = LocalForm::new(Point::from([Scalar::ONE]), b);
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
    fn structural_cycle() {
        let mut shape = Shape::new();

        // Trying to refer to edge that is not from the same shape. Should fail.
        let edge = Edge::line_segment_from_points([[0., 0., 0.], [1., 0., 0.]]);
        shape.insert(Cycle::new(vec![edge.clone()]));
        let err =
            validate(shape.clone(), &ValidationConfig::default()).unwrap_err();
        assert!(err.missing_edge(&edge));

        // Referring to edge that *is* from the same shape. Should work.
        let edge = Edge::line_segment_from_points([[0., 0., 0.], [1., 0., 0.]]);
        shape.insert(Cycle::new(vec![edge]));
    }

    #[test]
    fn structural_edge() {
        let mut shape = Shape::new();

        let curve = Curve::x_axis();
        let a = Vertex::from_point([1., 0., 0.]);
        let b = Vertex::from_point([2., 0., 0.]);

        let a = LocalForm::new(Point::from([1.]), a);
        let b = LocalForm::new(Point::from([2.]), b);

        // Shouldn't work. Nothing has been added to `shape`.
        shape.insert(Edge {
            curve: LocalForm::canonical_only(curve),
            vertices: VerticesOfEdge::from_vertices([a.clone(), b.clone()]),
        });
        let err =
            validate(shape.clone(), &ValidationConfig::default()).unwrap_err();
        assert!(err.missing_curve(&curve));
        assert!(err.missing_vertex(&a.canonical()));
        assert!(err.missing_vertex(&b.canonical()));

        let curve = Curve::x_axis();
        let a = Vertex::from_point([1., 0., 0.]);
        let b = Vertex::from_point([2., 0., 0.]);

        let a = LocalForm::new(Point::from([1.]), a);
        let b = LocalForm::new(Point::from([2.]), b);

        // Everything has been added to `shape` now. Should work!
        shape.insert(Edge {
            curve: LocalForm::canonical_only(curve),
            vertices: VerticesOfEdge::from_vertices([a, b]),
        });
    }

    #[test]
    fn structural_face() {
        let mut shape = Shape::new();

        let triangle = [[0., 0.], [1., 0.], [0., 1.]];

        let surface = Surface::xy_plane();
        let cycle = Cycle::builder(surface).build_polygon(triangle);

        // Nothing has been added to `shape`. Should fail.
        shape.insert(Face::new(
            surface,
            vec![cycle.clone()],
            Vec::new(),
            [255, 0, 0, 255],
        ));
        let err =
            validate(shape.clone(), &ValidationConfig::default()).unwrap_err();
        assert!(err.missing_surface(&surface));
        assert!(err.missing_cycle(&cycle.canonical()));

        let surface = Surface::xy_plane();
        let cycle = Cycle::builder(surface).build_polygon(triangle);

        // Everything has been added to `shape` now. Should work!
        shape.insert(Face::new(
            surface,
            vec![cycle],
            Vec::new(),
            [255, 0, 0, 255],
        ));
    }

    #[test]
    fn uniqueness_vertex() -> anyhow::Result<()> {
        let mut shape = Shape::new();

        let point = Point::from([0., 0., 0.]);

        // Adding a vertex should work.
        shape.insert(Vertex { point });
        validate(shape.clone(), &ValidationConfig::default())?;

        // Adding a second vertex with the same point should fail.
        shape.insert(Vertex { point });
        let result = validate(shape, &ValidationConfig::default());
        assert!(matches!(result, Err(ValidationError::Uniqueness(_))));

        Ok(())
    }
}
