use std::{collections::HashSet, fmt};

use fj_math::{Point, Scalar};

use crate::{
    geometry::{Curve, Surface},
    topology::{Cycle, Edge, Face, Vertex},
};

use super::{stores::Stores, Handle, Object};

pub trait Validate {
    fn validate(
        &self,
        handle: Option<&Handle<Self>>,
        min_distance: Scalar,
        stores: &Stores,
    ) -> Result<(), ValidationError>
    where
        Self: Object;
}

impl Validate for Point<3> {
    fn validate(
        &self,
        _: Option<&Handle<Self>>,
        _: Scalar,
        _: &Stores,
    ) -> Result<(), ValidationError> {
        Ok(())
    }
}

impl Validate for Curve<3> {
    fn validate(
        &self,
        _: Option<&Handle<Self>>,
        _: Scalar,
        _: &Stores,
    ) -> Result<(), ValidationError> {
        Ok(())
    }
}

impl Validate for Surface {
    fn validate(
        &self,
        _: Option<&Handle<Self>>,
        _: Scalar,
        _: &Stores,
    ) -> Result<(), ValidationError> {
        Ok(())
    }
}

impl Validate for Vertex<3> {
    /// Validate the vertex
    ///
    /// # Implementation note
    ///
    /// In the future, this method is likely to validate more than it already
    /// does. See documentation of [`crate::kernel`] for some context on that.
    fn validate(
        &self,
        handle: Option<&Handle<Self>>,
        min_distance: Scalar,
        stores: &Stores,
    ) -> Result<(), ValidationError> {
        let point = self.point.canonical();

        if !stores.points.contains(&point) {
            return Err(StructuralIssues {
                missing_point: Some(point),
                ..StructuralIssues::default()
            }
            .into());
        }
        for existing in stores.vertices.iter() {
            if Some(&existing) == handle {
                continue;
            }

            let distance = (existing.get().point() - self.point()).magnitude();

            if distance < min_distance {
                return Err(ValidationError::Uniqueness);
            }
        }

        Ok(())
    }
}

impl Validate for Edge<3> {
    fn validate(
        &self,
        _: Option<&Handle<Self>>,
        _: Scalar,
        stores: &Stores,
    ) -> Result<(), ValidationError> {
        let mut missing_curve = None;
        let mut missing_vertices = HashSet::new();

        if !stores.curves.contains(&self.curve.canonical()) {
            missing_curve = Some(self.curve.canonical());
        }
        for vertices in &self.vertices {
            for vertex in vertices {
                if !stores.vertices.contains(&vertex.canonical()) {
                    missing_vertices.insert(vertex.canonical().clone());
                }
            }
        }

        if missing_curve.is_some() || !missing_vertices.is_empty() {
            return Err(StructuralIssues {
                missing_curve,
                missing_vertices,
                ..StructuralIssues::default()
            }
            .into());
        }

        Ok(())
    }
}

impl Validate for Cycle<3> {
    /// Validate the cycle
    ///
    /// # Implementation note
    ///
    /// The validation of the cycle should be extended to cover more cases:
    /// - That those edges form a cycle.
    /// - That the cycle is not self-overlapping.
    /// - That there exists no duplicate cycle, with the same edges.
    fn validate(
        &self,
        _: Option<&Handle<Self>>,
        _: Scalar,
        stores: &Stores,
    ) -> Result<(), ValidationError> {
        let mut missing_edges = HashSet::new();
        for edge in &self.edges {
            let edge = edge.canonical();

            if !stores.edges.contains(&edge) {
                missing_edges.insert(edge.clone());
            }
        }

        if !missing_edges.is_empty() {
            return Err(StructuralIssues {
                missing_edges,
                ..StructuralIssues::default()
            }
            .into());
        }

        Ok(())
    }
}

impl Validate for Face {
    fn validate(
        &self,
        _: Option<&Handle<Self>>,
        _: Scalar,
        stores: &Stores,
    ) -> Result<(), ValidationError> {
        if let Face::Face(face) = self {
            let mut missing_surface = None;
            let mut missing_cycles = HashSet::new();

            if !stores.surfaces.contains(&face.surface) {
                missing_surface = Some(face.surface.clone());
            }
            for cycle in
                face.exteriors.as_handle().chain(face.interiors.as_handle())
            {
                if !stores.cycles.contains(&cycle) {
                    missing_cycles.insert(cycle);
                }
            }

            if missing_surface.is_some() || !missing_cycles.is_empty() {
                return Err(StructuralIssues {
                    missing_surface,
                    missing_cycles,
                    ..StructuralIssues::default()
                }
                .into());
            }
        }

        Ok(())
    }
}

