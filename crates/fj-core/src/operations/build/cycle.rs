use fj_math::{Point, Scalar, Vector};
use itertools::Itertools;

use crate::{
    operations::{build::BuildHalfEdge, insert::Insert, update::UpdateCycle},
    storage::Handle,
    topology::{Cycle, HalfEdge, Surface},
    Core,
};

/// Build a [`Cycle`]
///
/// See [module-level documentation] for context.
///
/// [module-level documentation]: super
pub trait BuildCycle {
    /// Build an empty cycle
    fn empty() -> Cycle {
        Cycle::new([])
    }

    /// # Build a circle
    ///
    /// This circle is built out of 4 distinct arcs.
    ///
    /// ## Implementation Note
    ///
    /// The cycle can't be built out of a single half-edge. That would be
    /// invalid although there's not validation check to document and enforce
    /// that yet. Please refer to the following issue for more information:
    /// <https://github.com/hannobraun/fornjot/issues/2374>
    ///
    /// The cycle is built out of 4 arcs specifically, because that's easier to
    /// implement than three, and building it out of two creates geometry that
    /// the cycle winding code can't handle right now. The following issue has
    /// more information on the cycle winding problems:
    /// <https://github.com/hannobraun/fornjot/issues/2130>
    fn circle(
        center: impl Into<Point<2>>,
        radius: impl Into<Scalar>,
        surface: Handle<Surface>,
        core: &mut Core,
    ) -> Cycle {
        let center = center.into();
        let radius = radius.into();

        let radius_right = Vector::from([radius, Scalar::ZERO]);
        let radius_up = Vector::from([Scalar::ZERO, radius]);

        let a = center + radius_right;
        let b = center + radius_up;
        let c = center - radius_right;
        let d = center - radius_up;

        let angle = Scalar::TAU / 4.;

        let ab = HalfEdge::arc(a, b, angle, surface.clone(), core).insert(core);
        let bc = HalfEdge::arc(b, c, angle, surface.clone(), core).insert(core);
        let cd = HalfEdge::arc(c, d, angle, surface.clone(), core).insert(core);
        let da = HalfEdge::arc(d, a, angle, surface.clone(), core).insert(core);

        Cycle::empty().add_half_edges([ab, bc, cd, da], core)
    }

    /// Build a polygon
    fn polygon<P, Ps>(
        points: Ps,
        surface: Handle<Surface>,
        core: &mut Core,
    ) -> Cycle
    where
        P: Into<Point<2>>,
        Ps: IntoIterator<Item = P>,
        Ps::IntoIter: Clone + ExactSizeIterator,
    {
        let edges = points
            .into_iter()
            .map(Into::into)
            .circular_tuple_windows()
            .map(|(start, end)| {
                HalfEdge::line_segment(
                    [start, end],
                    None,
                    surface.clone(),
                    core,
                )
            });

        Cycle::new(edges)
    }
}

impl BuildCycle for Cycle {}
