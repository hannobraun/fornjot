use std::{collections::HashSet, fmt};

use crate::{
    geometry::{Curve, Surface},
    shape::{stores::Stores, Handle},
    topology::{Cycle, Edge, Face, Vertex},
};

pub fn validate_edge(
    edge: &Edge<3>,
    stores: &Stores,
) -> Result<(), StructuralIssues> {
    let mut missing_curve = None;
    let mut missing_vertices = HashSet::new();

    if !stores.curves.contains(&edge.curve.canonical()) {
        missing_curve = Some(edge.curve.canonical());
    }
    for vertex in edge.vertices.iter() {
        if !stores.vertices.contains(&vertex.canonical()) {
            missing_vertices.insert(vertex.canonical().clone());
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
    stores: &Stores,
) -> Result<(), StructuralIssues> {
    let mut missing_edges = HashSet::new();
    for edge in &cycle.edges {
        let edge = edge.canonical();

        if !stores.edges.contains(&edge) {
            missing_edges.insert(edge.clone());
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
    stores: &Stores,
) -> Result<(), StructuralIssues> {
    if let Face::Face(face) = face {
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
    pub missing_curve: Option<Handle<Curve<3>>>,

    /// Missing vertices found in edge validation
    pub missing_vertices: HashSet<Handle<Vertex>>,

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
