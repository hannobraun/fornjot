//! Operations to update objects

mod cycle;
mod face;
mod half_edge;
mod region;
mod shell;
mod sketch;
mod solid;

pub use self::{
    cycle::{UpdateCycle, UpdateCycleWithSelector},
    face::{UpdateFace, UpdateFaceWithSelector},
    half_edge::{UpdateHalfEdge, UpdateHalfEdgeWithSelector},
    region::{UpdateRegion, UpdateRegionWithSelector},
    shell::{UpdateShell, UpdateShellWithSelector},
    sketch::{UpdateSketch, UpdateSketchWithSelector},
    solid::{UpdateSolid, UpdateSolidWithSelector},
};
