use std::{collections::HashSet, fmt};

use crate::objects::{Curve, Cycle, Edge, Face, Surface, Vertex};

pub fn validate_edge(
    edge: &Edge<3>,
    curves: &HashSet<Curve<3>>,
    vertices: &HashSet<Vertex>,
) -> Result<(), StructuralIssues> {
    let mut missing_curve = None;
    let mut missing_vertices = HashSet::new();

    if !curves.contains(&edge.curve()) {
        missing_curve = Some(edge.curve.canonical().get());
    }
    for vertex in edge.vertices().into_iter().flatten() {
        if !vertices.contains(&vertex) {
            missing_vertices.insert(vertex);
        }
    }

    if missing_curve.is_some() || !missing_vertices.is_empty() {
        return Err(StructuralIssues {
            missing_curve,
            missing_vertices,
            ..StructuralIssues::default()
        });
    }

    Ok(())
}

pub fn validate_cycle(
    cycle: &Cycle<3>,
    edges: &HashSet<Edge<3>>,
) -> Result<(), StructuralIssues> {
    let mut missing_edges = HashSet::new();
    for edge in cycle.edges() {
        if !edges.contains(&edge) {
            missing_edges.insert(edge);
        }
    }

    if !missing_edges.is_empty() {
        return Err(StructuralIssues {
            missing_edges,
            ..StructuralIssues::default()
        });
    }

    Ok(())
}

pub fn validate_face(
    face: &Face,
    cycles: &HashSet<Cycle<3>>,
    surfaces: &HashSet<Surface>,
) -> Result<(), StructuralIssues> {
    if let Face::Face(face) = face {
        let mut missing_surface = None;
        let mut missing_cycles = HashSet::new();

        if !surfaces.contains(&face.surface()) {
            missing_surface = Some(face.surface.get());
        }
        for cycle in face.all_cycles() {
            if !cycles.contains(&cycle) {
                missing_cycles.insert(cycle);
            }
        }

        if missing_surface.is_some() || !missing_cycles.is_empty() {
            return Err(StructuralIssues {
                missing_surface,
                missing_cycles,
                ..StructuralIssues::default()
            });
        }
    }

    Ok(())
}

/// Structural issues found during validation
///
/// Used by [`ValidationError`].
#[derive(Debug, Default, thiserror::Error)]
pub struct StructuralIssues {
    /// Missing curve found in edge validation
    pub missing_curve: Option<Curve<3>>,

    /// Missing vertices found in edge validation
    pub missing_vertices: HashSet<Vertex>,

    /// Missing edges found in cycle validation
    pub missing_edges: HashSet<Edge<3>>,

    /// Missing surface found in face validation
    pub missing_surface: Option<Surface>,

    /// Missing cycles found in face validation
    pub missing_cycles: HashSet<Cycle<3>>,
}

impl fmt::Display for StructuralIssues {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Structural issues found:")?;

        if let Some(curve) = &self.missing_curve {
            writeln!(f, "- Missing curve: {:?}", curve)?;
        }
        if !self.missing_vertices.is_empty() {
            writeln!(f, "- Missing vertices:")?;

            for vertex in &self.missing_vertices {
                writeln!(f, "  - {:?}", vertex)?;
            }
        }
        if !self.missing_edges.is_empty() {
            writeln!(f, "- Missing edges:")?;

            for edge in &self.missing_edges {
                writeln!(f, "  - {}", edge)?;
            }
        }
        if let Some(surface) = &self.missing_surface {
            writeln!(f, "- Missing surface: {:?}", surface)?;
        }
        if !self.missing_cycles.is_empty() {
            writeln!(f, "- Missing cycles:")?;

            for cycle in &self.missing_cycles {
                writeln!(f, "  - {:?}", cycle)?;
            }
        }

        Ok(())
    }
}
