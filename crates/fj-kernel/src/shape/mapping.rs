use std::collections::HashMap;

use crate::objects::{Curve, Cycle, Edge, Face, Surface, Vertex};

use super::Handle;

/// A mapping between objects in different shapes
pub struct Mapping {
    pub(super) curves: OneMapping<Curve<3>>,
    pub(super) surfaces: OneMapping<Surface>,
    pub(super) vertices: OneMapping<Vertex>,
    pub(super) edges: OneMapping<Edge<3>>,
    pub(super) cycles: OneMapping<Cycle<3>>,
    pub(super) faces: OneMapping<Face>,
}

impl Mapping {
    /// Access the curve mapped from the provided curve
    ///
    /// # Panics
    ///
    /// Panics, if `object` can not be found in the mapping.
    pub fn curve(&self, object: &Handle<Curve<3>>) -> Handle<Curve<3>> {
        self.curves
            .get(object)
            .expect("Could not find curve in mapping")
            .clone()
    }

    /// Access the surface mapped from the provided surface
    ///
    /// # Panics
    ///
    /// Panics, if `object` can not be found in the mapping.
    pub fn surface(&self, object: &Handle<Surface>) -> Handle<Surface> {
        self.surfaces
            .get(object)
            .expect("Could not find surface in mapping")
            .clone()
    }

    /// Access the vertex mapped from the provided vertex
    ///
    /// # Panics
    ///
    /// Panics, if `object` can not be found in the mapping.
    pub fn vertex(&self, object: &Handle<Vertex>) -> Handle<Vertex> {
        self.vertices
            .get(object)
            .expect("Could not find vertex in mapping")
            .clone()
    }

    /// Access the edge mapped from the provided edge
    ///
    /// # Panics
    ///
    /// Panics, if `object` can not be found in the mapping.
    pub fn edge(&self, object: &Handle<Edge<3>>) -> Handle<Edge<3>> {
        self.edges
            .get(object)
            .expect("Could not find edge in mapping")
            .clone()
    }

    /// Access the cycle mapped from the provided cycle
    ///
    /// # Panics
    ///
    /// Panics, if `object` can not be found in the mapping.
    pub fn cycle(&self, object: &Handle<Cycle<3>>) -> Handle<Cycle<3>> {
        self.cycles
            .get(object)
            .expect("Could not find vertex in mapping")
            .clone()
    }

    /// Access the face mapped from the provided face
    ///
    /// # Panics
    ///
    /// Panics, if `object` can not be found in the mapping.
    pub fn face(&self, object: &Handle<Face>) -> Handle<Face> {
        self.faces
            .get(object)
            .expect("Could not find face in mapping")
            .clone()
    }
}

type OneMapping<T> = HashMap<Handle<T>, Handle<T>>;
