use std::rc::Rc;

use fj_interop::Tolerance;
use fj_math::{Aabb, Point};

use crate::{
    geometry::{SurfaceApprox, SurfaceGeometry},
    handle::Handle,
    topology::{face::Face, surface::Surface},
};

pub trait Flip {
    fn flip(&self) -> Self;
}

impl Flip for Face {
    fn flip(&self) -> Self {
        Face::new(
            Handle::new(self.surface.flip()),
            self.half_edges.clone(),
            self.is_internal,
        )
    }
}

impl Flip for Surface {
    fn flip(&self) -> Self {
        Self {
            geometry: Rc::new(FlippedSurface {
                original: self.geometry.clone(),
            }),
        }
    }
}

#[derive(Debug)]
pub struct FlippedSurface {
    pub original: Rc<dyn SurfaceGeometry>,
}

impl SurfaceGeometry for FlippedSurface {
    fn point_from_local(&self, mut point: Point<2>) -> Point<3> {
        point.u = -point.u;
        self.original.point_from_local(point)
    }

    fn approximate(
        &self,
        boundary: &Aabb<2>,
        tolerance: Tolerance,
    ) -> SurfaceApprox {
        let mut approx = self.original.approximate(boundary, tolerance);

        for point in &mut approx.points {
            point.u = -point.u;
        }

        approx
    }
}
