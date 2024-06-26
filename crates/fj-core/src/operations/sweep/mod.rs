//! Sweep objects along a path to create new objects
//!
//! Sweeps 1D or 2D objects along a straight path, creating a 2D or 3D object,
//! respectively.

mod cycle;
mod face;
mod half_edge;
mod path;
mod region;
mod shell_face;
mod sketch;
mod vertex;

pub use self::{
    cycle::{SweepCycle, SweptCycle},
    face::SweepFace,
    half_edge::{SweepHalfEdge, SweptHalfEdge},
    path::SweepSurfacePath,
    region::{SweepRegion, SweptRegion},
    shell_face::{ShellExtendedBySweep, SweepFaceOfShell},
    sketch::SweepSketch,
    vertex::SweepVertex,
};

use std::collections::BTreeMap;

use crate::{
    storage::{Handle, ObjectId},
    topology::{Curve, Vertex},
};

/// A cache used for sweeping
#[derive(Default)]
pub struct SweepCache {
    /// Cache for curves
    pub curves: BTreeMap<ObjectId, Handle<Curve>>,

    /// Cache for vertices
    pub vertices: BTreeMap<ObjectId, Handle<Vertex>>,
}
