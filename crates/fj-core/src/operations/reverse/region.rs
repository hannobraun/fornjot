use crate::{objects::Region, operations::insert::Insert, Core};

use super::{Reverse, ReverseCurveCoordinateSystems};

impl Reverse for Region {
    fn reverse(&self, core: &mut Core) -> Self {
        let exterior = self.exterior().reverse(core).insert(core);
        let interiors = self
            .interiors()
            .iter()
            .map(|cycle| cycle.reverse(core).insert(core));

        Region::new(exterior, interiors, self.color())
    }
}

impl ReverseCurveCoordinateSystems for Region {
    fn reverse_curve_coordinate_systems(&self, core: &mut Core) -> Self {
        let exterior = self
            .exterior()
            .reverse_curve_coordinate_systems(core)
            .insert(core);
        let interiors = self.interiors().iter().map(|cycle| {
            cycle.reverse_curve_coordinate_systems(core).insert(core)
        });

        Region::new(exterior, interiors, self.color())
    }
}
