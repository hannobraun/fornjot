use crate::geometry::triangulation::trapezoidation::{
    graph::Graph,
    ids::Id,
    region::{BoundingRegions, Get as _, HorizontalBoundary, Region},
};

pub fn update(id: Id, graph: &mut Graph) {
    let y = graph.get(id).y().unwrap().clone();

    // Update lower region
    let below = Region::get_mut(y.below, graph);
    below.upper_boundary = Some(HorizontalBoundary {
        point: y.point,
        regions: BoundingRegions::One(y.above),
    });
    if let Some(lower_boundary) = &below.lower_boundary {
        for lower_id in lower_boundary.regions.iter() {
            replace_in_upper_boundary(lower_id, id, y.below, graph);
        }
    }

    // Update upper region
    Region::get_mut(y.above, graph).lower_boundary = Some(HorizontalBoundary {
        point: y.point,
        regions: BoundingRegions::One(y.below),
    });
}

pub fn replace_in_upper_boundary(id: Id, old: Id, new: Id, graph: &mut Graph) {
    Region::get_mut(id, graph)
        .upper_boundary_mut()
        .regions
        .replace(old, new);
}

#[cfg(test)]
mod tests {
    use crate::geometry::triangulation::trapezoidation::{
        graph,
        insert::point,
        point::Point,
        region::{BoundingRegions, Get as _, HorizontalBoundary, Region},
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
        update(id_y, &mut graph);

        let y = graph.get(id_y).y().unwrap();
        assert_eq!(
            Region::get(y.below, &graph).upper_boundary.clone().unwrap(),
            HorizontalBoundary {
                point: y.point,
                regions: BoundingRegions::One(y.above),
            }
        );
        assert_eq!(
            Region::get(y.above, &graph).lower_boundary.clone().unwrap(),
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
        update(id_y, &mut graph);

        let y = graph.get(id_y).y().unwrap();
        let lowest = y.below;

        // Now split the upper of those two regions again.
        let id_y = point::insert(Point::new(0.0, 1.0), &mut graph).unwrap();

        update(id_y, &mut graph);

        let y = graph.get(id_y).y().unwrap();
        assert_eq!(
            Region::get(lowest, &graph)
                .upper_boundary
                .clone()
                .unwrap()
                .regions,
            BoundingRegions::One(y.below)
        );
    }
}
