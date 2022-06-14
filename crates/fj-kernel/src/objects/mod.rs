//! Objects of a shape
//!
//! Objects, in Fornjot parlance, are the elements that make up shapes. An
//! object can be simple and just contain data (like, for example, [`Vertex`]),
//! or they can be quite complex and refer to other objects.

mod curve;
mod cycle;
mod edge;
mod face;
mod global_vertex;
mod surface;
mod vertex;

pub use self::{
    curve::Curve,
    cycle::Cycle,
    edge::{Edge, VerticesOfEdge},
    face::{CyclesInFace, Face, FaceBRep},
    global_vertex::GlobalVertex,
    surface::{Surface, SweptCurve},
    vertex::Vertex,
};
