use fj_math::Point;
use itertools::Itertools;

use crate::{
    objects::{Cycle, HalfEdge},
    operations::{BuildHalfEdge, Insert},
    services::Services,
};

/// Build a [`Cycle`]
pub trait BuildCycle {
    /// Build an empty cycle
    fn empty() -> Cycle {
        Cycle::new([])
    }

    /// Build a polygon
    fn polygon<P, Ps>(points: Ps, services: &mut Services) -> Cycle
    where
        P: Into<Point<2>>,
        Ps: IntoIterator<Item = P>,
        Ps::IntoIter: Clone + ExactSizeIterator,
    {
        let half_edges = points
            .into_iter()
            .map(Into::into)
            .circular_tuple_windows()
            .map(|(start, end)| {
                HalfEdge::line_segment([start, end], None, services)
                    .insert(services)
            });

        Cycle::new(half_edges)
    }
}

impl BuildCycle for Cycle {}
