use fj_math::Point;

use crate::{
    geometry::{CurveBoundary, SurfacePath},
    objects::{Curve, HalfEdge, Vertex},
    operations::insert::Insert,
    storage::Handle,
    Instance,
};

/// Update a [`HalfEdge`]
pub trait UpdateHalfEdge {
    /// Update the path of the edge
    #[must_use]
    fn update_path(
        &self,
        update: impl FnOnce(SurfacePath) -> SurfacePath,
    ) -> Self;

    /// Update the boundary of the edge
    #[must_use]
    fn update_boundary(
        &self,
        update: impl FnOnce(CurveBoundary<Point<1>>) -> CurveBoundary<Point<1>>,
    ) -> Self;

    /// Update the curve of the edge
    #[must_use]
    fn update_curve<T>(
        &self,
        update: impl FnOnce(&Handle<Curve>, &mut Instance) -> T,
        core: &mut Instance,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Curve>>;

    /// Update the start vertex of the edge
    #[must_use]
    fn update_start_vertex<T>(
        &self,
        update: impl FnOnce(&Handle<Vertex>, &mut Instance) -> T,
        core: &mut Instance,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Vertex>>;
}

impl UpdateHalfEdge for HalfEdge {
    fn update_path(
        &self,
        update: impl FnOnce(SurfacePath) -> SurfacePath,
    ) -> Self {
        HalfEdge::new(
            update(self.path()),
            self.boundary(),
            self.curve().clone(),
            self.start_vertex().clone(),
        )
    }

    fn update_boundary(
        &self,
        update: impl FnOnce(CurveBoundary<Point<1>>) -> CurveBoundary<Point<1>>,
    ) -> Self {
        HalfEdge::new(
            self.path(),
            update(self.boundary()),
            self.curve().clone(),
            self.start_vertex().clone(),
        )
    }

    fn update_curve<T>(
        &self,
        update: impl FnOnce(&Handle<Curve>, &mut Instance) -> T,
        core: &mut Instance,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Curve>>,
    {
        HalfEdge::new(
            self.path(),
            self.boundary(),
            update(self.curve(), core).insert(&mut core.services),
            self.start_vertex().clone(),
        )
    }

    fn update_start_vertex<T>(
        &self,
        update: impl FnOnce(&Handle<Vertex>, &mut Instance) -> T,
        core: &mut Instance,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Vertex>>,
    {
        HalfEdge::new(
            self.path(),
            self.boundary(),
            self.curve().clone(),
            update(self.start_vertex(), core).insert(&mut core.services),
        )
    }
}
