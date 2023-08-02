use crate::{objects::Region, operations::Insert, services::Services};

use super::Reverse;

impl Reverse for Region {
    fn reverse(&self, services: &mut Services) -> Self {
        let exterior = self.exterior().reverse(services).insert(services);
        let interiors = self
            .interiors()
            .map(|cycle| cycle.reverse(services).insert(services));

        Region::new(exterior, interiors, self.color())
    }
}
