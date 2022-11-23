use std::convert::Infallible;

use fj_math::Vector;

use crate::{
    geometry::{path::GlobalPath, surface::SurfaceGeometry},
    storage::{Handle, Store},
    validate::{
        CycleValidationError, FaceValidationError, HalfEdgeValidationError,
        SurfaceVertexValidationError, Validate, VertexValidationError,
    },
};

use super::{
    Curve, Cycle, Face, GlobalCurve, GlobalEdge, GlobalVertex, HalfEdge, Shell,
    Sketch, Solid, Surface, SurfaceVertex, Vertex,
};

/// The available object stores
///
/// # Implementation Note
///
/// The intention is to eventually manage all objects in here. Making this
/// happen is simply a case of putting in the required work. See [#1021].
///
/// [#1021]: https://github.com/hannobraun/Fornjot/issues/1021
#[derive(Debug, Default)]
pub struct Objects {
    /// Store for [`Curve`]s
    pub curves: Curves,

    /// Store for [`Cycle`]s
    pub cycles: Cycles,

    /// Store for [`Face`]s
    pub faces: Faces,

    /// Store for [`GlobalCurve`]s
    pub global_curves: GlobalCurves,

    /// Store for [`GlobalEdge`]s
    pub global_edges: GlobalEdges,

    /// Store for [`GlobalVertex`] objects
    pub global_vertices: GlobalVertices,

    /// Store for [`HalfEdge`]s
    pub half_edges: HalfEdges,

    /// Store for [`Shell`]s
    pub shells: Shells,

    /// Store for [`Sketch`]es
    pub sketches: Sketches,

    /// Store for [`Solid`]s
    pub solids: Solids,

    /// Store for [`SurfaceVertex`] objects
    pub surface_vertices: SurfaceVertices,

    /// Store for [`Surface`]s
    pub surfaces: Surfaces,

    /// Store for [`Vertex`] objects
    pub vertices: Vertices,
}

impl Objects {
    /// Construct a new instance of `Stores`
    pub fn new() -> Self {
        Self::default()
    }
}

/// Store for [`Curve`]s
#[derive(Debug, Default)]
pub struct Curves {
    store: Store<Curve>,
}

impl Curves {
    /// Reserve a slot for an object in the store
    pub fn reserve(&self) -> Handle<Curve> {
        self.store.reserve()
    }

    /// Insert a [`Curve`] into the store
    pub fn insert(
        &self,
        handle: Handle<Curve>,
        curve: Curve,
    ) -> Result<(), Infallible> {
        curve.validate()?;
        self.store.insert(handle, curve);
        Ok(())
    }
}

/// Store for [`Cycle`]s
#[derive(Debug, Default)]
pub struct Cycles {
    store: Store<Cycle>,
}

impl Cycles {
    /// Reserve a slot for an object in the store
    pub fn reserve(&self) -> Handle<Cycle> {
        self.store.reserve()
    }

    /// Insert a [`Cycle`] into the store
    pub fn insert(
        &self,
        handle: Handle<Cycle>,
        cycle: Cycle,
    ) -> Result<(), CycleValidationError> {
        cycle.validate()?;
        self.store.insert(handle, cycle);
        Ok(())
    }
}

/// Store for [`Face`]s
#[derive(Debug, Default)]
pub struct Faces {
    store: Store<Face>,
}

impl Faces {
    /// Reserve a slot for an object in the store
    pub fn reserve(&self) -> Handle<Face> {
        self.store.reserve()
    }

    /// Insert a [`Face`] into the store
    pub fn insert(
        &self,
        handle: Handle<Face>,
        face: Face,
    ) -> Result<(), FaceValidationError> {
        face.validate()?;
        self.store.insert(handle, face);
        Ok(())
    }
}

/// Store for [`GlobalCurve`]s
#[derive(Debug, Default)]
pub struct GlobalCurves {
    store: Store<GlobalCurve>,
}

impl GlobalCurves {
    /// Reserve a slot for an object in the store
    pub fn reserve(&self) -> Handle<GlobalCurve> {
        self.store.reserve()
    }

    /// Insert a [`GlobalCurve`] into the store
    pub fn insert(
        &self,
        handle: Handle<GlobalCurve>,
        global_curve: GlobalCurve,
    ) -> Result<(), Infallible> {
        global_curve.validate()?;
        self.store.insert(handle, global_curve);
        Ok(())
    }
}

/// Store for [`GlobalEdge`]s
#[derive(Debug, Default)]
pub struct GlobalEdges {
    store: Store<GlobalEdge>,
}

impl GlobalEdges {
    /// Reserve a slot for an object in the store
    pub fn reserve(&self) -> Handle<GlobalEdge> {
        self.store.reserve()
    }

    /// Insert a [`GlobalEdge`] into the store
    pub fn insert(
        &self,
        handle: Handle<GlobalEdge>,
        global_edge: GlobalEdge,
    ) -> Result<(), Infallible> {
        global_edge.validate()?;
        self.store.insert(handle, global_edge);
        Ok(())
    }
}

/// Store for [`GlobalVertex`] objects
#[derive(Debug, Default)]
pub struct GlobalVertices {
    store: Store<GlobalVertex>,
}

impl GlobalVertices {
    /// Reserve a slot for an object in the store
    pub fn reserve(&self) -> Handle<GlobalVertex> {
        self.store.reserve()
    }

    /// Insert a [`GlobalVertex`] into the store
    pub fn insert(
        &self,
        handle: Handle<GlobalVertex>,
        global_vertex: GlobalVertex,
    ) -> Result<(), Infallible> {
        global_vertex.validate()?;
        self.store.insert(handle, global_vertex);
        Ok(())
    }
}

