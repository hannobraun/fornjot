use fj_math::{Line, Point};

use crate::geometry::{path::GlobalPath, surface::SurfaceGeometry};

/// A two-dimensional shape
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Surface {
    geometry: SurfaceGeometry,
}

impl Surface {
    /// Construct a `Surface` from two paths that define its coordinate system
    pub fn new(geometry: SurfaceGeometry) -> Self {
        Self { geometry }
    }

    /// Construct a plane from 3 points
    pub fn plane_from_points(points: [impl Into<Point<3>>; 3]) -> Self {
        let [a, b, c] = points.map(Into::into);

        let u = GlobalPath::Line(Line::from_points([a, b]));
        let v = c - a;

        Self {
            geometry: SurfaceGeometry { u, v },
        }
    }

    /// Access the surface's geometry
    pub fn geometry(&self) -> SurfaceGeometry {
        self.geometry
    }
}
