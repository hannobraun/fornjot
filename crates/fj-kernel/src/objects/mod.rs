//! Objects of a shape
//!
//! Objects, in Fornjot parlance, are the elements that make up shapes. An
//! object can be simple and just contain data (like [`GlobalVertex`], for
//! example), or they can be quite complex and refer to other objects.

mod curve;
mod cycle;
mod edge;
mod face;
mod sketch;
mod solid;
mod surface;
mod vertex;

pub use self::{
    curve::{CurveKind, GlobalCurve},
    cycle::Cycle,
    edge::{Edge, VerticesOfEdge},
    face::Face,
    sketch::Sketch,
    solid::Solid,
    surface::{Surface, SweptCurve},
    vertex::{GlobalVertex, Vertex},
};
