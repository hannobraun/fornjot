use std::collections::BTreeMap;

use fj_math::Vector;

use crate::{
    storage::Handle,
    topology::{HalfEdge, Surface, Topology},
};

use super::{GlobalPath, HalfEdgeGeom, SurfaceGeom};

/// Geometric data that is associated with topological objects
pub struct Geometry {
    half_edge: BTreeMap<Handle<HalfEdge>, HalfEdgeGeom>,
    surface: BTreeMap<Handle<Surface>, SurfaceGeom>,

    xy_plane: Handle<Surface>,
    xz_plane: Handle<Surface>,
    yz_plane: Handle<Surface>,
}

impl Geometry {
    /// Create a new instance of `Geometry`
    pub fn new(topology: &Topology) -> Self {
        let mut self_ = Self {
            half_edge: BTreeMap::new(),
            surface: BTreeMap::new(),

            xy_plane: topology.surfaces.xy_plane(),
            xz_plane: topology.surfaces.xz_plane(),
            yz_plane: topology.surfaces.yz_plane(),
        };

        self_.define_surface_inner(
            self_.xy_plane.clone(),
            SurfaceGeom {
                u: GlobalPath::x_axis(),
                v: Vector::unit_y(),
            },
        );
        self_.define_surface_inner(
            self_.xz_plane.clone(),
            SurfaceGeom {
                u: GlobalPath::x_axis(),
                v: Vector::unit_z(),
            },
        );
        self_.define_surface_inner(
            self_.yz_plane.clone(),
            SurfaceGeom {
                u: GlobalPath::y_axis(),
                v: Vector::unit_z(),
            },
        );

        self_
    }

    pub(crate) fn define_half_edge_inner(
        &mut self,
        half_edge: Handle<HalfEdge>,
        geometry: HalfEdgeGeom,
    ) {
        self.half_edge.insert(half_edge, geometry);
    }

    pub(crate) fn define_surface_inner(
        &mut self,
        surface: Handle<Surface>,
        geometry: SurfaceGeom,
    ) {
        self.surface.insert(surface, geometry);
    }

    /// # Access the geometry of the provided half-edge
    ///
    /// ## Panics
    ///
    /// Panics, if the geometry of the half-edge is not defined.
    pub fn of_half_edge(&self, half_edge: &Handle<HalfEdge>) -> &HalfEdgeGeom {
        self.half_edge
            .get(half_edge)
            .expect("Expected geometry of half-edge to be defined")
    }

    /// # Access the geometry of the provided surface
    ///
    /// ## Panics
    ///
    /// Panics, if the geometry of the surface is not defined.
    pub fn of_surface(&self, surface: &Handle<Surface>) -> &SurfaceGeom {
        self.surface
            .get(surface)
            .expect("Expected geometry of surface to be defined")
    }

    /// Access the geometry of the xy-plane
    pub fn xy_plane(&self) -> &SurfaceGeom {
        self.of_surface(&self.xy_plane)
    }

    /// Access the geometry of the xz-plane
    pub fn xz_plane(&self) -> &SurfaceGeom {
        self.of_surface(&self.xz_plane)
    }

    /// Access the geometry of the yz-plane
    pub fn yz_plane(&self) -> &SurfaceGeom {
        self.of_surface(&self.yz_plane)
    }
}
