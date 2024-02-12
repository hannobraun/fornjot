use std::borrow::Borrow;

use crate::{
    objects::Face,
    operations::{
        build::Polygon,
        derive::DeriveFrom,
        insert::{Insert, IsInsertedNo, IsInsertedYes},
    },
    Core,
};

use super::{Reverse, ReverseCurveCoordinateSystems};

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

impl ReverseCurveCoordinateSystems for Face {
    fn reverse_curve_coordinate_systems(&self, core: &mut Core) -> Self {
        let region = self
            .region()
            .reverse_curve_coordinate_systems(core)
            .insert(core)
            .derive_from(self.region(), core);
        Face::new(self.surface().clone(), region)
    }
}

impl<const D: usize> ReverseCurveCoordinateSystems
    for Polygon<D, IsInsertedNo>
{
    fn reverse_curve_coordinate_systems(&self, core: &mut Core) -> Self {
        let face = self.face.borrow().reverse_curve_coordinate_systems(core);
        self.replace_face(face)
    }
}

impl<const D: usize> ReverseCurveCoordinateSystems
    for Polygon<D, IsInsertedYes>
{
    fn reverse_curve_coordinate_systems(&self, core: &mut Core) -> Self {
        let face: &Face = self.face.borrow();
        let face = face
            .reverse_curve_coordinate_systems(core)
            .insert(core)
            .derive_from(&self.face, core);

        self.replace_face(face)
    }
}
