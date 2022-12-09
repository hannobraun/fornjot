use crate::{
    insert::Insert,
    objects::{Face, Objects},
    partial2::{FullToPartialCache, Partial, PartialFace, PartialObject},
    services::Service,
    storage::Handle,
};

use super::Reverse;

impl Reverse for Handle<Face> {
    fn reverse(self, objects: &mut Service<Objects>) -> Self {
        let mut cache = FullToPartialCache::default();

        let exterior = Partial::from_full(
            self.exterior().clone().reverse(objects),
            &mut cache,
        );
        let interiors = self
            .interiors()
            .map(|cycle| {
                Partial::from_full(cycle.clone().reverse(objects), &mut cache)
            })
            .collect::<Vec<_>>();

        let face = PartialFace {
            exterior,
            interiors,
            color: Some(self.color()),
        };
        face.build(objects).insert(objects)
    }
}
