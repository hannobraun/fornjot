//! Convenient API to build objects

use fj_math::Point;

use crate::objects::{Cycle, Face, Surface};

/// API for building a [`Face`]
#[must_use]
pub struct FaceBuilder {
    surface: Surface,
    exterior: Option<Vec<Point<2>>>,
    interiors: Vec<Vec<Point<2>>>,
    color: Option<[u8; 4]>,
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
    pub fn with_exterior_polygon(
        self,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> Self {
        let points = points.into_iter().map(Into::into).collect();

        Self {
            exterior: Some(points),
            ..self
        }
    }

    /// Add an interior polygon to the face
    pub fn with_interior_polygon(
        self,
        points: impl IntoIterator<Item = impl Into<Point<2>>>,
    ) -> Self {
        let points = points.into_iter().map(Into::into).collect();

        let mut interiors = self.interiors;
        interiors.push(points);

        Self { interiors, ..self }
    }

    /// Define the color of the face
    pub fn with_color(mut self, color: [u8; 4]) -> Self {
        self.color = Some(color);
        self
    }

    /// Build the face
    pub fn build(self) -> Face {
        let surface = self.surface;

        let mut exteriors = Vec::new();
        if let Some(points) = self.exterior {
            let cycle = Cycle::polygon_from_points(&self.surface, points);
            exteriors.push(cycle);
        }

        let mut interiors = Vec::new();
        for points in self.interiors {
            let cycle = Cycle::polygon_from_points(&self.surface, points);
            interiors.push(cycle);
        }

        let color = self.color.unwrap_or([255, 0, 0, 255]);

        Face::new(surface, exteriors, interiors, color)
    }
}
