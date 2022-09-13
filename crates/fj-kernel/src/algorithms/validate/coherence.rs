use std::fmt;

use fj_math::{Point, Scalar};

use crate::objects::{Curve, Vertex};

pub fn validate_curve(
    curve: &Curve,
    max_distance: impl Into<Scalar>,
) -> Result<(), CoherenceIssues> {
    let max_distance = max_distance.into();

    let points_curve = [-2., -1., 0., 1., 2.].map(|point| Point::from([point]));

    for point_curve in points_curve {
        let point_surface = curve.kind().point_from_curve_coords(point_curve);
        let point_surface_as_global =
            curve.surface().point_from_surface_coords(point_surface);
        let point_global = curve
            .global_form()
            .kind()
            .point_from_curve_coords(point_curve);

        let distance = (point_surface_as_global - point_global).magnitude();

        if distance > max_distance {
            Err(CurveCoherenceMismatch {
                point_curve,
                point_surface,
                point_surface_as_global,
                point_global,
                curve: *curve,
            })?
        }
    }

    Ok(())
}

pub fn validate_vertex(
    vertex: &Vertex,
    max_distance: impl Into<Scalar>,
) -> Result<(), CoherenceIssues> {
    let max_distance = max_distance.into();

    // Validate that the local and global forms of the vertex match. As a side
    // effect, this also happens to validate that the global form of the vertex
    // lies on the curve.

    let local = vertex.position();
    let local_as_global = vertex
        .curve()
        .global_form()
        .kind()
        .point_from_curve_coords(local);
    let global = vertex.global_form().position();
    let distance = (local_as_global - global).magnitude();

    if distance > max_distance {
        Err(VertexCoherenceMismatch {
            local,
            local_as_global,
            global,
        })?
    }

    Ok(())
}

/// Issues in coherence validation
#[allow(clippy::large_enum_variant)]
#[derive(Debug, thiserror::Error)]
pub enum CoherenceIssues {
    /// Mismatch between the surface and global forms of a curve
    #[error("Mismatch between surface and global forms of curve")]
    Curve(#[from] CurveCoherenceMismatch),

    /// Mismatch between the local and global coordinates of a vertex
    #[error("Mismatch between local and global coordinates of vertex")]
    Vertex(#[from] VertexCoherenceMismatch),
}

/// A mismatch between the surface and global forms of a curve
///
/// Used in [`CoherenceIssues`].
#[derive(Debug, thiserror::Error)]
pub struct CurveCoherenceMismatch {
    /// The curve coordinate for which a mismatch was found
    pub point_curve: Point<1>,

    /// The curve coordinate, converted to surface coordinates
    pub point_surface: Point<2>,

    /// The surface coordinates, converted to global coordinates
    pub point_surface_as_global: Point<3>,

    /// The curve coordinate, converted to global coordinates
    pub point_global: Point<3>,

    /// The incoherent curve
    pub curve: Curve,
}

impl fmt::Display for CurveCoherenceMismatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "local: {:?} (converted to surface: {:?}; to global: {:?}), global: {:?},",
            self.point_curve, self.point_surface, self.point_surface_as_global, self.point_global,
        )
    }
}

/// A mismatch between the local and global forms of a vertex
///
/// Used in [`CoherenceIssues`].
#[derive(Debug, Default, thiserror::Error)]
pub struct VertexCoherenceMismatch {
    /// The local form of the object
    pub local: Point<1>,

    /// The local form of the object, converted into the global form
    pub local_as_global: Point<3>,

    /// The global form of the object
    pub global: Point<3>,
}

impl fmt::Display for VertexCoherenceMismatch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "local: {:?} (converted to global: {:?}), global: {:?},",
            self.local, self.local_as_global, self.global,
        )
    }
}
