use fj_interop::mesh::Color;

use crate::{
    insert::Insert,
    objects::{Face, Objects, Surface},
    services::Service,
    storage::Handle,
};

use super::CycleBuilder;

/// Builder API for [`Face`]
pub struct FaceBuilder {
    surface: Handle<Surface>,
    exterior: CycleBuilder,
    interiors: Vec<CycleBuilder>,
    color: Option<Color>,
}
impl FaceBuilder {
    /// Create an instance of `FaceBuilder`
    pub fn new(surface: Handle<Surface>) -> Self {
        Self {
            surface,
            exterior: CycleBuilder::new(),
            interiors: Vec::new(),
            color: None,
        }
    }

    /// Replace the face's exterior cycle
    pub fn with_exterior(mut self, exterior: CycleBuilder) -> Self {
        self.exterior = exterior;
        self
    }

    /// Add an interior cycle to the face
    pub fn with_interior(mut self, interior: CycleBuilder) -> Self {
        self.interiors.push(interior);
        self
    }

    /// Define the color of the face
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Build the face
    pub fn build(self, objects: &mut Service<Objects>) -> Face {
        let exterior = self.exterior.build(objects).insert(objects);
        let interiors = self
            .interiors
            .into_iter()
            .map(|cycle| cycle.build(objects).insert(objects));

        Face::new(self.surface, exterior, interiors, self.color)
    }
}
