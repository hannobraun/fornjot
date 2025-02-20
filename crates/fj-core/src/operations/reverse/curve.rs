use crate::{
    Core,
    operations::{derive::DeriveFrom, insert::Insert},
    storage::Handle,
    topology::{Curve, Surface},
};

use super::ReverseCurveCoordinateSystems;

impl ReverseCurveCoordinateSystems for (&Handle<Curve>, &Handle<Surface>) {
    type Reversed = Handle<Curve>;

    fn reverse_curve_coordinate_systems(
        self,
        core: &mut Core,
    ) -> Self::Reversed {
        // We don't *actually* need a surface here, because *all* local
        // definitions need to be reversed in sync anyway. However, then this
        // method couldn't be called in `HalfEdge`'s implementation, meaning the
        // way this trait works had to change completely.
        //
        // At this point it's probably not worth it to re-architect the trait,
        // since eventually, redundant geometry definitions shouldn't exist any
        // more, and then the existence of this trait, at least in its current
        // form, is in question anyway.
        //
        // Once the dust has settled, we can figure out what needs to happen
        // here. In the meantime, we can have this weird implementation, and
        // rely on the callers to make sure everything stays consistent.
        //
        // [1]: https://github.com/hannobraun/fornjot/issues/2290
        let (curve, surface) = self;

        let mut curve_geom = core
            .layers
            .geometry
            .of_curve(curve)
            .unwrap()
            .local_on(surface)
            .unwrap()
            .clone();
        curve_geom.path = curve_geom.path.reverse();

        let curve = Curve::new().insert(core).derive_from(curve, core);

        core.layers.geometry.define_curve(
            curve.clone(),
            surface.clone(),
            curve_geom,
        );

        curve
    }
}
