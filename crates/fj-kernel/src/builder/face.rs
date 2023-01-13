use std::collections::VecDeque;

use fj_interop::ext::ArrayExt;

use crate::{
    objects::{Cycle, HalfEdge, Surface},
    partial::{Partial, PartialCycle, PartialFace},
};

use super::{CycleBuilder, HalfEdgeBuilder, ObjectArgument, SurfaceBuilder};

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

    /// Connect the face to another face at the provided half-edges
    ///
    /// Assumes that the provided half-edges, once translated into local
    /// equivalents of this face, form a cycle.
    ///
    /// Returns the local equivalents of the provided half-edges.
    fn connect_to_closed_edges<O>(
        &mut self,
        edges: O,
    ) -> O::SameSize<Partial<HalfEdge>>
    where
        O: ObjectArgument<Partial<HalfEdge>>;

    /// Add an interior cycle
    fn add_interior(&mut self) -> Partial<Cycle>;

    /// Update the face's surface as a plane
    ///
    /// The plane geometry is inferred from three of the face's vertices. Also
    /// infers any undefined `SurfaceVertex` positions.
    ///
    /// # Panics
    ///
    /// Assumes that the face exterior has exactly three vertices to use. Panics
    /// otherwise. This is a temporary limitation, not a fundamental one. It
    /// could be overcome with some more work.
    fn update_surface_as_plane(&mut self) -> Partial<Surface>;
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
            this.write().update_from_other_edge(&other);
            this
        })
    }

    fn connect_to_closed_edges<O>(
        &mut self,
        edges: O,
    ) -> O::SameSize<Partial<HalfEdge>>
    where
        O: ObjectArgument<Partial<HalfEdge>>,
    {
        edges.map(|other| {
            let mut this = self.exterior.write().add_half_edge();
            this.write().update_from_other_edge(&other);
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

    fn update_surface_as_plane(&mut self) -> Partial<Surface> {
        let mut exterior = self.exterior.write();
        let mut vertices = {
            exterior.half_edges.iter().map(|half_edge| {
                half_edge.read().back().read().surface_form.clone()
            })
        };

        let vertices = {
            let array = [
                vertices.next().expect("Expected exactly three vertices"),
                vertices.next().expect("Expected exactly three vertices"),
                vertices.next().expect("Expected exactly three vertices"),
            ];

            assert!(
                vertices.next().is_none(),
                "Expected exactly three vertices"
            );

            array
        };
        let points = vertices.each_ref_ext().map(|vertex| {
            vertex
                .read()
                .global_form
                .read()
                .position
                .expect("Need global position to infer plane")
        });

        let points_surface =
            exterior.surface.write().update_as_plane_from_points(points);

        for (mut surface_vertex, point) in vertices.zip_ext(points_surface) {
            surface_vertex.write().position = Some(point);
        }

        exterior.surface.clone()
    }
}
