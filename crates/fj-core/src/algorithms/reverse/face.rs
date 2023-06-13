use crate::{
    objects::{Face, Region},
    operations::Insert,
    services::Services,
    storage::Handle,
};

use super::Reverse;

impl Reverse for Handle<Face> {
    fn reverse(self, services: &mut Services) -> Self {
        let exterior = self.exterior().clone().reverse(services);
        let interiors = self
            .interiors()
            .map(|cycle| cycle.clone().reverse(services))
            .collect::<Vec<_>>();

        let region = Region::new(exterior, interiors, self.color());
        Face::new(self.surface().clone(), region).insert(services)
    }
}
