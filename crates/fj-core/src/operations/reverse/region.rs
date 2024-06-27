use crate::{
    operations::{derive::DeriveFrom, insert::Insert},
    topology::Region,
    Core,
};

use super::Reverse;

impl Reverse for Region {
    fn reverse(&self, core: &mut Core) -> Self {
        let exterior = self
            .exterior()
            .reverse(core)
            .insert(core)
            .derive_from(self.exterior(), core);
        let interiors = self.interiors().iter().map(|cycle| {
            cycle.reverse(core).insert(core).derive_from(cycle, core)
        });

        Region::new(exterior, interiors)
    }
}
