use std::{collections::HashSet, fmt};

use fj_math::Point;

use crate::{
    geometry::{Curve, Surface},
    shape::Handle,
    topology::{Cycle, Edge, Vertex},
};

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
