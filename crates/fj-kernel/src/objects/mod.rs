//! Objects of a shape
//!
//! Objects, in Fornjot parlance, are the elements that make up shapes. An
//! object can be simple and just contain data (like [`GlobalVertex`], for
//! example), or they can be quite complex and refer to other objects (which is
//! actually most of them).
//!
//! # Object Identity vs Object Equality
//!
//! Two objects are *equal*, if they contain the same data. For example, two
//! instances of [`GlobalVertex`] are equal, if they have the same position.
//! This doesn't mean those objects are *identical*. They might have been
//! created by different pieces of code. Or maybe by the same piece of code, but
//! at different times, maybe even based on different inputs.
//!
//! This distinction is relevant, because non-identical objects that are
//! *supposed* to be equal can end up being equal, if they are created based on
//! simple input data (as you might have in a unit test). But they might end up
//! slightly different, if they are created based on complex input data (as you
//! might have in the real world).
//!
//! ## An Example
//!
//! Let's talk about a specific example: two simple curves (straight lines that
//! are coincident with coordinate system axes) which are intersecting at a
//! simple point. Let's say the intersection point sits at the global origin
//! (`[0, 0, 0]`), and its local coordinate on each line also happens to be `0`.
//!
//! If you compute the global coordinates from each of the line-local
//! coordinates, you'll end up with the same result for sure. If we create two
//! [`GlobalVertex`] instances from these global coordinates, any validation
//! code that expects those two instances to be equal, will be happy.
//!
//! But what if the situation is not so simple? Let's say the curves are circles
//! instead of lines, and instead of being all neat, they are at some arbitrary
//! location in space, oriented at weird angles. The local coordinates of their
//! intersection point are not `0`, but different values that are not neatly
//! represented by floating point values.
//!
//! In such a situation, you have an excellent chance of ending up with slightly
//! different global coordinates, if you compute them from each local
//! coordinate. If you're building a [`Cycle`], and this intersection point is
//! where the two curves connect, you could end up with a gap (or self-
//! intersection) in the cycle. If that ends up exported to a triangle mesh,
//! that mesh will be invalid.
//!
//! ## Validation Must Use Identity
//!
//! To prevent such situations, where everything looked fine during development,
//! but you end up with a bug in production, any validation code that compares
//! objects and expects them to be the same, must do that comparison based on
//! identity, not equality. That way, this problem can never happen, because we
//! never expect non-identical objects to be the same.
//!
//! For our example, this would mean we compute *one* [`GlobalVertex`] from
//! *one* of the local coordinates.
//!
//! ## How Identity Works
//!
//! We can exactly determine the identity of an object, thanks to [centralized
//! object storage][`Objects`]. If objects are created at different times,
//! potentially by different code, they end up being stored at different memory
//! locations, regardless of their (non-)equality.
//!
//! If you have two [`Handle`]s, you can compare the identity of the objects
//! they point to using the `id` method.
//!
//! ## Implementation Note
//!
//! As of this writing, most objects are not managed in the centralized object
//! storage. Changing this is an ongoing effort ([#1021]).
//!
//! [`Handle`]: crate::storage::Handle
//! [#1021]: https://github.com/hannobraun/Fornjot/issues/1021

mod curve;
mod cycle;
mod edge;
mod face;
mod shell;
mod sketch;
mod solid;
mod surface;
mod vertex;

pub use self::{
    curve::{Curve, GlobalCurve},
    cycle::Cycle,
    edge::{GlobalEdge, HalfEdge, VerticesInNormalizedOrder},
    face::{Face, FaceSet, Handedness},
    shell::Shell,
    sketch::Sketch,
    solid::Solid,
    surface::Surface,
    vertex::{GlobalVertex, SurfaceVertex, Vertex},
};

use fj_math::Vector;

use crate::{
    path::GlobalPath,
    storage::{Handle, Store},
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
    pub curves: Store<Curve>,

    /// Store for [`Cycle`]s
    pub cycles: Store<Cycle>,

    /// Store for [`Face`]s
    pub faces: Store<Face>,

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

/// Store for [`GlobalCurve`]s
#[derive(Debug, Default)]
pub struct GlobalCurves {
    store: Store<GlobalCurve>,
}

impl GlobalCurves {
    /// Insert a [`GlobalCurve`] into the store
    pub fn insert(&self, global_curve: GlobalCurve) -> Handle<GlobalCurve> {
        self.store.insert(global_curve)
    }
}

/// Store for [`GlobalEdge`]s
#[derive(Debug, Default)]
pub struct GlobalEdges {
    store: Store<GlobalEdge>,
}

impl GlobalEdges {
    /// Insert a [`GlobalEdge`] into the store
    pub fn insert(&self, global_edge: GlobalEdge) -> Handle<GlobalEdge> {
        self.store.insert(global_edge)
    }
}

/// Store for [`GlobalVertex`] objects
#[derive(Debug, Default)]
pub struct GlobalVertices {
    store: Store<GlobalVertex>,
}

impl GlobalVertices {
    /// Insert a [`GlobalVertex`] into the store
    pub fn insert(&self, global_vertex: GlobalVertex) -> Handle<GlobalVertex> {
        self.store.insert(global_vertex)
    }
}

/// Store for [`HalfEdge`]s
#[derive(Debug, Default)]
pub struct HalfEdges {
    store: Store<HalfEdge>,
}

impl HalfEdges {
    /// Insert a [`HalfEdge`] into the store
    pub fn insert(&self, half_edge: HalfEdge) -> Handle<HalfEdge> {
        self.store.insert(half_edge)
    }
}

/// Store for [`Shell`]s
#[derive(Debug, Default)]
pub struct Shells {
    store: Store<Shell>,
}

impl Shells {
    /// Insert a [`Shell`] into the store
    pub fn insert(&self, shell: Shell) -> Handle<Shell> {
        self.store.insert(shell)
    }
}

/// Store for [`Sketch`]es
#[derive(Debug, Default)]
pub struct Sketches {
    store: Store<Sketch>,
}

impl Sketches {
    /// Insert a [`Sketch`] into the store
    pub fn insert(&self, sketch: Sketch) -> Handle<Sketch> {
        self.store.insert(sketch)
    }
}

/// Store for [`Solid`]s
#[derive(Debug, Default)]
pub struct Solids {
    store: Store<Solid>,
}

impl Solids {
    /// Insert a [`Solid`] into the store
    pub fn insert(&self, solid: Solid) -> Handle<Solid> {
        self.store.insert(solid)
    }
}

/// Store for [`SurfaceVertex`] objects
#[derive(Debug, Default)]
pub struct SurfaceVertices {
    store: Store<SurfaceVertex>,
}

impl SurfaceVertices {
    /// Insert a [`SurfaceVertex`] into the store
    pub fn insert(
        &self,
        surface_vertex: SurfaceVertex,
    ) -> Handle<SurfaceVertex> {
        self.store.insert(surface_vertex)
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
    /// Insert a [`Surface`] into the store
    pub fn insert(&self, surface: Surface) -> Handle<Surface> {
        self.store.insert(surface)
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
        let store = Store::new();

        let xy_plane =
            store.insert(Surface::new(GlobalPath::x_axis(), Vector::unit_y()));
        let xz_plane =
            store.insert(Surface::new(GlobalPath::x_axis(), Vector::unit_z()));
        let yz_plane =
            store.insert(Surface::new(GlobalPath::y_axis(), Vector::unit_z()));

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
    /// Insert a [`Vertex`] into the store
    pub fn insert(&self, vertex: Vertex) -> Handle<Vertex> {
        self.store.insert(vertex)
    }
}
