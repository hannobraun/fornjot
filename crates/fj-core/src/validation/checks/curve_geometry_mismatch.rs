use fj_math::{Point, Scalar};

use crate::{storage::Handle, topology::HalfEdge};

/// # [`Shell`] contains [`Curve`] with contradicting geometry definitions
///
/// Curve geometry is defined locally, in the 2D coordinates of a surface. A
/// curve can be on multiple surfaces, where those intersect, and these local
/// definitions exist for all surfaces that a curve is on.
///
/// This means that multiple redundant definitions might exist for each curve.
/// This validation check makes sure that these definitions match.
///
/// ## Implementation Note
///
/// That multiple redundant definitions exist, is undesirable in the first
/// place. However, we can't just use one global definition in 3D, as we need
/// the local 2D definitions to approximate and triangulate curves, and we
/// currently don't have the tools to project a global definition into a local
/// context.
///
/// Eventually, it should be possible to define the geometry of a curve once,
/// either locally or globally, and then convert that single definition into
/// (other) local contexts, as needed. There currently is no issue to track that
/// specifically, but there is the following issue, which is a prerequisite for
/// making the required tooling practical:
///
/// <https://github.com/hannobraun/fornjot/issues/2118>
///
/// [`Shell`]: crate::topology::Shell
/// [`Curve`]: crate::topology::Curve
#[derive(Clone, Debug)]
pub struct CurveGeometryMismatch {
    /// One of the half-edges, whose curves have mismatching geometry
    pub half_edge_a: Handle<HalfEdge>,

    /// The other of the half-edges, whose curves have mismatching geometry
    pub half_edge_b: Handle<HalfEdge>,

    /// The point on the curves, where they don't match
    pub point_curve: Point<1>,

    /// The same point in 3D coordinates, according to `half_edge_a`'s curve
    pub point_a: Point<3>,

    /// The same point in 3D coordinates, according to `half_edge_b`'s curve
    pub point_b: Point<3>,

    /// The distance between those 3D coordinates
    pub distance: Scalar,
}
