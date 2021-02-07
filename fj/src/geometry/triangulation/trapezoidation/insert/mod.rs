use crate::geometry::triangulation::trapezoidation::segment::Segment;

use super::{
    graph::{Graph, X, Y},
    insert_point::insert_point,
    insert_segment::insert_segment,
};

pub fn insert<Region>(segment: Segment, graph: &mut Graph<X, Y, Region>)
where
    Region: Default,
{
    insert_point(segment.upper(), graph);
    insert_point(segment.lower(), graph);
    insert_segment(segment, graph);
}

#[cfg(test)]
mod tests {
    use crate::geometry::triangulation::trapezoidation::{
        graph::{self, Node, X, Y},
        point::Point,
        segment::Segment,
    };

    use super::insert;

    type Graph = graph::Graph<X, Y, Region>;

    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    struct Region(u64);

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
