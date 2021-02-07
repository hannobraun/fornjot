use crate::geometry::triangulation::trapezoidation::{
    point,
    segment::{self, Segment},
};

use super::{
    graph::{Graph, Node, X, Y},
    ids::Id,
};

/// Find the regions that are split by the given segment
pub fn find_regions_for_segment<Region>(
    segment: &Segment,
    graph: &Graph<X, Y, Region>,
) -> Vec<Id> {
    let mut next_nodes = vec![graph.source()];
    let mut found_regions = Vec::new();

    while let Some(current_node) = next_nodes.pop() {
        match graph.get(current_node) {
            Node::X(X {
                segment: s,
                left,
                right,
            }) => {
                match segment.relation_to_segment(s) {
                    Some(segment::Relation::Left) => {
                        next_nodes.push(*left);
                    }
                    Some(segment::Relation::Right) => {
                        next_nodes.push(*right);
                    }
                    None => {
                        // This is a case that can obviously happen between the
                        // segments of a trapezoidation. The question is, if it
                        // can actually happen with a valid graph. I hope it
                        // can't, because I don't know how to handle it.
                        panic!(
                            "No defined relation between segments: {:?}, {:?}",
                            segment, s
                        )
                    }
                }
            }
            Node::Y(Y {
                point,
                below,
                above,
            }) => {
                match segment.relation_to_point(point) {
                    Some(point::Relation::Below) => {
                        next_nodes.push(*below);
                    }
                    Some(point::Relation::Above) => {
                        next_nodes.push(*above);
                    }
                    None => {
                        // No defined relation to a Y node means that the
                        // segment cuts the regions below _and_ above the node.
                        next_nodes.push(*below);
                        next_nodes.push(*above);
                    }
                }
            }
            Node::Sink(_) => {
                found_regions.push(current_node);
            }
        }
    }

    found_regions
}

#[cfg(test)]
mod tests {
    use crate::geometry::triangulation::trapezoidation::{
        graph::{self, Node, X, Y},
        point::Point,
        region::TestRegion as Region,
        segment::Segment,
    };

    use super::find_regions_for_segment;

    type Graph = graph::Graph<X, Y, Region>;

    #[test]
    fn find_regions_for_segment_should_find_root_region_if_none_other_exist() {
        let graph = Graph::new();

        let region = find_regions_for_segment(
            &Segment::new(Point::new(0.0, 0.0), Point::new(0.0, 1.0)).unwrap(),
            &graph,
        );
        assert_eq!(region, vec![graph.source()]);
    }

    #[test]
    fn find_regions_for_segment_should_choose_the_right_path_at_x_node() {
        let mut graph = Graph::new();

        let left = graph.insert_sink(Region(1));
        let right = graph.insert_sink(Region(2));

        let node = Node::X(X {
            segment: Segment::new(Point::new(1.0, 0.0), Point::new(1.0, 1.0))
                .unwrap(),
            left,
            right,
        });

        graph.replace(graph.source(), node);

        assert_eq!(
            find_regions_for_segment(
                &Segment::new(Point::new(0.0, 0.0), Point::new(0.0, 1.0))
                    .unwrap(),
                &graph
            ),
            vec![left],
        );
        assert_eq!(
            find_regions_for_segment(
                &Segment::new(Point::new(2.0, 0.0), Point::new(2.0, 1.0))
                    .unwrap(),
                &graph
            ),
            vec![right],
        );
    }

    #[test]
    fn find_regions_for_segment_should_choose_the_right_path_at_y_node() {
        let mut graph = Graph::new();

        let below = graph.insert_sink(Region(1));
        let above = graph.insert_sink(Region(2));

        let node = Node::Y(Y {
            point: Point::new(0.0, 1.0),
            below,
            above,
        });

        graph.replace(graph.source(), node);

        assert_eq!(
            find_regions_for_segment(
                &Segment::new(Point::new(0.0, 0.0), Point::new(0.0, 1.0))
                    .unwrap(),
                &graph
            ),
            vec![below],
        );
        assert_eq!(
            find_regions_for_segment(
                &Segment::new(Point::new(0.0, 1.0), Point::new(0.0, 2.0))
                    .unwrap(),
                &graph
            ),
            vec![above],
        );
    }

    #[test]
    fn find_regions_for_segment_should_follow_both_paths_at_y_node() {
        let mut graph = Graph::new();

        let below = graph.insert_sink(Region(1));
        let above = graph.insert_sink(Region(2));

        let node = Node::Y(Y {
            point: Point::new(0.0, 1.0),
            below,
            above,
        });

        graph.replace(graph.source(), node);

        assert_eq!(
            find_regions_for_segment(
                &Segment::new(Point::new(0.0, 0.0), Point::new(0.0, 2.0))
                    .unwrap(),
                &graph
            ),
            vec![above, below],
        );
    }
}
