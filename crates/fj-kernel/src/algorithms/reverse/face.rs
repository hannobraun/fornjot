use crate::{
    objects::{Face, Objects},
    operations::Insert,
    services::Service,
    storage::Handle,
};

use super::Reverse;

impl Reverse for Handle<Face> {
    fn reverse(self, objects: &mut Service<Objects>) -> Self {
        let exterior = self.exterior().clone().reverse(objects);
        let interiors = self
            .interiors()
            .map(|cycle| cycle.clone().reverse(objects))
            .collect::<Vec<_>>();

        Face::new(self.surface().clone(), exterior, interiors, self.color())
            .insert(objects)
    }
}
