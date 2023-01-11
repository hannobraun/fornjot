use std::collections::VecDeque;

use crate::{
    objects::{Cycle, HalfEdge},
    partial::{MaybeSurfacePath, Partial, PartialCycle, PartialFace},
};

use super::{CycleBuilder, ObjectArgument};

/// Builder API for [`PartialFace`]
pub trait FaceBuilder {
    /// Connect the face to another face at the provided half-edges
    ///
    /// Assumes that the provided half-edges, once translated into local
    /// equivalents of this face, will not form a cycle.
    ///
    /// Returns the local equivalents of the provided half-edges and, as the
    /// last entry, an additional half-edge that closes the cycle.
    fn connect_to_open_edges<O>(
        &mut self,
        edges: O,
    ) -> O::SizePlusOne<Partial<HalfEdge>>
    where
        O: ObjectArgument<Partial<HalfEdge>>;

    /// Add an interior cycle
    fn add_interior(&mut self) -> Partial<Cycle>;
}

impl FaceBuilder for PartialFace {
    fn connect_to_open_edges<O>(
        &mut self,
        edges: O,
    ) -> O::SizePlusOne<Partial<HalfEdge>>
    where
        O: ObjectArgument<Partial<HalfEdge>>,
    {
        // We need to create the additional half-edge last, but at the same time
        // need to provide it to the `map_plus_one` method first. Really no
        // choice but to create them all in one go, as we do here.
        let mut half_edges = VecDeque::new();
        for _ in 0..edges.num_objects() {
            half_edges.push_back(self.exterior.write().add_half_edge());
        }
        let additional_half_edge = self.exterior.write().add_half_edge();

        edges.map_plus_one(additional_half_edge, |other| {
            let mut this = half_edges.pop_front().expect(
                "Pushed correct number of half-edges; should be able to pop",
            );

            let global_curve = other.read().curve().read().global_form.clone();
            this.write().curve().write().global_form = global_curve.clone();
            this.write().global_form.write().curve = global_curve;

            this.write().curve().write().path = other
                .read()
                .curve()
                .read()
                .path
                .as_ref()
                .map(MaybeSurfacePath::to_undefined);

            for (this, other) in this
                .write()
                .vertices
                .iter_mut()
                .zip(other.read().vertices.iter().rev())
            {
                this.write().position = other.read().position;
                this.write()
                    .surface_form
                    .write()
                    .global_form
                    .write()
                    .position = other
                    .read()
                    .surface_form
                    .read()
                    .global_form
                    .read()
                    .position;
            }

            this
        })
    }

    fn add_interior(&mut self) -> Partial<Cycle> {
        let cycle = Partial::from_partial(PartialCycle {
            surface: self.exterior.read().surface.clone(),
            ..Default::default()
        });
        self.interiors.push(cycle.clone());
        cycle
    }
}
