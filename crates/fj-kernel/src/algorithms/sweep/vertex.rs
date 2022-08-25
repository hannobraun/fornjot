use fj_interop::mesh::Color;

use crate::{
    algorithms::approx::Tolerance,
    objects::{GlobalCurve, GlobalEdge, GlobalVertex, VerticesOfEdge},
};

use super::{Path, Sweep};

impl Sweep for GlobalVertex {
    type Swept = GlobalEdge;

    fn sweep(
        self,
        path: impl Into<Path>,
        _: impl Into<Tolerance>,
        _: Color,
    ) -> Self::Swept {
        let a = self;
        let b =
            GlobalVertex::from_position(self.position() + path.into().inner());

        let curve =
            GlobalCurve::build().line_from_points([a.position(), b.position()]);

        GlobalEdge::new(curve, VerticesOfEdge::from_vertices([a, b]))
    }
}
