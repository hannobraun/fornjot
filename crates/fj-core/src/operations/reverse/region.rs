use crate::{objects::Region, operations::insert::Insert, Instance};

use super::{Reverse, ReverseCurveCoordinateSystems};

impl Reverse for Region {
    fn reverse(&self, core: &mut Instance) -> Self {
        let exterior = self.exterior().reverse(core).insert(&mut core.services);
        let interiors = self
            .interiors()
            .iter()
            .map(|cycle| cycle.reverse(core).insert(&mut core.services));

        Region::new(exterior, interiors, self.color())
    }
}

impl ReverseCurveCoordinateSystems for Region {
    fn reverse_curve_coordinate_systems(&self, core: &mut Instance) -> Self {
        let exterior = self
            .exterior()
            .reverse_curve_coordinate_systems(core)
            .insert(&mut core.services);
        let interiors = self.interiors().iter().map(|cycle| {
            cycle
                .reverse_curve_coordinate_systems(core)
                .insert(&mut core.services)
        });

        Region::new(exterior, interiors, self.color())
    }
}
