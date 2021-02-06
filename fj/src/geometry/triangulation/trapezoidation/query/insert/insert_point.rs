use crate::geometry::triangulation::trapezoidation::{
    point::Point,
    query::{
        find_region_for_point::{find_region_for_point, Found},
        graph::{Graph, Node, X, Y},
    },
};

pub fn insert_point<Region>(point: Point, graph: &mut Graph<X, Y, Region>)
where
    Region: Default,
{
    match find_region_for_point(&point, graph) {
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

        let point_to_insert = Point::new(0.0, 0.0);
        insert_point(point_to_insert, &mut graph);

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

                // Children should be sinks
                assert_eq!(graph.get(below).is_sink(), true);
                assert_eq!(graph.get(above).is_sink(), true);

                // Children should be distinct
                assert_ne!(below, above);

                // Children should be new nodes
                assert_ne!(graph.source(), below);
                assert_ne!(graph.source(), above);
            }
            node => panic!("Unexpected node: {:?}", node),
        }
    }

    #[test]
    fn insert_point_should_find_correct_region() {
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

        insert_point(Point::new(0.0, 2.0), &mut graph);

        assert_eq!(graph.get(below), &Node::Sink(region_below));
        assert_eq!(graph.get(above).is_y(), true);
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
