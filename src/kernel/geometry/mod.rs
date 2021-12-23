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
///
/// This distinction is not observed here, but moving things into that direction
/// is the intention.
pub enum Curve {
    /// A circle
    ///
    /// This representation is not optimal, for two reasons:
    /// - It doesn't define the center point of the circle. For that reason,
    ///   only circles centered on the origin are supported at this point.
    /// - It doesn't define where the circle begins. For the purposes of
    ///   defining an arc on the circle, the zero angle will implicitly be to
    ///   the right.
    ///
    /// It might be better to define a circle using two points: The center, and
    /// the "zero" point on the circumference.
    Circle {
        /// The radius of the circle
        radius: f64,
    },

    /// A line, defined by two points
    Line {
        /// One point defining the line
        a: Point,

        /// The other point defining the line
        b: Point,
    },
}
