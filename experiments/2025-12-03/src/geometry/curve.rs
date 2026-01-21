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
        // We know that this `center` vector must be coplanar with `self.end`
        // and `self.dir`, and perpendicular to `self.dir`. We can start by
        // computing a vector that fulfills these requirements.
        let dir_perp = self.end - self.end.vector_projecting_onto(&self.dir);
        dbg!(dir_perp);

        // `dir_perp` is colinear with the `center` vector we seek:
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
