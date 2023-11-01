use crate::{objects::Region, operations::insert::Insert, services::Services};

use super::{Reverse, ReverseCurveCoordinateSystems};

impl Reverse for Region {
    fn reverse(&self, services: &mut Services) -> Self {
        let exterior = self.exterior().reverse(services).insert(services);
        let interiors = self
            .interiors()
            .iter()
            .map(|cycle| cycle.reverse(services).insert(services));

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
