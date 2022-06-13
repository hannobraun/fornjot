use std::fmt;

use fj_math::{Point, Scalar};

use crate::topology::Edge;

pub fn validate_edge(
    edge: &Edge<3>,
    max_distance: impl Into<Scalar>,
) -> Result<(), CoherenceIssues> {
    let max_distance = max_distance.into();

    // Validate that the local and canonical forms of the vertices match. As a
    // side effect, this also happens to validate that the canonical forms of
    // the vertices lie on the curve.

    let mut edge_vertex_mismatches = Vec::new();

    for vertex in edge.vertices.iter() {
        let local = *vertex.local();
        let local_3d = edge.curve().point_from_curve_coords(local);
        let canonical = vertex.canonical().get().point;
        let distance = (local_3d - canonical).magnitude();

        if distance > max_distance {
            edge_vertex_mismatches.push(CoherenceMismatch {
                local,
                local_3d,
                canonical,
                distance,
            });
        }
    }

    if !edge_vertex_mismatches.is_empty() {
        return Err(CoherenceIssues {
            edge_vertex_mismatches,
        });
    }

    Ok(())
}

/// Geometric issues found during validation
///
/// Used by [`ValidationError`].
///
/// [`ValidationError`]: super::ValidationError
#[derive(Debug, Default, thiserror::Error)]
pub struct CoherenceIssues {
    /// Mismatches between the local and canonical forms of edge vertices
    pub edge_vertex_mismatches: Vec<CoherenceMismatch>,
}

impl fmt::Display for CoherenceIssues {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Geometric issues found:")?;

        if !self.edge_vertex_mismatches.is_empty() {
            writeln!(f, "- Edge vertex mismatches:")?;

            for mismatch in &self.edge_vertex_mismatches {
                writeln!(f, "  - {}", mismatch)?;
            }
        }

        Ok(())
    }
}

/// A mismatch between the local and canonical forms of an edge vertex
///
/// Used in [`CoherenceIssues`].
#[derive(Debug)]
pub struct CoherenceMismatch {
    /// The local form of the vertex
    pub local: Point<1>,

    /// The local form of the vertex, converted to 3D
    pub local_3d: Point<3>,

    /// The canonical form of the vertex
    pub canonical: Point<3>,

    /// The distance between the local and canonical forms
    pub distance: Scalar,
}

impl fmt::Display for CoherenceMismatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "local: {:?} (in 3D: {:?}), canonical: {:?}, distance: {}",
            self.local, self.local_3d, self.canonical, self.distance
        )
    }
}

#[cfg(test)]
mod tests {
    use fj_math::Scalar;

    use crate::{
        shape::{LocalForm, Shape},
        topology::Edge,
    };

    #[test]
    fn validate_edge() -> anyhow::Result<()> {
        let mut shape = Shape::new();

        let deviation = Scalar::from_f64(0.25);

        let edge = Edge::builder(&mut shape)
            .build_line_segment_from_points([[0., 0., 0.], [1., 0., 0.]])?
            .get();
        let edge = Edge {
            vertices: edge.vertices.map(|vertex| {
                LocalForm::new(
                    *vertex.local() + [deviation],
                    vertex.canonical(),
                )
            }),
            ..edge
        };
        assert!(super::validate_edge(&edge, deviation * 2.).is_ok());
        assert!(super::validate_edge(&edge, deviation / 2.).is_err());

        Ok(())
    }
}
