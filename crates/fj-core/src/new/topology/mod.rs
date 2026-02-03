//! # Topological primitives to represent shapes
//!
//! See [`Topology`], which is the main entry point to this module.

mod objects;
mod store;

pub use self::{
    objects::{Face, HalfEdge, Solid, Vertex},
    store::{Index, Store},
};

/// # Stores for the topological primitives
///
/// Contains stores for all topological primitives. Creating an instances of
/// this struct would typically be the first thing you would do, when using
/// Fornjot.
///
/// ```
/// let topology = Topology::new();
/// // call code that operates on the topological objects here
/// ```
///
/// Though nothing prevents you from creating multiple instances of this struct
/// (or not creating any at all, as you can create the [`Store`]s it contains
/// separately), Fornjot has been designed with the assumptions that one
/// instance of `Topology` (or more accurately, one set of `Store`s) exists.
///
/// You may create multiple `Topology` instances, to keep various shapes
/// completely isolated from each other. Though the implications of that are
/// currently unexplored. (There may be differences in performance.)
///
/// If you mix multiple `Topology` instances though, you are inviting trouble.
/// See the documentation [`Store`] for more details on that.
#[derive(Default)]
pub struct Topology {
    pub faces: Store<Face>,
    pub half_edges: Store<HalfEdge>,
    pub solids: Store<Solid>,
    pub vertices: Store<Vertex>,
}

impl Topology {
    pub fn new() -> Self {
        Self::default()
    }
}
