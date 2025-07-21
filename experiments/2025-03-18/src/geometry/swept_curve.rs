use fj_math::{Point, Vector};

use super::{AnchoredCurve, Line, curve::FloatingCurve};

#[derive(Debug)]
pub struct SweptCurve {
    pub u: AnchoredCurve,
    pub v: FloatingCurve,
}

impl SweptCurve {
    pub fn plane_from_coord_system(
        origin: impl Into<Point<3>>,
        axes: [impl Into<Vector<3>>; 2],
    ) -> Self {
        let origin = origin.into();
        let [u, v] = axes.map(Into::into).map(|direction| Line { direction });

        Self {
            u: AnchoredCurve::from_origin_and_curve(origin, u),
            v: FloatingCurve::new(v),
        }
    }

    pub fn point_from_local(&self, point: impl Into<Point<2>>) -> Point<3> {
        let [u, v] = point.into().coords.components;
        self.u.point_from_local([u]) + self.v.vector_from_local_point([v])
    }

    pub fn flip(&self) -> Self {
        Self {
            u: self.u.clone(),
            v: self.v.flip(),
        }
    }

    pub fn translate(&self, offset: impl Into<Vector<3>>) -> Self {
        Self {
            u: self.u.translate(offset),
            v: self.v.clone(),
        }
    }
}
