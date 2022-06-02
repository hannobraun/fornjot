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
    pub(super) vertices: OneMapping<Vertex>,
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

    /// Access the point mapped from the provided point
    ///
    /// # Panics
    ///
    /// Panics, if `object` can not be found in the mapping.
    pub fn point(&self, object: &Handle<Point<3>>) -> Handle<Point<3>> {
        self.points
            .get(object)
            .expect("Could not find point in mapping")
            .clone()
    }

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
    pub fn vertices(&self) -> &OneMapping<Vertex> {
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
