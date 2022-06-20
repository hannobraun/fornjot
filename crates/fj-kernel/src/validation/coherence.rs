use std::fmt;

use fj_math::{Point, Scalar};

use crate::objects::Edge;

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
        let local_as_canonical = edge.curve().point_from_curve_coords(local);
        let canonical = vertex.canonical().get().point;
        let distance = (local_as_canonical - canonical).magnitude();

        if distance > max_distance {
            edge_vertex_mismatches.push(CoherenceMismatch {
                local,
                local_as_canonical,
                canonical,
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
    pub edge_vertex_mismatches: Vec<CoherenceMismatch<Point<1>, Point<3>>>,
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

/// A mismatch between the local and canonical forms of an object
///
/// Used in [`CoherenceIssues`].
#[derive(Debug)]
pub struct CoherenceMismatch<Local, Canonical> {
    /// The local form of the object
    pub local: Local,

    /// The local form of the object, converted into the canonical form
    pub local_as_canonical: Canonical,

    /// The canonical form of the object
    pub canonical: Canonical,
}

impl<Local, Canonical> fmt::Display for CoherenceMismatch<Local, Canonical>
where
    Local: fmt::Debug,
    Canonical: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "local: {:?} (converted to canonical: {:?}), canonical: {:?},",
            self.local, self.local_as_canonical, self.canonical,
        )
    }
}

#[cfg(test)]
mod tests {
    use fj_math::Scalar;

    use crate::{
        objects::Edge,
        shape::{LocalForm, Shape},
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
