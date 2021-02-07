use crate::geometry::triangulation::trapezoidation::{
    find_regions_for_segment::find_regions_for_segment,
    graph::{Graph, Node, X, Y},
    region,
    segment::Segment,
};

pub fn insert<Region>(
    segment: Segment,
    graph: &mut Graph<X, Y, Region>,
) -> Vec<X>
where
    Region: region::Source,
{
    let mut inserted_nodes = Vec::new();

    for region in find_regions_for_segment(&segment, graph) {
        // TASK: Split existing region instead of creating new ones.
        let left = graph.insert_sink(Region::source());
        let right = graph.insert_sink(Region::source());

        let node = X {
            segment,
            left,
            right,
        };

        graph.replace(region, Node::X(node));
        inserted_nodes.push(node);
    }

    inserted_nodes
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
    fn insert_should_split_the_region_the_segment_goes_through() {
        let mut graph = Graph::new();

        let segment_to_insert =
            Segment::new(Point::new(0.0, 0.0), Point::new(0.0, 1.0)).unwrap();
        insert(segment_to_insert, &mut graph);

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
                graph.get(left).unwrap_sink();
                graph.get(right).unwrap_sink();

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
    fn insert_should_return_inserted_node() {
        let mut graph = Graph::new();

        let nodes = insert(
            Segment::new(Point::new(0.0, 0.0), Point::new(0.0, 1.0)).unwrap(),
            &mut graph,
        );
        let nodes: Vec<_> =
            nodes.into_iter().map(|node| Node::X(node)).collect();

        assert_eq!(vec![*graph.get(graph.source())], nodes);
    }

    #[test]
    fn insert_should_find_the_right_regions() {
        let mut graph = Graph::new();

        let region_left = Region::new(1);
        let left = graph.insert_sink(region_left);
        let right = graph.insert_sink(Region::new(2));

        let node = Node::X(X {
            segment: Segment::new(Point::new(0.0, 0.0), Point::new(0.0, 1.0))
                .unwrap(),
            left,
            right,
        });
        graph.replace(graph.source(), node);

        insert(
            Segment::new(Point::new(1.0, 0.0), Point::new(1.0, 1.0)).unwrap(),
            &mut graph,
        );

        assert_eq!(graph.get(left), &Node::Sink(region_left));
        graph.get(right).unwrap_x();
    }
}
