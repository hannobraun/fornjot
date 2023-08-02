use std::borrow::Borrow;

use crate::{
    objects::Face,
    operations::{Insert, IsInsertedNo, IsInsertedYes, Polygon},
    services::Services,
};

use super::Reverse;

impl Reverse for Face {
    fn reverse(&self, services: &mut Services) -> Self {
        let region = self.region().reverse(services).insert(services);
        Face::new(self.surface().clone(), region)
    }
}

impl<const D: usize> Reverse for Polygon<D, IsInsertedNo> {
    fn reverse(&self, services: &mut Services) -> Self {
        let face = self.face.borrow().reverse(services);
        self.replace_face(face)
    }
}

impl<const D: usize> Reverse for Polygon<D, IsInsertedYes> {
    fn reverse(&self, services: &mut Services) -> Self {
        let face: &Face = self.face.borrow();
        let face = face.reverse(services).insert(services);

        self.replace_face(face)
    }
}
