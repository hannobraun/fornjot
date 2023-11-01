use std::borrow::Borrow;

use crate::{
    objects::Face,
    operations::{Insert, IsInsertedNo, IsInsertedYes, build::Polygon},
    services::Services,
};

use super::{Reverse, ReverseCurveCoordinateSystems};

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

impl ReverseCurveCoordinateSystems for Face {
    fn reverse_curve_coordinate_systems(
        &self,
        services: &mut Services,
    ) -> Self {
        let region = self
            .region()
            .reverse_curve_coordinate_systems(services)
            .insert(services);
        Face::new(self.surface().clone(), region)
    }
}

impl<const D: usize> ReverseCurveCoordinateSystems
    for Polygon<D, IsInsertedNo>
{
    fn reverse_curve_coordinate_systems(
        &self,
        services: &mut Services,
    ) -> Self {
        let face = self
            .face
            .borrow()
            .reverse_curve_coordinate_systems(services);
        self.replace_face(face)
    }
}

impl<const D: usize> ReverseCurveCoordinateSystems
    for Polygon<D, IsInsertedYes>
{
    fn reverse_curve_coordinate_systems(
        &self,
        services: &mut Services,
    ) -> Self {
        let face: &Face = self.face.borrow();
        let face = face
            .reverse_curve_coordinate_systems(services)
            .insert(services);

        self.replace_face(face)
    }
}
