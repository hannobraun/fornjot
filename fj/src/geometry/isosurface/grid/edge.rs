use std::fmt;

use super::Value;

/// An edge of a grid cell in an isosurface extraction grid
#[derive(Clone, Copy, PartialEq)]
pub struct Edge {
    /// The value at the origin of the edge, i.e. the point the edge points from
    pub a: Value,

    /// The value at the point the edge points to
    pub b: Value,
}

impl Edge {
    /// Reverse the edge
    ///
    /// Returns an edge that has the `a` and `b` fields swapped.
    pub fn reverse(self) -> Self {
        Self {
            a: self.b,
            b: self.a,
        }
    }

    /// Swap the distance values of the edge points
    ///
    /// Returns an edge that has the `distance` field of the two `Value`s
    /// swapped, leaving the other data as-is.
    pub fn swap_distance_values(self) -> Self {
        Self {
            a: Value {
                index: self.a.index,
                point: self.a.point,
                distance: self.b.distance,
            },
            b: Value {
                index: self.b.index,
                point: self.b.point,
                distance: self.a.distance,
            },
        }
    }

    /// Compute the length of the edge
    pub fn length(&self) -> f32 {
        let a = self.a.point;
        let b = self.b.point;

        (b - a).magnitude()
    }

    /// Compute the direction of the edge
    pub fn direction(&self) -> Direction {
        let a = self.a.point;
        let b = self.b.point;

        let direction =
            [signum(b.x - a.x), signum(b.y - a.y), signum(b.z - a.z)];

        #[rustfmt::skip]
        let (axis, sign) = match direction {
            [ 0,  0, -1] => (Axis::Z, Sign::Neg),
            [ 0,  0,  1] => (Axis::Z, Sign::Pos),
            [ 0, -1,  0] => (Axis::Y, Sign::Neg),
            [ 0,  1,  0] => (Axis::Y, Sign::Pos),
            [-1,  0,  0] => (Axis::X, Sign::Neg),
            [ 1,  0,  0] => (Axis::X, Sign::Pos),

            direction => panic!(
                "Invalid direction ({:?}).\
                Only axis-aligned directions allowed.",
                direction
            ),
        };

        Direction { axis, sign }
    }

    /// Determine whether the edge is at a surface
    ///
    /// An edge is at the surface if it crosses the surface, or it touches the
    /// surface from the outside.
    pub fn at_surface(&self) -> bool {
        let min = f32::min(self.a.distance, self.b.distance);
        let max = f32::max(self.a.distance, self.b.distance);

        min <= 0.0 && max > 0.0
    }
}

impl fmt::Debug for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} => {:?}", self.a, self.b)
    }
}

fn signum(v: f32) -> i32 {
    if v == 0.0 {
        0
    } else {
        v.signum() as i32
    }
}

/// The direction of an edge
#[derive(Debug)]
pub struct Direction {
    /// The axis along which the edge is aligned
    pub axis: Axis,

    /// The alignment of the edge along its axis
    pub sign: Sign,
}

/// An axis in three-dimensional space
#[derive(Debug, Eq, PartialEq)]
pub enum Axis {
    X,
    Y,
    Z,
}

/// The alignment of an edge along an axis
#[derive(Debug, Eq, PartialEq)]
pub enum Sign {
    /// The edge points into the negative direction
    Neg,

    /// The edge points into the positive direction
    Pos,
}

#[cfg(test)]
mod tests {
    use crate::geometry::isosurface::grid;

    use super::Edge;

    #[test]
    fn at_surface_should_detect_whether_edge_is_at_surface() {
        fn value_at(distance: f32) -> grid::Value {
            // Dummy values that the code under test doesn't care about.
            let index = [0, 0, 0].into();
            let point = [0.0, 0.0, 0.0].into();

            grid::Value {
                index,
                point,
                distance,
            }
        }

        let inside_surface = Edge {
            a: value_at(-0.2),
            b: value_at(-0.1),
        };
        assert_eq!(inside_surface.at_surface(), false);
        assert_eq!(inside_surface.reverse().at_surface(), false);

        let outside_surface = Edge {
            a: value_at(0.1),
            b: value_at(0.2),
        };
        assert_eq!(outside_surface.at_surface(), false);
        assert_eq!(outside_surface.reverse().at_surface(), false);

        let through_surface = Edge {
            a: value_at(-0.1),
            b: value_at(0.1),
        };
        assert_eq!(through_surface.at_surface(), true);
        assert_eq!(through_surface.reverse().at_surface(), true);

        let inside_to_surface = Edge {
            a: value_at(-0.1),
            b: value_at(0.0),
        };
        assert_eq!(inside_to_surface.at_surface(), false);
        assert_eq!(inside_to_surface.reverse().at_surface(), false);

        let outside_to_surface = Edge {
            a: value_at(0.0),
            b: value_at(0.1),
        };
        assert_eq!(outside_to_surface.at_surface(), true);
        assert_eq!(outside_to_surface.reverse().at_surface(), true);
    }
}
