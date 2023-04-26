use std::ops::RangeInclusive;

use crate::{
    objects::Cycle,
    operations::{Insert, UpdateCycle, UpdateHalfEdge},
    services::Services,
};

/// Join a [`Cycle`] to another
pub trait JoinCycle {
    /// Join the cycle to another
    ///
    /// Joins the cycle to the other at the provided ranges. The ranges specify
    /// the indices of the half-edges that are joined together.
    ///
    /// A modulo operation is applied to all indices before use, so in a cycle
    /// of 3 half-edges, indices `0` and `3` refer to the same half-edge. This
    /// allows for specifying a range that crosses the "seam" of the cycle.
    ///
    /// # Panics
    ///
    /// Panics, if the ranges have different lengths.
    ///
    /// # Implementation Note
    ///
    /// The use of the `RangeInclusive` type might be a bit limiting, as other
    /// range types might be more convenient in a given use case. This
    /// implementation was chosen for now, as it wasn't clear whether the
    /// additional complexity of using `RangeBounds` would be worth it.
    ///
    /// A solution based on `SliceIndex` could theoretically be used, but that
    /// trait is partially unstable. In addition, it's not clear how it could be
    /// used generically, as it could yield a range (which can be iterated over)
    /// or a single item (which can not). This is not a hard problem in
    /// principle (a single item could just be an iterator of length 1), but I
    /// don't see it how to address this in Rust in a reasonable way.
    ///
    /// Maybe a custom trait that is implemented for `usize` and all range types
    /// would be the best solution.
    fn join_to(
        &self,
        other: &Cycle,
        range: RangeInclusive<usize>,
        other_range: RangeInclusive<usize>,
        services: &mut Services,
    ) -> Self;
}

impl JoinCycle for Cycle {
    fn join_to(
        &self,
        other: &Cycle,
        range: RangeInclusive<usize>,
        range_other: RangeInclusive<usize>,
        services: &mut Services,
    ) -> Self {
        assert_eq!(
            range.end() - range.start(),
            range_other.end() - range_other.start()
        );

        let mut cycle = self.clone();

        for (index, index_other) in range.zip(range_other) {
            let index = index % self.len();
            let index_other = index_other % self.len();

            let half_edge = self
                .nth_half_edge(index)
                .expect("Index must be valid, due to use of `%` above");
            let half_edge_other = other
                .nth_half_edge(index_other)
                .expect("Index must be valid, due to use of `%` above");

            let vertex_a = other
                .half_edge_after(half_edge_other)
                .expect("Expected other cycle to contain edge")
                .start_vertex()
                .clone();
            let vertex_b = half_edge_other.start_vertex().clone();

            let next_edge = cycle
                .half_edge_after(half_edge)
                .expect("Expected this cycle to contain edge");

            let this_joined = half_edge
                .replace_start_vertex(vertex_a)
                .replace_global_form(half_edge_other.global_form().clone())
                .insert(services);
            let next_joined =
                next_edge.replace_start_vertex(vertex_b).insert(services);

            cycle = cycle
                .replace_half_edge(half_edge, this_joined)
                .replace_half_edge(next_edge, next_joined)
        }

        cycle
    }
}
