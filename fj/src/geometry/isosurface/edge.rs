use super::GridIndex;

#[derive(Debug, PartialEq)]
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

    pub fn at_surface(&self) -> bool {
        let min = f32::min(self.a.value, self.b.value);
        let max = f32::max(self.a.value, self.b.value);

        min <= 0.0 && max > 0.0
    }
}

#[derive(Debug, PartialEq)]
pub struct Value {
    pub index: GridIndex,
    pub value: f32,
}

#[cfg(test)]
mod tests {
    use super::{Edge, Value};

    #[test]
    fn at_surface_should_detect_whether_edge_is_at_surface() {
        let inside_surface = Edge {
            a: Value {
                index: [0, 0, 0].into(),
                value: -0.2,
            },
            b: Value {
                index: [0, 0, 0].into(),
                value: -0.1,
            },
        };
        assert_eq!(inside_surface.at_surface(), false);
        assert_eq!(inside_surface.reverse().at_surface(), false);

        let outside_surface = Edge {
            a: Value {
                index: [0, 0, 0].into(),
                value: 0.1,
            },
            b: Value {
                index: [0, 0, 0].into(),
                value: 0.2,
            },
        };
        assert_eq!(outside_surface.at_surface(), false);
        assert_eq!(outside_surface.reverse().at_surface(), false);

        let through_surface = Edge {
            a: Value {
                index: [0, 0, 0].into(),
                value: -0.1,
            },
            b: Value {
                index: [0, 0, 0].into(),
                value: 0.1,
            },
        };
        assert_eq!(through_surface.at_surface(), true);
        assert_eq!(through_surface.reverse().at_surface(), true);

        let inside_to_surface = Edge {
            a: Value {
                index: [0, 0, 0].into(),
                value: -0.1,
            },
            b: Value {
                index: [0, 0, 0].into(),
                value: 0.0,
            },
        };
        assert_eq!(inside_to_surface.at_surface(), false);
        assert_eq!(inside_to_surface.reverse().at_surface(), false);

        let outside_to_surface = Edge {
            a: Value {
                index: [0, 0, 0].into(),
                value: 0.0,
            },
            b: Value {
                index: [0, 0, 0].into(),
                value: 0.1,
            },
        };
        assert_eq!(outside_to_surface.at_surface(), true);
        assert_eq!(outside_to_surface.reverse().at_surface(), true);
    }
}
