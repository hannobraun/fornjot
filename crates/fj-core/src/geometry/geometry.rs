use std::collections::BTreeMap;

use fj_math::Vector;

use crate::{
    storage::Handle,
    topology::{Curve, Surface, Topology, Vertex},
};

use super::{
    surfaces::SweptCurve,
    traits::{GenPolyline, GenTriMesh},
    vertex::LocalVertexGeom,
    Path, VertexGeom,
};

/// Geometric data that is associated with topological objects
pub struct Geometry {
    curve: BTreeMap<Handle<Curve>, CurveGeom>,
    surface: BTreeMap<Handle<Surface>, SweptCurve>,
    vertex: BTreeMap<Handle<Vertex>, VertexGeom>,

    curve_generators: BTreeMap<Handle<Curve>, CurveGenerator>,
    surface_generators: BTreeMap<Handle<Surface>, SurfaceGenerator>,

    space_2d: Handle<Surface>,

    xy_plane: Handle<Surface>,
    xz_plane: Handle<Surface>,
    yz_plane: Handle<Surface>,
}

impl Geometry {
    /// Create a new instance of `Geometry`
    pub fn new(topology: &Topology) -> Self {
        let mut self_ = Self {
            curve: BTreeMap::new(),
            surface: BTreeMap::new(),
            vertex: BTreeMap::new(),

            curve_generators: BTreeMap::new(),
            surface_generators: BTreeMap::new(),

            space_2d: topology.surfaces.space_2d(),

            xy_plane: topology.surfaces.xy_plane(),
            xz_plane: topology.surfaces.xz_plane(),
            yz_plane: topology.surfaces.yz_plane(),
        };

        self_.define_surface_inner(
            self_.xy_plane.clone(),
            SweptCurve {
                u: Path::x_axis(),
                v: Vector::unit_y(),
            },
        );
        self_.define_surface_inner(
            self_.xz_plane.clone(),
            SweptCurve {
                u: Path::x_axis(),
                v: Vector::unit_z(),
            },
        );
        self_.define_surface_inner(
            self_.yz_plane.clone(),
            SweptCurve {
                u: Path::y_axis(),
                v: Vector::unit_z(),
            },
        );

        self_.define_surface_inner_2(
            self_.xy_plane.clone(),
            SurfaceGenerator {
                geometry: Box::new(SweptCurve {
                    u: Path::x_axis(),
                    v: Vector::unit_y(),
                }),
            },
        );
        self_.define_surface_inner_2(
            self_.xz_plane.clone(),
            SurfaceGenerator {
                geometry: Box::new(SweptCurve {
                    u: Path::x_axis(),
                    v: Vector::unit_z(),
                }),
            },
        );
        self_.define_surface_inner_2(
            self_.yz_plane.clone(),
            SurfaceGenerator {
                geometry: Box::new(SweptCurve {
                    u: Path::y_axis(),
                    v: Vector::unit_z(),
                }),
            },
        );

        self_
    }

    pub(crate) fn define_curve_inner(
        &mut self,
        curve: Handle<Curve>,
        surface: Handle<Surface>,
        geometry: LocalCurveGeom,
    ) {
        self.curve
            .entry(curve)
            .or_default()
            .definitions
            .insert(surface, geometry);
    }

    pub(crate) fn define_curve_inner_2(
        &mut self,
        curve: Handle<Curve>,
        geometry: CurveGenerator,
    ) {
        self.curve_generators.insert(curve, geometry);
    }

    pub(crate) fn define_surface_inner(
        &mut self,
        surface: Handle<Surface>,
        geometry: SweptCurve,
    ) {
        if surface == self.space_2d {
            panic!("Attempting to define geometry for 2D space");
        }

        if self.surface.contains_key(&surface)
            && (surface == self.xy_plane
                || surface == self.xz_plane
                || surface == self.yz_plane)
        {
            panic!("Attempting to redefine basis plane.");
        }

        self.surface.insert(surface, geometry);
    }

    pub(crate) fn define_surface_inner_2(
        &mut self,
        surface: Handle<Surface>,
        geometry: SurfaceGenerator,
    ) {
        if surface == self.space_2d {
            panic!("Attempting to define geometry for 2D space");
        }

        if self.surface_generators.contains_key(&surface)
            && (surface == self.xy_plane
                || surface == self.xz_plane
                || surface == self.yz_plane)
        {
            panic!("Attempting to redefine basis plane.");
        }

        self.surface_generators.insert(surface, geometry);
    }

