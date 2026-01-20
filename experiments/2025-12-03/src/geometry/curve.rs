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
        // This is a placeholder for the actual approximation of the arc that
        // still needs to happen.
        let _ = self.dir;
        Vec::new()

        // Okay, now on to the real approximation. If we had the center and
        // radius of the circle that the arc was on, we could do that easily.
        // Both could be defined by a `center` vector that points from the start
        // to the center.
        //
        // We know that the center point must be in the plane that `self.end`
        // and `self.dir` define. Or in other words, it must be a linear
        // combination of both:
        //
        // ```
        // center = a * end + b * dir (1)
        // ```
        //
        // This gives us an equation with two unknowns. Two more, and we should
        // have a system that we can solve.
        //
        // `self.dir` is the direction of the arc at the start point. This is a
        // tangent of the circle, meaning it and the `center` vector must be
        // orthogonal:
        //
        // ```
        // center * dir = 0 (2)
        // ```
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
        // center * end = (end * end) / 2 (3)
        // ```
        //
        // This gives us a system of three equations that we can hopefully
        // solve.
    }
}
