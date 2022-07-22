//! Convenient API to build objects

use fj_interop::mesh::Color;

use crate::objects::{Cycle, Face, Surface};

/// API for building a [`Face`]
#[must_use]
pub struct FaceBuilder {
    surface: Surface,
    exterior: Option<Cycle>,
    interiors: Vec<Cycle>,
    color: Option<Color>,
}

impl FaceBuilder {
    /// Construct a new instance of `FaceBuilder`
    pub fn new(surface: Surface) -> Self {
        Self {
            surface,
            exterior: None,
            interiors: Vec::new(),
            color: None,
        }
    }

    /// Make the exterior or the face a polygon
    pub fn with_exterior(self, cycle: Cycle) -> Self {
        Self {
            exterior: Some(cycle),
            ..self
        }
    }

    /// Add an interior polygon to the face
    pub fn with_interior(self, cycle: Cycle) -> Self {
        let mut interiors = self.interiors;
        interiors.push(cycle);

        Self { interiors, ..self }
    }

    /// Define the color of the face
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Build the face
    pub fn build(self) -> Face {
        let surface = self.surface;

        let mut exteriors = Vec::new();
        if let Some(cycle) = self.exterior {
            exteriors.push(cycle);
        }

        let interiors = self.interiors;

        let color = self.color.unwrap_or_default();

        Face::new(surface)
            .with_exteriors(exteriors)
            .with_interiors(interiors)
            .with_color(color)
    }
}
