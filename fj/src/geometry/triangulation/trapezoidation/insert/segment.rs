use crate::geometry::triangulation::trapezoidation::{
    find_regions_for_segment::find_regions_for_segment,
    graph::{Graph, Node, X, Y},
    segment::Segment,
};

pub fn insert_segment<Region>(segment: Segment, graph: &mut Graph<X, Y, Region>)
where
    Region: Default,
{
    for region in find_regions_for_segment(&segment, graph) {
        let left = graph.insert_sink(Region::default());
        let right = graph.insert_sink(Region::default());

        graph.replace(
            region,
            Node::X(X {
                segment,
                left,
                right,
            }),
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::triangulation::trapezoidation::{
        graph::{self, Node, X, Y},
        point::Point,
        segment::Segment,
    };

    use super::insert_segment;

    type Graph = graph::Graph<X, Y, Region>;

    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    struct Region(u64);

    #[test]
    fn insert_segment_should_split_the_region_the_segment_goes_through() {
        let mut graph = Graph::new();

        let segment_to_insert =
            Segment::new(Point::new(0.0, 0.0), Point::new(0.0, 1.0)).unwrap();
        insert_segment(segment_to_insert, &mut graph);

        match graph.get(graph.source()) {
            Node::X(X {
                segment,
                left,
                right,
            }) => {
                let left = *left;
                let right = *right;

                // Point must have been inserted.
                assert_eq!(segment, &segment_to_insert);

                // Children should be sinks
                assert_eq!(graph.get(left).is_sink(), true);
                assert_eq!(graph.get(right).is_sink(), true);

                // Children should be distinct
                assert_ne!(left, right);

                // Children should be new nodes
                assert_ne!(graph.source(), left);
                assert_ne!(graph.source(), right);
            }
            node => panic!("Unexpected node: {:?}", node),
        }
    }

    #[test]
    fn insert_segment_should_find_the_right_regions() {
        let mut graph = Graph::new();

        let region_left = Region(1);
        let left = graph.insert_sink(region_left);
        let right = graph.insert_sink(Region(2));

        let node = Node::X(X {
            segment: Segment::new(Point::new(0.0, 0.0), Point::new(0.0, 1.0))
                .unwrap(),
            left,
            right,
        });
        graph.replace(graph.source(), node);

        insert_segment(
            Segment::new(Point::new(1.0, 0.0), Point::new(1.0, 1.0)).unwrap(),
            &mut graph,
        );

        assert_eq!(graph.get(left), &Node::Sink(region_left));
        assert_eq!(graph.get(right).is_x(), true);
    }
}
