mod point;
mod segment;

use std::fmt::Debug;

use crate::geometry::triangulation::trapezoidation::segment::Segment;

use super::{
    graph::{Graph, X, Y},
    region,
};

// TASK: Update regions after inserting the nodes. The functions to do this
//       should live in another module and be tested separately.
pub fn insert<Region>(segment: Segment, graph: &mut Graph<X, Y, Region>)
where
    Region: Copy + Debug + region::Split,
{
    point::insert(segment.upper(), graph);
    point::insert(segment.lower(), graph);
    segment::insert(segment, graph);
}

#[cfg(test)]
mod tests {
    use crate::geometry::triangulation::trapezoidation::{
        graph::{self, Node, X, Y},
        point::Point,
        region::TestRegion as Region,
        segment::Segment,
    };

    use super::insert;

    type Graph = graph::Graph<X, Y, Region>;

    #[test]
    fn insert_should_insert_upper_point_then_lower_point_then_segment() {
        let mut graph = Graph::new();

        let segment =
            Segment::new(Point::new(0.0, 0.0), Point::new(0.0, 1.0)).unwrap();

        insert(segment, &mut graph);

        let below = match graph.get(graph.source()) {
            Node::Y(Y { point, below, .. }) => {
                assert_eq!(point, &segment.upper());
                *below
            }
            node => panic!("Unexpected node: {:?}", node),
        };

        let above = match graph.get(below) {
            Node::Y(Y { point, above, .. }) => {
                assert_eq!(point, &segment.lower());
                *above
            }
            node => panic!("Unexpected node: {:?}", node),
        };

        match graph.get(above) {
            Node::X(X { segment: s, .. }) => {
                assert_eq!(s, &segment);
            }
            node => panic!("Unexpected node: {:?}", node),
        }
    }
}
