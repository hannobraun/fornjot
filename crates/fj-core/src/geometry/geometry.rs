use std::collections::BTreeMap;

use fj_math::Vector;

use crate::{
    objects::{Objects, Surface},
    storage::{Handle, HandleWrapper},
};

use super::{GlobalPath, SurfaceGeometry};

/// Geometric data that is associated with topological objects
pub struct Geometry {
    surface: BTreeMap<HandleWrapper<Surface>, SurfaceGeometry>,

    xy_plane: Handle<Surface>,
    xz_plane: Handle<Surface>,
    yz_plane: Handle<Surface>,
}

impl Geometry {
    /// Create a new instance of `Geometry`
    pub fn new(objects: &Objects) -> Self {
        let mut self_ = Self {
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

    pub(crate) fn define_surface_inner(
        &mut self,
        surface: Handle<Surface>,
        geometry: SurfaceGeometry,
    ) {
        self.surface.insert(surface.clone().into(), geometry);
    }

    /// # Access the geometry of the provided surface
    ///
    /// ## Panics
    ///
    /// Panics, if the geometry of surface is not defined.
    pub fn of_surface(&self, surface: &Handle<Surface>) -> SurfaceGeometry {
        self.surface
            .get(&surface.clone().into())
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
