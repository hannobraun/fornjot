use std::collections::BTreeMap;

use fj_math::Vector;

use crate::{
    storage::Handle,
    topology::{HalfEdge, Surface, Topology},
};

use super::{GlobalPath, HalfEdgeGeometry, SurfaceGeometry};

/// Geometric data that is associated with topological objects
pub struct Geometry {
    half_edge: BTreeMap<Handle<HalfEdge>, HalfEdgeGeometry>,
    surface: BTreeMap<Handle<Surface>, SurfaceGeometry>,

    xy_plane: Handle<Surface>,
    xz_plane: Handle<Surface>,
    yz_plane: Handle<Surface>,
}

impl Geometry {
    /// Create a new instance of `Geometry`
    pub fn new(objects: &Topology) -> Self {
        let mut self_ = Self {
            half_edge: BTreeMap::new(),
            surface: BTreeMap::new(),

            xy_plane: objects.surfaces.xy_plane(),
            xz_plane: objects.surfaces.xz_plane(),
            yz_plane: objects.surfaces.yz_plane(),
        };

        self_.define_surface_inner(
            self_.xy_plane.clone(),
            SurfaceGeometry {
                u: GlobalPath::x_axis(),
                v: Vector::unit_y(),
            },
        );
        self_.define_surface_inner(
            self_.xz_plane.clone(),
            SurfaceGeometry {
                u: GlobalPath::x_axis(),
                v: Vector::unit_z(),
            },
        );
        self_.define_surface_inner(
            self_.yz_plane.clone(),
            SurfaceGeometry {
                u: GlobalPath::y_axis(),
                v: Vector::unit_z(),
            },
        );

        self_
    }

    pub(crate) fn define_half_edge_inner(
        &mut self,
        half_edge: Handle<HalfEdge>,
        geometry: HalfEdgeGeometry,
    ) {
        self.half_edge.insert(half_edge, geometry);
    }

    pub(crate) fn define_surface_inner(
        &mut self,
        surface: Handle<Surface>,
        geometry: SurfaceGeometry,
    ) {
        self.surface.insert(surface, geometry);
    }

    /// # Access the geometry of the provided half-edge
    ///
    /// ## Panics
    ///
    /// Panics, if the geometry of the half-edge is not defined.
    pub fn of_half_edge(
        &self,
        half_edge: &Handle<HalfEdge>,
    ) -> HalfEdgeGeometry {
        self.half_edge
            .get(half_edge)
            .copied()
            .expect("Expected geometry of half-edge to be defined")
    }

    /// # Access the geometry of the provided surface
    ///
    /// ## Panics
    ///
    /// Panics, if the geometry of surface is not defined.
    pub fn of_surface(&self, surface: &Handle<Surface>) -> SurfaceGeometry {
        self.surface
            .get(surface)
            .copied()
            .expect("Expected geometry of surface to be defined")
    }

    /// Access the geometry of the xy-plane
    pub fn xy_plane(&self) -> SurfaceGeometry {
        self.of_surface(&self.xy_plane)
    }

    /// Access the geometry of the xz-plane
    pub fn xz_plane(&self) -> SurfaceGeometry {
        self.of_surface(&self.xz_plane)
    }

    /// Access the geometry of the yz-plane
    pub fn yz_plane(&self) -> SurfaceGeometry {
        self.of_surface(&self.yz_plane)
    }
}
