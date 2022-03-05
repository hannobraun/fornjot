use crate::{
    kernel::{
        geometry::{Circle, Curve, Line},
        topology::{
            edges::{Cycle, Edge},
            vertices::Vertex,
        },
    },
    math::{Point, Vector},
};

/// The edges of a shape
#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Edges {
    /// The cycles that the edges of the shape form
    ///
    /// Code reading this field generally assumes that cycles do not overlap.
    /// This precondition is currently not checked, and must be upheld by all
    /// code writing to this field.
    pub cycles: Vec<Cycle>,
}

impl Edges {
    /// Construct a new instance of `Edges`, with a single cycle
    pub fn single_cycle(edges: impl IntoIterator<Item = Edge>) -> Self {
        let cycle = Cycle {
            edges: edges.into_iter().collect(),
        };

        Self {
            cycles: vec![cycle],
        }
    }

    /// Construct an edge
    ///
    /// If vertices are provided in `vertices`, they must be on `curve`.
    ///
    /// This constructor will convert the vertices into curve coordinates. If
    /// they are not on the curve, this will result in their projection being
    /// converted into curve coordinates, which is likely not the caller's
    /// intention.
    pub fn create(
        &mut self,
        curve: Curve,
        vertices: Option<[Vertex; 2]>,
    ) -> Edge {
        Edge { curve, vertices }
    }

    /// Construct an edge that is a line segment
    pub fn create_line_segment(&mut self, vertices: [Vertex; 2]) -> Edge {
        self.create(
            Curve::Line(Line::from_points(
                vertices.clone().map(|vertex| vertex.point()),
            )),
            Some(vertices),
        )
    }

    /// Create a circle
    pub fn create_circle(&mut self, radius: f64) -> Edge {
        self.create(
            Curve::Circle(Circle {
                center: Point::origin(),
                radius: Vector::from([radius, 0.]),
            }),
            None,
        )
    }
}
