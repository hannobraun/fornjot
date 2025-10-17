use fj_math::Point;

#[derive(Debug)]
pub struct CurveApproxFloating {
    /// # The points that approximate the curvature of the curve
    ///
    /// This does not include the boundary of the approximation.
    pub curvature: Vec<Point<1>>,
}
