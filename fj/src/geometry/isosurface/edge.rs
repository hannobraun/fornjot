use decorum::R32;
use nalgebra::Point;

use super::GridIndex;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Edge {
    pub a: Value,
    pub b: Value,
}

impl Edge {
    pub fn reverse(self) -> Self {
        Self {
            a: self.b,
            b: self.a,
        }
    }

    pub fn swap_values(self) -> Self {
        Self {
            a: Value {
                index: self.a.index,
                point: self.a.point,
                value: self.b.value,
            },
            b: Value {
                index: self.b.index,
                point: self.b.point,
                value: self.a.value,
            },
        }
    }

    pub fn length(&self) -> f32 {
        (self.b.point - self.a.point).magnitude()
    }

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

    pub fn at_surface(&self) -> bool {
        let min = f32::min(self.a.value.into(), self.b.value.into());
        let max = f32::max(self.a.value.into(), self.b.value.into());

        min <= 0.0 && max > 0.0
    }
}

fn signum(v: f32) -> i32 {
    if v == 0.0 {
        0
    } else {
        v.signum() as i32
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Value {
    pub index: GridIndex,
    pub point: Point<f32, 3>,
    pub value: R32,
}

#[derive(Debug)]
pub struct Direction {
    pub axis: Axis,
    pub sign: Sign,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Axis {
    X,
    Y,
    Z,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Sign {
    Neg,
    Pos,
}

#[cfg(test)]
mod tests {
    use super::{Edge, Value};

    #[test]
    fn at_surface_should_detect_whether_edge_is_at_surface() {
        let inside_surface = Edge {
            a: Value {
                index: [0, 0, 0].into(),
                point: [0.0, 0.0, 0.0].into(),
                value: (-0.2).into(),
            },
            b: Value {
                index: [0, 0, 0].into(),
                point: [0.0, 0.0, 0.0].into(),
                value: (-0.1).into(),
            },
        };
        assert_eq!(inside_surface.at_surface(), false);
        assert_eq!(inside_surface.reverse().at_surface(), false);

        let outside_surface = Edge {
            a: Value {
                index: [0, 0, 0].into(),
                point: [0.0, 0.0, 0.0].into(),
                value: 0.1.into(),
            },
            b: Value {
                index: [0, 0, 0].into(),
                point: [0.0, 0.0, 0.0].into(),
                value: 0.2.into(),
            },
        };
        assert_eq!(outside_surface.at_surface(), false);
        assert_eq!(outside_surface.reverse().at_surface(), false);

        let through_surface = Edge {
            a: Value {
                index: [0, 0, 0].into(),
                point: [0.0, 0.0, 0.0].into(),
                value: (-0.1).into(),
            },
            b: Value {
                index: [0, 0, 0].into(),
                point: [0.0, 0.0, 0.0].into(),
                value: 0.1.into(),
            },
        };
        assert_eq!(through_surface.at_surface(), true);
        assert_eq!(through_surface.reverse().at_surface(), true);

        let inside_to_surface = Edge {
            a: Value {
                index: [0, 0, 0].into(),
                point: [0.0, 0.0, 0.0].into(),
                value: (-0.1).into(),
            },
            b: Value {
                index: [0, 0, 0].into(),
                point: [0.0, 0.0, 0.0].into(),
                value: 0.0.into(),
            },
        };
        assert_eq!(inside_to_surface.at_surface(), false);
        assert_eq!(inside_to_surface.reverse().at_surface(), false);

        let outside_to_surface = Edge {
            a: Value {
                index: [0, 0, 0].into(),
                point: [0.0, 0.0, 0.0].into(),
                value: 0.0.into(),
            },
            b: Value {
                index: [0, 0, 0].into(),
                point: [0.0, 0.0, 0.0].into(),
                value: 0.1.into(),
            },
        };
        assert_eq!(outside_to_surface.at_surface(), true);
        assert_eq!(outside_to_surface.reverse().at_surface(), true);
    }
}
