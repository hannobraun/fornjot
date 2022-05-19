use std::collections::HashMap;

use fj_math::Point;

use crate::{
    geometry::{Curve, Surface},
    topology::{Cycle, Edge, Face, Vertex},
};

use super::Handle;

/// A mapping between objects in different shapes
pub struct Mapping {
    pub(super) points: OneMapping<Point<3>>,
    pub(super) curves: OneMapping<Curve<3>>,
    pub(super) surfaces: OneMapping<Surface>,
    pub(super) vertices: OneMapping<Vertex<3>>,
    pub(super) edges: OneMapping<Edge<3>>,
    pub(super) cycles: OneMapping<Cycle<3>>,
    pub(super) faces: OneMapping<Face>,
}

impl Mapping {
    pub(super) fn new() -> Self {
        Self {
            points: OneMapping::new(),
            curves: OneMapping::new(),
            surfaces: OneMapping::new(),
            vertices: OneMapping::new(),
            edges: OneMapping::new(),
            cycles: OneMapping::new(),
            faces: OneMapping::new(),
        }
    }

    /// Access iterator over the mapped points
    pub fn points(&self) -> &OneMapping<Point<3>> {
        &self.points
    }

    /// Access iterator over the mapped curves
    pub fn curves(&self) -> &OneMapping<Curve<3>> {
        &self.curves
    }

    /// Access iterator over the mapped surfaces
    pub fn surfaces(&self) -> &OneMapping<Surface> {
        &self.surfaces
    }

    /// Access iterator over the mapped vertices
    pub fn vertices(&self) -> &OneMapping<Vertex<3>> {
        &self.vertices
    }

    /// Access iterator over the mapped edges
    pub fn edges(&self) -> &OneMapping<Edge<3>> {
        &self.edges
    }

    /// Access iterator over the mapped cycles
    pub fn cycles(&self) -> &OneMapping<Cycle<3>> {
        &self.cycles
    }

    /// Access iterator over the mapped faces
    pub fn faces(&self) -> &OneMapping<Face> {
        &self.faces
    }
}

pub type OneMapping<T> = HashMap<Handle<T>, Handle<T>>;
