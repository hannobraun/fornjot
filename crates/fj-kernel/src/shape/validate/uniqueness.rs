use std::fmt;

use crate::{
    objects::{Edge, Vertex},
    shape::{stores::Store, Handle},
};

pub fn validate_vertex(
    vertex: &Vertex,
    handle: Option<&Handle<Vertex>>,
    vertices: &Store<Vertex>,
) -> Result<(), UniquenessIssues> {
    for existing in vertices.iter() {
        if Some(&existing) == handle {
            continue;
        }

        if &existing.get() == vertex {
            return Err(UniquenessIssues {
                duplicate_vertex: Some(existing),
                ..UniquenessIssues::default()
            });
        }
    }

    Ok(())
}

/// Validate that there isn't already an identical edge in the store
///
/// # Implementation Note
///
/// This only compares the vertices of the edge. This is enough for now, as only
/// straight edges have vertices to bound them. Once this is no longer the case,
/// this code will have to be updated.
pub fn validate_edge(
    edge: &Edge<3>,
    handle: Option<&Handle<Edge<3>>>,
    edges: &Store<Edge<3>>,
) -> Result<(), UniquenessIssues> {
    for existing in edges.iter() {
        if Some(&existing) == handle {
            continue;
        }

        if existing.get().vertices.are_same(&edge.vertices) {
            return Err(UniquenessIssues {
                duplicate_edge: Some(DuplicateEdge {
                    existing: existing.get(),
                    new: edge.clone(),
                }),
                ..UniquenessIssues::default()
            });
        }
    }

    Ok(())
}

/// Uniqueness issues found during validation
///
/// Used by [`ValidationError`].
///
/// # Implementation Note
///
/// This struct doesn't carry any actual information, currently. Information
/// about the specific uniqueness issues found can be added as required. For
/// now, this struct exists to ease the error handling code.
///
/// [`ValidationError`]: super::ValidationError
#[derive(Debug, Default, thiserror::Error)]
pub struct UniquenessIssues {
    /// Duplicate vertex found
    pub duplicate_vertex: Option<Handle<Vertex>>,

    /// Duplicate edge found
    pub duplicate_edge: Option<DuplicateEdge>,
}

impl fmt::Display for UniquenessIssues {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Uniqueness issues found:")?;

        if let Some(duplicate_vertex) = &self.duplicate_vertex {
            writeln!(f, "- Duplicate vertex ({:?}", duplicate_vertex)?;
        }

        if let Some(duplicate_edge) = &self.duplicate_edge {
            writeln!(f, "- Duplicate edge ({})", duplicate_edge)?;
        }

        Ok(())
    }
}

/// A duplicate edge
///
/// Used in [`UniquenessIssues`]
#[derive(Debug)]
pub struct DuplicateEdge {
    /// The existing edge
    pub existing: Edge<3>,

    /// The new edge
    pub new: Edge<3>,
}

impl fmt::Display for DuplicateEdge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "existing: {:?}, new: {:?}", self.existing, self.new)
    }
}
