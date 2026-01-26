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

pub struct HalfEdgeApprox {
    pub start: ApproxPoint<2>,
    pub other: Vec<ApproxPoint<2>>,
}

impl HalfEdgeApprox {
    pub fn points(&self) -> impl Iterator<Item = ApproxPoint<2>> {
        [self.start].into_iter().chain(self.other.iter().copied())
    }
}
