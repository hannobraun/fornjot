use crate::{
    objects::{Face, Region},
    operations::Insert,
    services::Services,
    storage::Handle,
};

use super::Reverse;

impl Reverse for Handle<Face> {
    fn reverse(self, services: &mut Services) -> Self {
        let exterior = self.region().exterior().clone().reverse(services);
        let interiors = self
            .region()
            .interiors()
            .map(|cycle| cycle.clone().reverse(services))
            .collect::<Vec<_>>();

        let region = Region::new(exterior, interiors, self.region().color())
            .insert(services);

        Face::new(self.surface().clone(), region).insert(services)
    }
}