/// Store for [`HalfEdge`]s
#[derive(Debug, Default)]
pub struct HalfEdges {
    store: Store<HalfEdge>,
}

impl HalfEdges {
    /// Reserve a slot for an object in the store
    pub fn reserve(&self) -> Handle<HalfEdge> {
        self.store.reserve()
    }

    /// Insert a [`HalfEdge`] into the store
    pub fn insert(
        &self,
        handle: Handle<HalfEdge>,
        half_edge: HalfEdge,
    ) -> Result<(), HalfEdgeValidationError> {
        half_edge.validate()?;
        self.store.insert(handle, half_edge);
        Ok(())
    }
}

/// Store for [`Shell`]s
#[derive(Debug, Default)]
pub struct Shells {
    store: Store<Shell>,
}

impl Shells {
    /// Reserve a slot for an object in the store
    pub fn reserve(&self) -> Handle<Shell> {
        self.store.reserve()
    }

    /// Insert a [`Shell`] into the store
    pub fn insert(
        &self,
        handle: Handle<Shell>,
        shell: Shell,
    ) -> Result<(), Infallible> {
        shell.validate()?;
        self.store.insert(handle, shell);
        Ok(())
    }
}

/// Store for [`Sketch`]es
#[derive(Debug, Default)]
pub struct Sketches {
    store: Store<Sketch>,
}

impl Sketches {
    /// Reserve a slot for an object in the store
    pub fn reserve(&self) -> Handle<Sketch> {
        self.store.reserve()
    }

    /// Insert a [`Sketch`] into the store
    pub fn insert(
        &self,
        handle: Handle<Sketch>,
        sketch: Sketch,
    ) -> Result<(), Infallible> {
        sketch.validate()?;
        self.store.insert(handle, sketch);
        Ok(())
    }
}

/// Store for [`Solid`]s
#[derive(Debug, Default)]
pub struct Solids {
    store: Store<Solid>,
}

impl Solids {
    /// Reserve a slot for an object in the store
    pub fn reserve(&self) -> Handle<Solid> {
        self.store.reserve()
    }

    /// Insert a [`Solid`] into the store
    pub fn insert(
        &self,
        handle: Handle<Solid>,
        solid: Solid,
    ) -> Result<(), Infallible> {
        solid.validate()?;
        self.store.insert(handle, solid);
        Ok(())
    }
}

/// Store for [`SurfaceVertex`] objects
#[derive(Debug, Default)]
pub struct SurfaceVertices {
    store: Store<SurfaceVertex>,
}

impl SurfaceVertices {
    /// Reserve a slot for an object in the store
    pub fn reserve(&self) -> Handle<SurfaceVertex> {
        self.store.reserve()
    }

    /// Insert a [`SurfaceVertex`] into the store
    pub fn insert(
        &self,
        handle: Handle<SurfaceVertex>,
        surface_vertex: SurfaceVertex,
    ) -> Result<(), SurfaceVertexValidationError> {
        surface_vertex.validate()?;
        self.store.insert(handle, surface_vertex);
        Ok(())
    }
}

/// Store for [`Surface`]s
#[derive(Debug)]
pub struct Surfaces {
    store: Store<Surface>,

    xy_plane: Handle<Surface>,
    xz_plane: Handle<Surface>,
    yz_plane: Handle<Surface>,
}

impl Surfaces {
    /// Reserve a slot for an object in the store
    pub fn reserve(&self) -> Handle<Surface> {
        self.store.reserve()
    }

    /// Insert a [`Surface`] into the store
    pub fn insert(
        &self,
        handle: Handle<Surface>,
        surface: Surface,
    ) -> Result<(), Infallible> {
        surface.validate()?;
        self.store.insert(handle, surface);
        Ok(())
    }

    /// Access the xy-plane
    pub fn xy_plane(&self) -> Handle<Surface> {
        self.xy_plane.clone()
    }

    /// Access the xz-plane
    pub fn xz_plane(&self) -> Handle<Surface> {
        self.xz_plane.clone()
    }

    /// Access the yz-plane
    pub fn yz_plane(&self) -> Handle<Surface> {
        self.yz_plane.clone()
    }
}

impl Default for Surfaces {
    fn default() -> Self {
        let store: Store<Surface> = Store::new();

        let xy_plane = store.reserve();
        store.insert(
            xy_plane.clone(),
            Surface::new(SurfaceGeometry {
                u: GlobalPath::x_axis(),
                v: Vector::unit_y(),
            }),
        );

        let xz_plane = store.reserve();
        store.insert(
            xz_plane.clone(),
            Surface::new(SurfaceGeometry {
                u: GlobalPath::x_axis(),
                v: Vector::unit_z(),
            }),
        );
        let yz_plane = store.reserve();
        store.insert(
            yz_plane.clone(),
            Surface::new(SurfaceGeometry {
                u: GlobalPath::y_axis(),
                v: Vector::unit_z(),
            }),
        );

        Self {
            store,
            xy_plane,
            xz_plane,
            yz_plane,
        }
    }
}

/// Store for [`Vertex`] objects
#[derive(Debug, Default)]
pub struct Vertices {
    store: Store<Vertex>,
}

impl Vertices {
    /// Reserve a slot for an object in the store
    pub fn reserve(&self) -> Handle<Vertex> {
        self.store.reserve()
    }

    /// Insert a [`Vertex`] into the store
    pub fn insert(
        &self,
        handle: Handle<Vertex>,
        vertex: Vertex,
    ) -> Result<(), VertexValidationError> {
        vertex.validate()?;
        self.store.insert(handle, vertex);
        Ok(())
    }
}
