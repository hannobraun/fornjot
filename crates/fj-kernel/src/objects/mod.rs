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

use crate::storage::Store;

pub use self::{
    curve::{Curve, GlobalCurve},
    cycle::Cycle,
    edge::{GlobalEdge, HalfEdge, VerticesInNormalizedOrder},
    face::{Face, Faces, Handedness},
    shell::Shell,
    sketch::Sketch,
    solid::Solid,
    surface::Surface,
    vertex::{GlobalVertex, SurfaceVertex, Vertex},
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
    /// Store for curves
    pub curves: Store<Curve>,

    /// Store for global curves
    pub global_curves: Store<GlobalCurve>,

    /// Store for global vertices
    pub global_vertices: Store<GlobalVertex>,

    /// Store for surfaces
    pub surfaces: Store<Surface>,
}

impl Objects {
    /// Construct a new instance of `Stores`
    pub fn new() -> Self {
        Self::default()
    }
}
