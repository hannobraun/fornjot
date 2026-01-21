use fj_math::Vector;

pub trait Curve {
    fn end(&self) -> Vector<3>;
    fn approx(&self) -> Vec<Vector<3>>;
}

pub struct Arc {
    pub end: Vector<3>,
    pub dir: Vector<3>,
}

impl Arc {
    pub fn to(end: impl Into<Vector<3>>, dir: impl Into<Vector<3>>) -> Self {
        Self {
            end: end.into(),
            dir: dir.into(),
        }
    }
}

impl Curve for Arc {
    fn end(&self) -> Vector<3> {
        self.end
    }

    fn approx(&self) -> Vec<Vector<3>> {
        // To approximate the arc, we need the center point and the radius of
        // the circle that is is defined on. We would have both, if we had a
        // `center` vector pointing from the start point of the arc to the
        // center point of the circle.
        //
        // We know two things about this center vector:
        //
        // 1. It is in the same plane that `self.end` and `self.dir` define.
        // 2. It is perpendicular to `self.dir`, which defines the tangent of
        //    the circle at the start point.
        //
        // As a first step, we can create a vector that is perpendicular to
        // `self.dir` and in the right plane, by projecting `self.end` into
        // `self.dir` and subtracting that projection from `self.end`. Let's
        // call the resulting vector `dir_perp`.
        //
        // The center vector we seek must then be a multiple of `dir_perp`:
        //
        // ```
        // center = t * dir_perp (1)
        // ```
        //
        // This gives us an equation with two unknowns, `center` and `t`, which
        // means we need another equation to solve it.
        //
        // And since both start and end are points on the circle, the vectors
        // between them and the center must have the same length:
        //
        // ```
        // |center| = |center - end|
        // |center|² = |center - end|²
        // center * center = (center - end) * (center - end)
        // center * center = center * center - 2 * center * end + end * end
        // 0 = -2 * center * end + end * end
        // 2 * center * end = end * end
        // center * end = (end * end) / 2 (2)
        // ```
        //
        // By substituting `center` from (1) in (2), we get `t`:
        //
        // ```
        // t * dir_perp * end = (end * end) / 2
        // t = (end * end) / (2 * dir_perp * end)
        // ```
        //
        // By putting that back into (1), we have our solution.
        //
        // Until we have implemented the above, here's a placeholder.
        let _ = self.dir;
        Vec::new()
    }
}
