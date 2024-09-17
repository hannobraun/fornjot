use std::{collections::BTreeMap, sync::Arc};

use fj_math::{Circle, Line, Point};

use crate::{
    algorithms::approx::PathApproxParams, storage::Handle, topology::Surface,
};

use super::{Path, Tolerance};

/// The geometric definition of a curve
#[derive(Clone, Debug, Default)]
pub struct CurveGeom {
    /// # The redundant local definitions of the curve geometry
    ///
    /// ## Implementation Note
    ///
    /// Having multiple redundant definitions is undesirable. However, we can't
    /// just use one global definition in 3D, as we need the local 2D
    /// definitions to triangulate faces, and we currently don't have the tools
    /// to project a global definition into a local context.
    ///
    /// Eventually, it should be possible to define the geometry of a curve
    /// once, either locally or globally, and then convert that single
    /// definition into (other) local contexts, as needed. There currently is no
    /// issue to track that specifically, but there is the following issue,
    /// which is a prerequisite for making the required tooling practical:
    ///
    /// <https://github.com/hannobraun/fornjot/issues/2118>
    pub definitions: BTreeMap<Handle<Surface>, LocalCurveGeom>,
}

impl CurveGeom {
    /// # Return the local definition on the provided surface
    pub fn local_on(
        &self,
        surface: &Handle<Surface>,
    ) -> Option<&LocalCurveGeom> {
        self.definitions.get(surface)
    }
}

/// The geometric definition of a curve, in 2D surface coordinates
#[derive(Clone, Debug)]
pub struct LocalCurveGeom {
    /// The path that defines the curve on its surface
    pub path: Path<2>,
}

/// # The geometric definition of a curve
///
/// Curves are represented by polylines, their uniform intermediate
/// representation. However, this representation can be 2D (local to a surface)
/// or 3D. This enum distinguishes between the two cases.
///
/// ## Implementation Note
///
/// The name, `CurveGeom2`, is a placeholder. As of this writing, there is an
/// ongoing transition to a new geometry system, and the name `CurveGeom` is
/// still taken by an old-style type.
#[derive(Clone)]
pub enum CurveGeom2 {
    /// # The curve is defined locally on a surface
    Surface {
        /// # The geometric representation of the curve
        geometry: Arc<dyn GenPolyline<2>>,

        /// # The surface that the curve geometry is defined on
        surface: Handle<Surface>,
    },

    /// # The curve is defined globally in 3D space
    Global {
        /// # The geometric representation of the curve
        geometry: Arc<dyn GenPolyline<3>>,
    },
}

/// # Generate polylines, the uniform representation of curve geometry
///
/// This trait provides a generic and uniform interface to curve geometry. It is
/// implemented by types that represent specific kinds of curve geometry.
///
/// It is generic over the dimensionality of the generated polyline. Typically,
/// two variants should be implemented per curve geometry type:
///
/// - `CurveGeom2<2>` for surface-local geometry.
/// - `CurveGeom2<3>` for global 3D geometry.
///
///
/// ## Determinism
///
/// For a given curve and a given tolerance, the uniform representation of a
/// curve must be deterministic. This means that the same representation must be
/// returned, regardless of which points on the curve are queried, and in what
/// order.
pub trait GenPolyline<const D: usize> {
    /// # Access the origin of the curve
    fn origin(&self) -> Point<D>;

    /// # Compute a line segment to approximate the curve at this point
    ///
    /// ## Degenerate Case
    ///
    /// If the curve requires no approximation (meaning it is a line), then per
    /// convention, a degenerate line segment is returned, that collapses to the
    /// provided point.
    fn line_segment_at(
        &self,
        point: Point<1>,
        tolerance: Tolerance,
    ) -> [Point<D>; 2];
}

impl<const D: usize> GenPolyline<D> for Circle<D> {
    fn origin(&self) -> Point<D> {
        self.center() + self.a()
    }

    fn line_segment_at(
        &self,
        point: Point<1>,
        tolerance: Tolerance,
    ) -> [Point<D>; 2] {
        let params = PathApproxParams::for_circle(self, tolerance);

        [point.t - params.increment(), point.t + params.increment()]
            .map(|point_circle| self.point_from_circle_coords([point_circle]))
    }
}

impl<const D: usize> GenPolyline<D> for Line<D> {
    fn origin(&self) -> Point<D> {
        self.origin()
    }

    fn line_segment_at(&self, point: Point<1>, _: Tolerance) -> [Point<D>; 2] {
        // Collapse line segment into a point, as per documentation.
        let point = self.origin() + self.direction() * point.t;

        [point, point]
    }
}

// This implementation is temporary, to ease the transition towards a curve
// geometry trait. Eventually, `CurveGeom2` is expected to replace `Path`.
impl<const D: usize> GenPolyline<D> for Path<D> {
    fn origin(&self) -> Point<D> {
        match self {
            Self::Circle(circle) => circle.origin(),
            Self::Line(line) => line.origin(),
        }
    }

    fn line_segment_at(
        &self,
        point: Point<1>,
        tolerance: Tolerance,
    ) -> [Point<D>; 2] {
        match self {
            Self::Circle(circle) => circle.line_segment_at(point, tolerance),
            Self::Line(line) => line.line_segment_at(point, tolerance),
        }
    }
}
