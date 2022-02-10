mod circle;
mod line;

use parry3d_f64::math::Isometry;

pub use self::{circle::Circle, line::Line};

use crate::math::Point;

/// A one-dimensional shape
///
/// The word "curve" is used as an umbrella term for all one-dimensional shapes,
/// and doesn't imply that those shapes need to be curved. Straight lines are
/// included.
///
/// The nomenclature is inspired by Boundary Representation Modelling Techniques
/// by Ian Stroud. "Curve" refers to unbounded one-dimensional geometry, while
/// while edges are bounded portions of curves.
#[derive(Clone, Debug)]
pub enum Curve {
    /// A circle
    Circle(Circle),

    /// A line
    Line(Line),

    /// A mock curve used for testing
    #[cfg(test)]
    Mock { approx: Vec<Point<3>> },
}

impl Curve {
    #[must_use]
    pub fn transform(self, transform: &Isometry<f64>) -> Self {
        match self {
            Self::Circle(circle) => Self::Circle(circle.transform(transform)),
            Self::Line(line) => Self::Line(line.transform(transform)),

            #[cfg(test)]
            Self::Mock { .. } => todo!(),
        }
    }

    /// Compute an approximation of the curve
    ///
    /// `tolerance` defines how far the approximation is allowed to deviate from
    /// the actual edge.
    ///
    /// # Implementation Note
    ///
    /// This only works as it is, because edges are severely limited and don't
    /// define which section of the curve they inhabit. Once they do that, we
    /// need an `approximate_between(a, b)` method instead, where `a` and `b`
    /// are the vertices that bound the edge on the curve.
    ///
    /// The `approx` methods of the curves then need to make sure to return
    /// those exact vertices as part of the approximation, and not accidentally
    /// compute some almost but not quite identical points for those vertices
    /// instead.
    pub fn approx(&self, tolerance: f64, out: &mut Vec<Point<3>>) {
        match self {
            Self::Circle(circle) => circle.approx(tolerance, out),
            Self::Line(Line { a, b }) => out.extend([*a, *b]),

            #[cfg(test)]
            Self::Mock { approx } => out.extend(approx),
        }
    }
}
