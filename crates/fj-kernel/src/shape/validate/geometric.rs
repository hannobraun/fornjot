use std::fmt;

use fj_math::Scalar;

use crate::topology::Edge;

pub fn validate_edge(
    edge: &Edge<3>,
    max_distance: impl Into<Scalar>,
) -> Result<(), GeometricIssues> {
    let max_distance = max_distance.into();

    // Validate that the local and canonical forms of the vertices match. As a
    // side effect, this also happens to validate that the canonical forms of
    // the vertices lie on the curve.
    if let Some(vertices) = &edge.vertices {
        for vertex in vertices {
            let local = *vertex.local();
            let local_3d = edge.curve().point_from_curve_coords(local);

            let distance =
                (local_3d - vertex.canonical().get().point()).magnitude();

            if distance > max_distance {
                return Err(GeometricIssues);
            }
        }
    }

    Ok(())
}

/// Geometric issues found during validation
///
/// Used by [`ValidationError`].
///
/// [`ValidationError`]: super::ValidationError
#[derive(Debug, Default, thiserror::Error)]
pub struct GeometricIssues;

impl fmt::Display for GeometricIssues {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Geometric issues found")?;
        Ok(())
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
            vertices: edge.vertices.clone().map(|vertices| {
                vertices.map(|vertex| {
                    LocalForm::new(
                        *vertex.local() + [deviation],
                        vertex.canonical(),
                    )
                })
            }),
            ..edge
        };
        assert!(super::validate_edge(&edge, deviation * 2.).is_ok());
        assert!(super::validate_edge(&edge, deviation / 2.).is_err());

        Ok(())
    }
}
