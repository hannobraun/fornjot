use crate::storage::Handle;

use super::{
    Curve, Cycle, Face, HalfEdge, Region, Shell, Sketch, Solid, Surface, Vertex,
};

/// A trait implemented for all object types
///
/// This trait is implemented for both `T` and `Handle<T>`, where `T` is the
/// type of any bare object. The `BareObject` associated type provides access to
/// the bare object type.
///
/// This is a piece of infrastructure that is useful for other traits, which
/// would otherwise have to duplicate its functionality. Users are unlikely to
/// be affected by this trait.
pub trait IsObject {
    /// The type of the bare object
    type BareObject;
}

impl IsObject for Handle<Curve> {
    type BareObject = Curve;
}

impl IsObject for Handle<Cycle> {
    type BareObject = Cycle;
}

impl IsObject for Handle<Face> {
    type BareObject = Face;
}

impl IsObject for Handle<HalfEdge> {
    type BareObject = HalfEdge;
}

impl IsObject for Handle<Region> {
    type BareObject = Region;
}

impl IsObject for Handle<Shell> {
    type BareObject = Shell;
}

impl IsObject for Handle<Sketch> {
    type BareObject = Sketch;
}

impl IsObject for Handle<Solid> {
    type BareObject = Solid;
}

impl IsObject for Handle<Surface> {
    type BareObject = Surface;
}

impl IsObject for Handle<Vertex> {
    type BareObject = Vertex;
}
