use std::fmt::Debug;

use crate::geometry::triangulation::seidel::trapezoidation::{
    find_regions_for_segment::find_regions_for_segment,
    graph::{Graph, Node, X, Y},
    ids::Id,
    region,
    segment::Segment,
};

pub fn insert<Region>(
    segment: Segment,
    graph: &mut Graph<X, Y, Region>,
) -> Vec<Id>
where
    Region: Debug + region::Get + region::Split,
{
    let mut inserted_nodes = Vec::new();

    for id in find_regions_for_segment(&segment, graph) {
        let (left, right) = Region::get(id, graph).split_x();

        let left = graph.insert_sink(left);
        let right = graph.insert_sink(right);

        graph.replace(
            id,
            Node::X(X {
                segment,
                left,
                right,
            }),
        );

        inserted_nodes.push(id);
    }

    inserted_nodes
}

#[cfg(test)]
mod tests {
    use crate::geometry::triangulation::seidel::trapezoidation::{
        graph::{self, Node, X, Y},
        point::Point,
        region::{Get as _, TestRegion as Region},
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

                // Children should be split from original region.
                assert_eq!(Region::get(left, &graph).split_left, true);
                assert_eq!(Region::get(right, &graph).split_right, true);
            }
            node => panic!("Unexpected node: {:?}", node),
        }
    }

    #[test]
    fn insert_should_return_inserted_node_id() {
        let mut graph = Graph::new();

        let ids = insert(
            Segment::new(Point::new(0.0, 0.0), Point::new(0.0, 1.0)).unwrap(),
            &mut graph,
        );

        assert_eq!(ids, vec![graph.source()]);
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
        graph.get(right).x().unwrap();
    }
}
