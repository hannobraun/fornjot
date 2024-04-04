use crate::storage::{Handle, Store};

use super::{
    Curve, Cycle, Face, HalfEdge, Region, Shell, Sketch, Solid, Surface, Vertex,
};

/// The stores for all topological objects
#[derive(Debug, Default)]
pub struct Topology {
    /// Store for [`Curve`]s
    pub curves: Store<Curve>,

    /// Store for [`Cycle`]s
    pub cycles: Store<Cycle>,

    /// Store for [`Face`]s
    pub faces: Store<Face>,

    /// Store for [`HalfEdge`]s
    pub half_edges: Store<HalfEdge>,

    /// Store for [`Region`]s
    pub regions: Store<Region>,

    /// Store for [`Shell`]s
    pub shells: Store<Shell>,

    /// Store for [`Sketch`]es
    pub sketches: Store<Sketch>,

    /// Store for [`Solid`]s
    pub solids: Store<Solid>,

    /// Store for [`Surface`]s
    pub surfaces: Surfaces,

    /// Store for [`Vertex`] objects
    pub vertices: Store<Vertex>,
}

impl Topology {
    /// Construct a new instance of `Stores`
    pub fn new() -> Self {
        Self::default()
    }
}

/// Store for [`Surface`]s
#[derive(Debug)]
pub struct Surfaces {
    store: Store<Surface>,

    space_2d: Handle<Surface>,

    xy_plane: Handle<Surface>,
    xz_plane: Handle<Surface>,
    yz_plane: Handle<Surface>,
}

impl Surfaces {
    /// Reserve a slot for an object in the store
    pub fn reserve(&self) -> Handle<Surface> {
        self.store.reserve()
    }

    /// Insert an object into the store
    pub fn insert(&mut self, handle: Handle<Surface>, surface: Surface) {
        self.store.insert(handle, surface);
    }

    /// Access the surface representing 2D space
    ///
    /// Every other surface is a 2D subspace within a 3D space. This surface is
    /// special, in that it represents the 2D space which is not located in a 3D
    /// space.
    pub fn space_2d(&self) -> Handle<Surface> {
        self.space_2d.clone()
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
        let mut store: Store<Surface> = Store::new();

        let space_2d = store.reserve();
        store.insert(space_2d.clone(), Surface::new());

        let xy_plane = store.reserve();
        store.insert(xy_plane.clone(), Surface::new());

        let xz_plane = store.reserve();
        store.insert(xz_plane.clone(), Surface::new());

        let yz_plane = store.reserve();
        store.insert(yz_plane.clone(), Surface::new());

        Self {
            store,
            space_2d,
            xy_plane,
            xz_plane,
            yz_plane,
        }
    }
}
