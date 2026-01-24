use fj_math::Point;

#[derive(Clone, Copy, Debug)]
pub struct ApproxPoint {
    pub local: Point<2>,
    pub global: Point<3>,
}

impl spade::HasPosition for ApproxPoint {
    type Scalar = f64;

    fn position(&self) -> spade::Point2<Self::Scalar> {
        let [x, y] = self.local.coords.components.map(|s| s.into_f64());
        spade::Point2 { x, y }
    }
}
