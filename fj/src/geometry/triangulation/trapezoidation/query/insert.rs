//! Insertion of segments into the graph

use crate::geometry::triangulation::trapezoidation::point::Point;

use super::{
    find_region::{find_region, Found},
    graph::{Graph, Node, X, Y},
};

// TASK: Implement `insert`
//       - Insert upper point
//       - Insert lower point
//       - Insert segment
// TASK: Decide where to update region when inserting.
// TASK: Merge all regions that have the same bounding segments.

pub fn insert_point<Region>(point: Point, graph: &mut Graph<X, Y, Region>)
where
    Region: Default,
{
    match find_region(&point, graph) {
        Found::Region(id) => {
            let below = graph.insert_sink(Region::default());
            let above = graph.insert_sink(Region::default());

            graph.replace(
                id,
                Node::Y(Y {
                    point,
                    below,
                    above,
                }),
            );
        }
        Found::Point(_) => {
            // Point is already in the graph. Nothing to do.
        }
    }
}

// TASK: Implement `insert_segment`:
//       - Compare with each node
//         - At Y node:
//           - If above or below, follow respective path
//           - If neither, follow _both_ paths
//         - At X node: Check whether left or right, follow respective path
//           - If segments overlap in y, "left" and "right" are unambiguously
//             defined depending on whether the left or right horizontal
//             extension of the overlapping endpoint hit the other segment. Just
//             find an endpoint that is between the other segment's endpoints,
//             find point on other segment with same y-coordinate,
//             compare x-coordinate.
//           - Endpoints of a segment can never be _on_ another segment, unless
//             the initial polygon is self-intersecting, which we don't accept
//             as valid input. (And the paper only allows segments with common
//             endpoints, at most.)
//           - But what if the segments don't overlap in y? There obviously are
//             segments like that, but will we ever have to compare them when
//             inserting into a valid tree? I don't know.
//         - At Sink:
//           Split region into left and right. Update bounding segments.

#[cfg(test)]
mod tests {
    use crate::geometry::triangulation::trapezoidation::{
        point::Point,
        query::graph::{self, Node, X, Y},
    };

    use super::insert_point;

    type Graph = graph::Graph<X, Y, Region>;

    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    struct Region(u64);

    #[test]
    fn insert_point_should_split_region_that_point_is_in() {
        let mut graph = Graph::new();

        let region_below = Region(1);
        let below = graph.insert_sink(region_below);
        let above = graph.insert_sink(Region(2));

        let node = Node::Y(Y {
            point: Point::new(0.0, 1.0),
            below,
            above,
        });

        graph.replace(graph.source(), node);

        let point_to_insert = Point::new(0.0, 2.0);
        insert_point(point_to_insert, &mut graph);

        // Region below should be untouched.
        assert_eq!(graph.get(below), &Node::Sink(region_below));

        // Region above should be replaced.
        match graph.get(above) {
            Node::Y(Y {
                point,
                below,
                above,
            }) => {
                // Point must have been inserted.
                assert_eq!(point, &point_to_insert);

                // Children should be sinks
                assert_eq!(graph.get(*below).is_sink(), true);
                assert_eq!(graph.get(*above).is_sink(), true);

                // Children should be distinct
                assert_ne!(below, above);
            }
            node => panic!("Unexpected node: {:?}", node),
        }
    }

    #[test]
    fn insert_point_should_do_nothing_if_point_already_present() {
        let mut graph = Graph::new();

        let region_below = Region(1);
        let below = graph.insert_sink(region_below);
        let above = graph.insert_sink(Region(2));

        let point = Point::new(0.0, 1.0);
        let node = Node::Y(Y {
            point,
            below,
            above,
        });

        graph.replace(graph.source(), node);

        let graph_before = graph.clone();
        insert_point(point, &mut graph);

        assert_eq!(graph_before, graph);
    }
}
