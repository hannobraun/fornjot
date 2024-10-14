//! Layer infrastructure for [`Geometry`]

use crate::{
    geometry::{
        surfaces::SweptCurve, CurveGeom2, Geometry, LocalCurveGeom,
        LocalVertexGeom, SurfaceGeom,
    },
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

    /// # Define the geometry of the provided curve
    ///
    /// ## Implementation Note
    ///
    /// There currently is an ongoing transition to a new geometry system. This
    /// method defines new-style geometry. Its name is temporary, while the
    /// method defining the old-style geometry is still taking up the more
    /// concise name.
    pub fn define_curve_2(
        &mut self,
        curve: Handle<Curve>,
        geometry: CurveGeom2,
    ) {
        let mut events = Vec::new();
        self.process(DefineCurve2 { curve, geometry }, &mut events);
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
        geometry: SweptCurve,
    ) {
        let mut events = Vec::new();
        self.process(DefineSurface { surface, geometry }, &mut events);
    }

    /// # Define the geometry of the provided surface
    ///
    /// ## Panics
    ///
    /// Panics, if the surface is a special pre-defined plane, like the basis
    /// planes (xy-, xz-, or yz-plane).
    ///
    /// ## Implementation Note
    ///
    /// There currently is an ongoing transition to a new geometry system. This
    /// method defines new-style geometry. Its name is temporary, while the
    /// method defining the old-style geometry is still taking up the more
    /// concise name.
    pub fn define_surface_2(
        &mut self,
        surface: Handle<Surface>,
        geometry: SurfaceGeom,
    ) {
        let mut events = Vec::new();
        self.process(DefineSurface2 { surface, geometry }, &mut events);
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

/// # Define the geometry of a curve
///
/// ## Implementation Note
///
/// There currently is an ongoing transition to a new geometry representation.
/// This type is involved in defining the new-style geometry. Its name is
/// temporary, while the respective type that defines old-style geometry is
/// still taking up the more compact name.
pub struct DefineCurve2 {
    curve: Handle<Curve>,
    geometry: CurveGeom2,
}

impl Command<Geometry> for DefineCurve2 {
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

impl Event<Geometry> for DefineCurve2 {
    fn evolve(&self, state: &mut Geometry) {
        // TASK: This can't work, as designed. I need to clone the geometry
        //       here, but I can't just clone a `Box`.
        state.define_curve_inner_2(self.curve.clone(), self.geometry.clone());
    }
}

/// Define the geometry of a surface
pub struct DefineSurface {
    surface: Handle<Surface>,
    geometry: SweptCurve,
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

/// Define the geometry of a surface
pub struct DefineSurface2 {
    surface: Handle<Surface>,
    geometry: SurfaceGeom,
}

impl Command<Geometry> for DefineSurface2 {
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

impl Event<Geometry> for DefineSurface2 {
    fn evolve(&self, state: &mut Geometry) {
        state.define_surface_inner_2(
            self.surface.clone(),
            self.geometry.clone(),
        );
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
