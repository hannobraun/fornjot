use crate::{
    objects::Face, operations::Insert, services::Services, storage::Handle,
};

use super::Reverse;

impl Reverse for Handle<Face> {
    fn reverse(self, services: &mut Services) -> Self {
        let exterior = self.exterior().clone().reverse(services);
        let interiors = self
            .interiors()
            .map(|cycle| cycle.clone().reverse(services))
            .collect::<Vec<_>>();

        Face::new(self.surface().clone(), exterior, interiors, self.color())
            .insert(&mut services.objects)
    }
}
