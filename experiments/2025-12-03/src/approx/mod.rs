use fj_math::Point;

#[derive(Clone, Copy, Debug)]
pub struct ApproxPoint<const D: usize> {
    pub local: Point<D>,
    pub global: Point<3>,
}

impl spade::HasPosition for ApproxPoint<2> {
    type Scalar = f64;

    fn position(&self) -> spade::Point2<Self::Scalar> {
        let [x, y] = self.local.coords.components.map(|s| s.into_f64());
        spade::Point2 { x, y }
    }
}
