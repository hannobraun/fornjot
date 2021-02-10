use crate::geometry::triangulation::trapezoidation::{
    graph::{Graph, Y},
    ids::Id,
    region::{BoundingRegions, HorizontalBoundary},
};

pub fn update(id_y: Id, y: Y, graph: &mut Graph) {
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

    if let Some(lower_boundary) = graph
        .get_mut(y.below)
        .sink_mut()
        .unwrap()
        .lower_boundary
        .clone()
    {
        for id in lower_boundary.regions.iter() {
            graph
                .get_mut(id)
                .sink_mut()
                .unwrap()
                .upper_boundary
                .as_mut()
                .unwrap()
                .regions
                .replace(id_y, y.below);
        }
    }
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

        let id_y = point::insert(Point::new(0.0, 0.0), &mut graph).unwrap();
        let y = graph.get(id_y).y().unwrap().clone();

        update(id_y, y.clone(), &mut graph);

        assert_eq!(
            graph
                .get(y.below)
                .sink()
                .unwrap()
                .upper_boundary
                .clone()
                .unwrap(),
            HorizontalBoundary {
                point: y.point,
                regions: BoundingRegions::One(y.above),
            }
        );
        assert_eq!(
            graph
                .get(y.above)
                .sink()
                .unwrap()
                .lower_boundary
                .clone()
                .unwrap(),
            HorizontalBoundary {
                point: y.point,
                regions: BoundingRegions::One(y.below),
            }
        );
    }

    #[test]
    fn update_should_update_lower_neighbors() {
        let mut graph = Graph::new();

        // Split original region horizontally.
        let id_y = point::insert(Point::new(0.0, 0.0), &mut graph).unwrap();
        let y = graph.get(id_y).y().unwrap().clone();
        update(id_y, y.clone(), &mut graph);

        let lowest = y.below;

        // Now split the upper of those two regions again.
        let id_y = point::insert(Point::new(0.0, 1.0), &mut graph).unwrap();
        let y = graph.get(id_y).y().unwrap().clone();

        update(id_y, y.clone(), &mut graph);

        assert_eq!(
            graph
                .get(lowest)
                .sink()
                .unwrap()
                .upper_boundary
                .clone()
                .unwrap()
                .regions,
            BoundingRegions::One(y.below)
        );
    }
}
