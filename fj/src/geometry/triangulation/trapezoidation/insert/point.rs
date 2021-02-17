use std::fmt::Debug;

use crate::geometry::triangulation::trapezoidation::{
    find_region_for_point::find_region_for_point,
    graph::{Graph, Node, X, Y},
    ids::Id,
    point::Point,
    region,
};

pub fn insert<Region>(
    point: Point,
    graph: &mut Graph<X, Y, Region>,
) -> Option<Id>
where
    Region: Debug + region::Get + region::Split,
{
    if let Some(id) = find_region_for_point(&point, graph) {
        let (below, above) = Region::get(id, graph).split_y();

        let below = graph.insert_sink(below);
        let above = graph.insert_sink(above);

        graph.replace(
            id,
            Node::Y(Y {
                point,
                below,
                above,
            }),
        );

        return Some(id);
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::geometry::triangulation::trapezoidation::{
        graph::{self, Node, X, Y},
        point::Point,
        region::{Get as _, TestRegion as Region},
    };

    use super::insert;

    type Graph = graph::Graph<X, Y, Region>;

    #[test]
    fn insert_should_split_region_that_point_is_in() {
        let mut graph = Graph::new();

        let point_to_insert = Point::new(0.0, 0.0);
        insert(point_to_insert, &mut graph);

        match graph.get(graph.source()) {
            Node::Y(Y {
                point,
                below,
                above,
            }) => {
                let below = *below;
                let above = *above;

                // Point must have been inserted.
                assert_eq!(point, &point_to_insert);

                // Children should be split from original region.
                assert_eq!(Region::get(below, &graph).split_lower, true);
                assert_eq!(Region::get(above, &graph).split_upper, true);
            }
            node => panic!("Unexpected node: {:?}", node),
        }
    }

    #[test]
    fn insert_should_return_inserted_node_id() {
        let mut graph = Graph::new();

        let id = insert(Point::new(0.0, 0.0), &mut graph).unwrap();

        assert_eq!(id, graph.source());
    }

    #[test]
    fn insert_should_find_correct_region() {
        let mut graph = Graph::new();

        let region_below = Region::new(1);
        let below = graph.insert_sink(region_below);
        let above = graph.insert_sink(Region::new(2));

        let node = Node::Y(Y {
            point: Point::new(0.0, 1.0),
            below,
            above,
        });
        graph.replace(graph.source(), node);

        insert(Point::new(0.0, 2.0), &mut graph);

        assert_eq!(graph.get(below), &Node::Sink(region_below));
        graph.get(above).y().unwrap();
    }

    #[test]
    fn insert_should_do_nothing_if_point_already_present() {
        let mut graph = Graph::new();

        let region_below = Region::new(1);
        let below = graph.insert_sink(region_below);
        let above = graph.insert_sink(Region::new(2));

        let point = Point::new(0.0, 1.0);
        let node = Node::Y(Y {
            point,
            below,
            above,
        });
        graph.replace(graph.source(), node);

        let graph_before = graph.clone();
        insert(point, &mut graph);

        assert_eq!(graph_before, graph);
    }
}
