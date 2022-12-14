//! API for building objects

// These are new-style builders that build on top of the partial object API.
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
    curve::CurveBuilder,
    cycle::CycleBuilder,
    edge::{GlobalEdgeBuilder, HalfEdgeBuilder},
    face::FaceBuilder,
    shell::ShellBuilder,
    sketch::SketchBuilder,
    solid::SolidBuilder,
    surface::SurfaceBuilder,
    vertex::{GlobalVertexBuilder, SurfaceVertexBuilder, VertexBuilder},
};
