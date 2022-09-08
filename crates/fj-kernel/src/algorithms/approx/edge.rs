//! Edge approximation
//!
//! The approximation of a curve is its first vertex, combined with the
//! approximation of its curve. The second vertex is left off, as edge
//! approximations are usually used to build cycle approximations, and this way,
//! the caller doesn't have to call with duplicate vertices.

use fj_math::{Point, Scalar};

use crate::objects::{Edge, GlobalVertex, SurfaceVertex, Vertex};

use super::{
    curve::{CurveApprox, RangeOnCurve},
    Approx, ApproxPoint,
};

impl Approx for &Edge {
    type Approximation = EdgeApprox;

    fn approx(self, tolerance: super::Tolerance) -> Self::Approximation {
        // The range is only used for circles right now.
        let [a, b] = match self.vertices().get() {
            Some(vertices) => vertices.map(|&vertex| vertex),
            None => {
                // Creating vertices from nothing, just for the sake of
                // approximation is a bit weird. But this code is a temporary
                // fallback anyway. It'll do for now, and it will likely be
                // removed soon.

                let start_curve = Point::from([Scalar::ZERO]);
                let end_curve = Point::from([Scalar::TAU]);

                // We're dealing with a circle here. Start and end are identical
                // points, in global coordinates.
                let vertex_global = {
                    let point_global = self
                        .global_form()
                        .curve()
                        .kind()
                        .point_from_curve_coords(start_curve);

                    GlobalVertex::from_position(point_global)
                };

                let [start_surface, end_surface] = [start_curve, end_curve]
                    .map(|point_curve| {
                        let point_surface = self
                            .curve()
                            .kind()
                            .point_from_curve_coords(point_curve);
                        SurfaceVertex::new(
                            point_surface,
                            *self.curve().surface(),
                            vertex_global,
                        )
                    });

                let a = Vertex::new(
                    start_curve,
                    *self.curve(),
                    start_surface,
                    vertex_global,
                );
                let b = Vertex::new(
                    end_curve,
                    *self.curve(),
                    end_surface,
                    vertex_global,
                );

                [a, b]
            }
        };

        let range = RangeOnCurve::new([a, b]);

        let first = ApproxPoint::new(
            a.surface_form().position(),
            a.global_form().position(),
        );
        let curve_approx = (self.curve(), range).approx(tolerance);

        EdgeApprox {
            first,
            curve_approx,
        }
    }
}

/// An approximation of an [`Edge`]
#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct EdgeApprox {
    /// The point that approximates the first vertex of the curve
    pub first: ApproxPoint<2>,

    /// The approximation of the edge's curve
    pub curve_approx: CurveApprox,
}

impl EdgeApprox {
    /// Compute the points that approximate the edge
    pub fn points(&self) -> Vec<ApproxPoint<2>> {
        let mut points = Vec::new();

        points.push(self.first.clone());
        points.extend(self.curve_approx.points.clone());

        points
    }
}
