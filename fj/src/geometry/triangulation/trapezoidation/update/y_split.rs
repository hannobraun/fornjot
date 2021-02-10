use crate::geometry::triangulation::trapezoidation::{
    graph::{Graph, Y},
    region::{BoundingRegions, HorizontalBoundary},
};

pub fn update(y: Y, graph: &mut Graph) {
    graph.get_mut(y.below).sink_mut().unwrap().upper_boundary =
        Some(HorizontalBoundary {
            point: y.point,
            regions: BoundingRegions::One(y.above),
        });
    graph.get_mut(y.above).sink_mut().unwrap().lower_boundary =
        Some(HorizontalBoundary {
            point: y.point,
            regions: BoundingRegions::One(y.below),
        });

    // TASK: Update lower neighbors.
}

#[cfg(test)]
mod tests {
    use crate::geometry::triangulation::trapezoidation::{
        graph,
        insert::point,
        point::Point,
        region::{BoundingRegions, HorizontalBoundary},
    };

    use super::update;

    // Looks useless, but actually makes sure that our calls to `Graph::new`
    // pick up the default type parameters, without us having to add an
    // additional type hint.
    type Graph = graph::Graph;

    #[test]
    fn update_should_update_new_boundary() {
        let mut graph = Graph::new();

        let node = point::insert(Point::new(0.0, 0.0), &mut graph).unwrap();
        let node = *graph.get(node).y().unwrap();

        update(node, &mut graph);

        assert_eq!(
            graph
                .get(node.below)
                .sink()
                .unwrap()
                .upper_boundary
                .unwrap(),
            HorizontalBoundary {
                point: node.point,
                regions: BoundingRegions::One(node.above),
            }
        );
        assert_eq!(
            graph
                .get(node.above)
                .sink()
                .unwrap()
                .lower_boundary
                .unwrap(),
            HorizontalBoundary {
                point: node.point,
                regions: BoundingRegions::One(node.below),
            }
        );
    }
}
