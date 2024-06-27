use std::borrow::Borrow;

use crate::{
    operations::{
        build::Polygon,
        derive::DeriveFrom,
        insert::{Insert, IsInsertedNo, IsInsertedYes},
    },
    topology::Face,
    Core,
};

use super::Reverse;

impl Reverse for Face {
    fn reverse(&self, core: &mut Core) -> Self {
        let region = self
            .region()
            .reverse(core)
            .insert(core)
            .derive_from(self.region(), core);
        Face::new(self.surface().clone(), region)
    }
}

impl<const D: usize> Reverse for Polygon<D, IsInsertedNo> {
    fn reverse(&self, core: &mut Core) -> Self {
        let face = self.face.borrow().reverse(core);
        self.replace_face(face)
    }
}

impl<const D: usize> Reverse for Polygon<D, IsInsertedYes> {
    fn reverse(&self, core: &mut Core) -> Self {
        let face: &Face = self.face.borrow();
        let face = face
            .reverse(core)
            .insert(core)
            .derive_from(&self.face, core);

        self.replace_face(face)
    }
}
