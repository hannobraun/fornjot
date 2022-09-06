//! Edge approximation
//!
//! The approximation of a curve is its first vertex, combined with the
//! approximation of its curve. The second vertex is left off, as edge
//! approximations are usually used to build cycle approximations, and this way,
//! the caller doesn't have to call with duplicate vertices.

use fj_math::{Point, Scalar};

use crate::objects::{Edge, GlobalVertex, SurfaceVertex, Vertex};

use super::{curve::RangeOnCurve, Approx};

impl Approx for &Edge {
    type Approximation = Vec<(Point<2>, Point<3>)>;

    fn approx(self, tolerance: super::Tolerance) -> Self::Approximation {
        // The range is only used for circles right now.
        let boundary = match self.vertices().get() {
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

        let range = RangeOnCurve { boundary };

        let mut points = (self.curve(), range).approx(tolerance);
        points.insert(
            0,
            (
                range.start().surface_form().position(),
                range.start().global_form().position(),
            ),
        );

        points
    }
}
