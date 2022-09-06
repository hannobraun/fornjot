use fj_math::{Point, Scalar};

use crate::objects::Edge;

use super::{curve::RangeOnCurve, Approx};

impl Approx for Edge {
    type Approximation = Vec<(Point<2>, Point<3>)>;
    type Params = ();

    fn approx(
        &self,
        tolerance: super::Tolerance,
        (): Self::Params,
    ) -> Self::Approximation {
        // The range is only used for circles right now.
        let boundary = match self.vertices().get() {
            Some(vertices) => vertices.map(|vertex| {
                (vertex.position(), vertex.global_form().position())
            }),
            None => {
                let start_curve = Point::from([Scalar::ZERO]);
                let end_curve = Point::from([Scalar::TAU]);

                // We're dealing with a circle here. Start and end are identical
                // points, in global coordinates.
                let point_global = self
                    .global()
                    .curve()
                    .kind()
                    .point_from_curve_coords(start_curve);

                [(start_curve, point_global), (end_curve, point_global)]
            }
        };

        let range = RangeOnCurve { boundary };

        self.curve().approx(tolerance, range)
    }
}
