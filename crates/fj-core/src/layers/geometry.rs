//! Layer infrastructure for [`Geometry`]

use crate::{
    geometry::{Geometry, LocalCurveGeom, LocalVertexGeom, SurfaceGeom},
    storage::Handle,
    topology::{Curve, Surface, Vertex},
};

use super::{Command, Event, Layer};

impl Layer<Geometry> {
    /// Define the geometry of the provided curve
    pub fn define_curve(
        &mut self,
        curve: Handle<Curve>,
        surface: Handle<Surface>,
        geometry: LocalCurveGeom,
    ) {
        let mut events = Vec::new();
        self.process(
            DefineCurve {
                curve,
                surface,
                geometry,
            },
            &mut events,
        );
    }

    /// # Define the geometry of the provided surface
    ///
    /// ## Panics
    ///
    /// Panics, if the surface is a special pre-defined plane, like the basis
    /// planes (xy-, xz-, or yz-plane).
    pub fn define_surface(
        &mut self,
        surface: Handle<Surface>,
        geometry: SurfaceGeom,
    ) {
        let mut events = Vec::new();
        self.process(DefineSurface { surface, geometry }, &mut events);
    }

    /// Define the geometry of the provided vertex
    pub fn define_vertex(
        &mut self,
        vertex: Handle<Vertex>,
        curve: Handle<Curve>,
        geometry: LocalVertexGeom,
    ) {
        let mut events = Vec::new();
        self.process(
            DefineVertex {
                vertex,
                curve,
                geometry,
            },
            &mut events,
        );
    }
}

/// Define the geometry of a curve
pub struct DefineCurve {
    curve: Handle<Curve>,
    surface: Handle<Surface>,
    geometry: LocalCurveGeom,
}

impl Command<Geometry> for DefineCurve {
    type Result = ();
    type Event = Self;

    fn decide(
        self,
        _: &Geometry,
        events: &mut Vec<Self::Event>,
    ) -> Self::Result {
        events.push(self);
    }
}

impl Event<Geometry> for DefineCurve {
    fn evolve(&self, state: &mut Geometry) {
        state.define_curve_inner(
            self.curve.clone(),
            self.surface.clone(),
            self.geometry.clone(),
        );
    }
}

/// Define the geometry of a surface
pub struct DefineSurface {
    surface: Handle<Surface>,
    geometry: SurfaceGeom,
}

impl Command<Geometry> for DefineSurface {
    type Result = ();
    type Event = Self;

    fn decide(
        self,
        _: &Geometry,
        events: &mut Vec<Self::Event>,
    ) -> Self::Result {
        events.push(self);
    }
}

impl Event<Geometry> for DefineSurface {
    fn evolve(&self, state: &mut Geometry) {
        state.define_surface_inner(self.surface.clone(), self.geometry);
    }
}

/// Define the geometry of a curve
pub struct DefineVertex {
    vertex: Handle<Vertex>,
    curve: Handle<Curve>,
    geometry: LocalVertexGeom,
}

impl Command<Geometry> for DefineVertex {
    type Result = ();
    type Event = Self;

    fn decide(
        self,
        _: &Geometry,
        events: &mut Vec<Self::Event>,
    ) -> Self::Result {
        events.push(self);
    }
}

impl Event<Geometry> for DefineVertex {
    fn evolve(&self, state: &mut Geometry) {
        state.define_vertex_inner(
            self.vertex.clone(),
            self.curve.clone(),
            self.geometry.clone(),
        );
    }
}