/// Returned by the various `add_` methods of the [`Shape`] API
pub type ValidationResult<T> = Result<Handle<T>, ValidationError>;

/// An error that can occur during a validation
#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    /// Structural validation failed
    ///
    /// Structural validation verifies, that all the object that an object
    /// refers to are already part of the shape.
    #[error("Structural validation failed")]
    Structural(#[from] StructuralIssues),

    /// Uniqueness validation failed
    ///
    /// Uniqueness validation checks, that an object is unique. Uniqueness is
    /// only required for topological objects, as there's no harm in geometric
    /// objects being duplicated.
    #[error("Uniqueness validation failed")]
    Uniqueness,

    /// Geometric validation failed
    ///
    /// Geometric validation checks, that various geometric constraints of an
    /// object are upheld. For example, edges or faces might not be allowed to
    /// intersect.
    #[error("Geometric validation failed")]
    Geometric,
}

impl ValidationError {
    /// Indicate whether validation found a missing curve
    #[cfg(test)]
    pub fn missing_curve(&self, curve: &Handle<Curve<3>>) -> bool {
        if let Self::Structural(StructuralIssues { missing_curve, .. }) = self {
            return missing_curve.as_ref() == Some(curve);
        }

        false
    }

    /// Indicate whether validation found a missing vertex
    #[cfg(test)]
    pub fn missing_vertex(&self, vertex: &Handle<Vertex<3>>) -> bool {
        if let Self::Structural(StructuralIssues {
            missing_vertices, ..
        }) = self
        {
            return missing_vertices.contains(vertex);
        }

        false
    }

    /// Indicate whether validation found a missing edge
    #[cfg(test)]
    pub fn missing_edge(&self, edge: &Handle<Edge<3>>) -> bool {
        if let Self::Structural(StructuralIssues { missing_edges, .. }) = self {
            return missing_edges.contains(edge);
        }

        false
    }

    /// Indicate whether validation found a missing surface
    #[cfg(test)]
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
    #[cfg(test)]
    pub fn missing_cycle(&self, cycle: &Handle<Cycle<3>>) -> bool {
        if let Self::Structural(StructuralIssues { missing_cycles, .. }) = self
        {
            return missing_cycles.contains(cycle);
        }

        false
    }
}

/// Structural issues found during validation
///
/// Used by [`ValidationError`].
#[derive(Debug, Default, thiserror::Error)]
pub struct StructuralIssues {
    /// Missing point found in vertex validation
    pub missing_point: Option<Handle<Point<3>>>,

    /// Missing curve found in edge validation
    pub missing_curve: Option<Handle<Curve<3>>>,

    /// Missing vertices found in edge validation
    pub missing_vertices: HashSet<Handle<Vertex<3>>>,

    /// Missing edges found in cycle validation
    pub missing_edges: HashSet<Handle<Edge<3>>>,

    /// Missing surface found in face validation
    pub missing_surface: Option<Handle<Surface>>,

    /// Missing cycles found in face validation
    pub missing_cycles: HashSet<Handle<Cycle<3>>>,
}

impl fmt::Display for StructuralIssues {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Structural issues found:")?;

        if let Some(point) = &self.missing_point {
            writeln!(f, "- Missing point: {:?}", point.get())?;
        }
        if let Some(curve) = &self.missing_curve {
            writeln!(f, "- Missing curve: {:?}", curve.get())?;
        }
        if !self.missing_vertices.is_empty() {
            writeln!(f, "- Missing vertices:")?;

            for vertex in &self.missing_vertices {
                writeln!(f, "  - {:?}", vertex.get())?;
            }
        }
        if !self.missing_edges.is_empty() {
            writeln!(f, "- Missing edges:")?;

            for edge in &self.missing_edges {
                writeln!(f, "  - {}", edge.get())?;
            }
        }
        if let Some(surface) = &self.missing_surface {
            writeln!(f, "- Missing surface: {:?}", surface.get())?;
        }
        if !self.missing_cycles.is_empty() {
            writeln!(f, "- Missing cycles:")?;

            for cycle in &self.missing_cycles {
                writeln!(f, "  - {:?}", cycle.get())?;
            }
        }

        Ok(())
    }
}
