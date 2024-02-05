use crate::{
    objects::Region, operations::insert::Insert, services::Services, Instance,
};

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
    fn reverse_curve_coordinate_systems(
        &self,
        services: &mut Services,
    ) -> Self {
        let exterior = self
            .exterior()
            .reverse_curve_coordinate_systems(services)
            .insert(services);
        let interiors = self.interiors().iter().map(|cycle| {
            cycle
                .reverse_curve_coordinate_systems(services)
                .insert(services)
        });

        Region::new(exterior, interiors, self.color())
    }
}
