use crate::{
    storage::Handle,
    topology::{Cycle, HalfEdge, Shell},
};

/// Query to find the cycle that a half-edge is part of
pub trait CycleOfHalfEdge {
    /// Find the cycle that a half-edge is part of
    fn find_cycle_of_half_edge(
        &self,
        half_edge: &Handle<HalfEdge>,
    ) -> Option<Handle<Cycle>>;
}

impl CycleOfHalfEdge for Shell {
    fn find_cycle_of_half_edge(
        &self,
        half_edge: &Handle<HalfEdge>,
    ) -> Option<Handle<Cycle>> {
        for face in self.faces() {
            for cycle in face.region().all_cycles() {
                for h in cycle.half_edges() {
                    if h == half_edge {
                        return Some(cycle.clone());
                    }
                }
            }
        }

        None
    }
}
