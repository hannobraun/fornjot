use fj_math::{Point, Scalar};

use crate::objects::Edge;

use super::{curve::RangeOnCurve, Approx};

impl Approx for Edge {
    type Approximation = Vec<(Point<1>, Point<3>)>;
    type Params = ();

    fn approx(
        &self,
        tolerance: super::Tolerance,
        (): Self::Params,
    ) -> Self::Approximation {
        let mut points = self.curve().approx(
            tolerance,
            // The range is only used for circles right now.
            RangeOnCurve {
                boundary: [[Scalar::ZERO].into(), [Scalar::TAU].into()],
            },
        );

        // Insert the exact vertices of this edge into the approximation. This
        // means we don't rely on the curve approximation to deliver accurate
        // representations of these vertices, which they might not be able to
        // do.
        //
        // If we used inaccurate representations of those vertices here, then
        // that would lead to bugs in the approximation, as points that should
        // refer to the same vertex would be understood to refer to very close,
        // but distinct vertices.
        let vertices = self
            .vertices()
            .convert(|vertex| (vertex.position(), vertex.global().position()));
        if let Some([a, _]) = vertices {
            points.insert(0, a);
        }

        points
    }
}
