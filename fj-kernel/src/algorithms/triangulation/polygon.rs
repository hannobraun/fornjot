use std::collections::HashSet;

use crate::{
    algorithms::CycleApprox,
    geometry::{self, Surface},
};

pub struct Polygon {
    pub segments: HashSet<[geometry::Point<2>; 2]>,
}

impl Polygon {
    pub fn new(
        exterior: CycleApprox,
        interiors: impl IntoIterator<Item = CycleApprox>,
        surface: Surface,
    ) -> Self {
        let segments = exterior
            .segments()
            .into_iter()
            .chain(
                interiors
                    .into_iter()
                    .flat_map(|cycle_approx| cycle_approx.segments()),
            )
            .into_iter()
            .map(|segment| {
                segment.points().map(|point| {
                    // Can't panic, unless the approximation wrongfully
                    // generates points that are not in the surface.
                    surface.point_model_to_surface(point)
                })
            })
            .collect();

        Self { segments }
    }

    pub fn contains_segment(&self, &[a, b]: &[geometry::Point<2>; 2]) -> bool {
        self.segments.contains(&[a, b]) || self.segments.contains(&[b, a])
    }
}
