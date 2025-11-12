use std::rc::Rc;

use fj_interop::Tolerance;
use fj_math::{Aabb, Point, Scalar, Vector};

use crate::{
    geometry::{CurveGeometry, Increment, SurfaceApprox, SurfaceGeometry},
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
pub struct FlippedCurve {
    pub original: Rc<dyn CurveGeometry>,
}

impl CurveGeometry for FlippedCurve {
    fn vector_from_local_point(&self, point: Point<1>) -> Vector<3> {
        self.original.vector_from_local_point(-point)
    }

    fn project_vector(&self, vector: Vector<3>) -> Point<1> {
        -self.original.project_vector(vector)
    }

    fn increment_at(
        &self,
        point: Point<1>,
        tolerance: Tolerance,
        size_hint: Scalar,
    ) -> Increment<1> {
        self.original.increment_at(point, tolerance, size_hint)
    }
}

#[derive(Debug)]
pub struct FlippedSurface {
    pub original: Rc<dyn SurfaceGeometry>,
}

impl SurfaceGeometry for FlippedSurface {
    fn point_from_local(&self, mut point: Point<2>) -> Point<3> {
        point.v = -point.v;
        self.original.point_from_local(point)
    }

    fn approximate(
        &self,
        boundary: &Aabb<2>,
        tolerance: Tolerance,
    ) -> SurfaceApprox {
        self.original.approximate(boundary, tolerance)
    }
}