    pub(crate) fn define_vertex_inner(
        &mut self,
        vertex: Handle<Vertex>,
        curve: Handle<Curve>,
        geometry: LocalVertexGeom,
    ) {
        self.vertex
            .entry(vertex)
            .or_default()
            .definitions
            .insert(curve, geometry);
    }

    /// # Access the geometry of the provided curve
    pub fn of_curve(&self, curve: &Handle<Curve>) -> Option<&CurveGeom> {
        self.curve.get(curve)
    }

    /// # Access the geometry of the provided surface
    ///
    /// ## Panics
    ///
    /// Panics, if the geometry of the surface is not defined.
    pub fn of_surface(&self, surface: &Handle<Surface>) -> &SweptCurve {
        self.surface
            .get(surface)
            .expect("Expected geometry of surface to be defined")
    }

    /// # Access the geometry of the provided vertex
    pub fn of_vertex(&self, vertex: &Handle<Vertex>) -> Option<&VertexGeom> {
        self.vertex.get(vertex)
    }

    /// # Access the geometry generator for the provided curve
    pub fn generator_for_curve(
        &self,
        curve: &Handle<Curve>,
    ) -> Option<&CurveGenerator> {
        self.curve_generators.get(curve)
    }

    /// # Access the geometry generator for the provided surface
    pub fn generator_for_surface(
        &self,
        surface: &Handle<Surface>,
    ) -> Option<&SurfaceGenerator> {
        self.surface_generators.get(surface)
    }

    /// Access the geometry of the xy-plane
    pub fn xy_plane(&self) -> &SweptCurve {
        self.of_surface(&self.xy_plane)
    }

    /// Access the geometry of the xz-plane
    pub fn xz_plane(&self) -> &SweptCurve {
        self.of_surface(&self.xz_plane)
    }

    /// Access the geometry of the yz-plane
    pub fn yz_plane(&self) -> &SweptCurve {
        self.of_surface(&self.yz_plane)
    }
}

/// The geometric definition of a curve
#[derive(Clone, Debug, Default)]
pub struct CurveGeom {
    /// # The redundant local definitions of the curve geometry
    ///
    /// ## Implementation Note
    ///
    /// Having multiple redundant definitions is undesirable. However, we can't
    /// just use one global definition in 3D, as we need the local 2D
    /// definitions to triangulate faces, and we currently don't have the tools
    /// to project a global definition into a local context.
    ///
    /// Eventually, it should be possible to define the geometry of a curve
    /// once, either locally or globally, and then convert that single
    /// definition into (other) local contexts, as needed. There currently is no
    /// issue to track that specifically, but there is the following issue,
    /// which is a prerequisite for making the required tooling practical:
    ///
    /// <https://github.com/hannobraun/fornjot/issues/2118>
    pub definitions: BTreeMap<Handle<Surface>, LocalCurveGeom>,
}

impl CurveGeom {
    /// # Return the local definition on the provided surface
    pub fn local_on(
        &self,
        surface: &Handle<Surface>,
    ) -> Option<&LocalCurveGeom> {
        self.definitions.get(surface)
    }
}

/// The geometric definition of a curve, in 2D surface coordinates
#[derive(Clone, Debug)]
pub struct LocalCurveGeom {
    /// The path that defines the curve on its surface
    pub path: Path<2>,
}

/// # The geometric definition of a curve
///
/// Curves are represented by polylines, their uniform intermediate
/// representation. However, this representation can be 2D (local to a surface)
/// or 3D. This enum distinguishes between the two cases.
///
/// ## Implementation Note
///
/// The name, `CurveGeom2`, is a placeholder. As of this writing, there is an
/// ongoing transition to a new geometry system, and the name `CurveGeom` is
/// still taken by an old-style type.
pub enum CurveGenerator {
    /// # The curve is defined locally on a surface
    Surface {
        /// # The geometric representation of the curve
        generator: Box<dyn GenPolyline<2>>,

        /// # The surface that the curve geometry is defined on
        surface: Handle<Surface>,
    },

    /// # The curve is defined globally in 3D space
    Global {
        /// # The geometric representation of the curve
        generator: Box<dyn GenPolyline<3>>,
    },
}

/// # The geometric definition of a surface
///
/// Surface are represented by triangle meshes, their uniform intermediate
/// representation.
pub struct SurfaceGenerator {
    /// # The geometric representation of the surface
    pub geometry: Box<dyn GenTriMesh>,
}
