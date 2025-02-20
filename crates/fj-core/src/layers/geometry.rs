//! Layer infrastructure for [`Geometry`]

use crate::{
    geometry::{
        CurveGeom2, Geometry, LocalCurveGeom, LocalVertexGeom, SurfaceGeom,
        surfaces::SweptCurve,
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
        self.process_command(DefineCurve {
            curve,
            surface,
            geometry,
        });
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
        self.process_command(DefineCurve2 { curve, geometry });
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
        self.process_command(DefineSurface { surface, geometry });
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
        self.process_command(DefineSurface2 { surface, geometry });
    }

    /// Define the geometry of the provided vertex
    pub fn define_vertex(
        &mut self,
        vertex: Handle<Vertex>,
        curve: Handle<Curve>,
        geometry: LocalVertexGeom,
    ) {
        self.process_command(DefineVertex {
            vertex,
            curve,
            geometry,
        });
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
    fn evolve(self, state: &mut Geometry) {
        state.define_curve_inner(self.curve, self.surface, self.geometry);
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
    fn evolve(self, state: &mut Geometry) {
        state.define_curve_inner_2(self.curve, self.geometry);
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
    fn evolve(self, state: &mut Geometry) {
        state.define_surface_inner(self.surface, self.geometry);
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
    fn evolve(self, state: &mut Geometry) {
        state.define_surface_inner_2(self.surface, self.geometry);
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
    fn evolve(self, state: &mut Geometry) {
        state.define_vertex_inner(self.vertex, self.curve, self.geometry);
    }
}
