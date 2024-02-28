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
}

impl Geometry {
    /// Create a new instance of `Geometry`
    pub fn new(objects: &Objects) -> Self {
        let mut self_ = Self {
            surface: BTreeMap::new(),
        };

        self_.define_surface_inner(
            objects.surfaces.xy_plane(),
            SurfaceGeometry {
                u: GlobalPath::x_axis(),
                v: Vector::unit_y(),
            },
        );
        self_.define_surface_inner(
            objects.surfaces.xz_plane(),
            SurfaceGeometry {
                u: GlobalPath::x_axis(),
                v: Vector::unit_z(),
            },
        );
        self_.define_surface_inner(
            objects.surfaces.yz_plane(),
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
}
