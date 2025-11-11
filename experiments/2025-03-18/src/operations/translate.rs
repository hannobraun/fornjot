use std::rc::Rc;

use fj_interop::Tolerance;
use fj_math::{Aabb, Point, Vector};

use crate::{
    geometry::{SurfaceApprox, SurfaceGeometry},
    handle::Handle,
    topology::{
        curve::Curve, face::Face, half_edge::HalfEdge, surface::Surface,
        vertex::Vertex,
    },
};

pub trait Translate {
    fn translate(&self, offset: impl Into<Vector<3>>) -> Self;
}

impl Translate for Curve {
    fn translate(&self, offset: impl Into<Vector<3>>) -> Self {
        let geometry = self.geometry.translate(offset);
        Curve { geometry }
    }
}

impl Translate for Face {
    fn translate(&self, offset: impl Into<Vector<3>>) -> Self {
        let offset = offset.into();

        Face::new(
            Handle::new(self.surface.translate(offset)),
            self.half_edges
                .iter()
                .map(|half_edge| Handle::new(half_edge.translate(offset))),
            self.is_internal,
        )
    }
}

impl Translate for HalfEdge {
    fn translate(&self, offset: impl Into<Vector<3>>) -> Self {
        let offset = offset.into();

        let curve = self.curve.translate(offset);
        let start = self.start.translate(offset);

        HalfEdge {
            curve: Handle::new(curve),
            start: Handle::new(start),
            is_internal: self.is_internal,
        }
    }
}

impl Translate for Surface {
    fn translate(&self, offset: impl Into<Vector<3>>) -> Self {
        let geometry = Rc::new(TranslatedSurface {
            original: self.geometry.clone(),
            offset: offset.into(),
        });
        Self { geometry }
    }
}

impl Translate for Vertex {
    fn translate(&self, offset: impl Into<Vector<3>>) -> Self {
        let offset = offset.into();
        Vertex::new(self.point + offset)
    }
}

#[derive(Debug)]
pub struct TranslatedSurface {
    pub original: Rc<dyn SurfaceGeometry>,
    pub offset: Vector<3>,
}

impl SurfaceGeometry for TranslatedSurface {
    fn point_from_local(&self, point: Point<2>) -> Point<3> {
        let point = self.original.point_from_local(point);
        point + self.offset
    }

    fn approximate(
        &self,
        boundary: &Aabb<2>,
        tolerance: Tolerance,
    ) -> SurfaceApprox {
        self.original.approximate(boundary, tolerance)
    }
}
