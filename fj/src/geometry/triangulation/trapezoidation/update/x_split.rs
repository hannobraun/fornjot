use crate::geometry::triangulation::trapezoidation::{
    graph::Graph,
    ids::Id,
    region::{BoundingRegions, HorizontalBoundary},
};

pub fn update(ids: &[Id], graph: &mut Graph) {
    for &id in ids {
        let x = graph.get(id).x().unwrap().clone();

        // Update new boundary.
        graph.get_mut(x.left).sink_mut().unwrap().right_segment =
            Some(x.segment);
        graph.get_mut(x.right).sink_mut().unwrap().left_segment =
            Some(x.segment);

        // Update upper neighbor.
        if let Some(boundary) = upper_boundary(x.left, graph) {
            match boundary.regions.clone() {
                BoundingRegions::One(upper_neighbor) => {
                    if let Some(boundary) =
                        lower_boundary(upper_neighbor, graph)
                    {
                        match boundary.regions.clone() {
                            BoundingRegions::One(_) => {
                                boundary.regions = BoundingRegions::Two {
                                    left: x.left,
                                    right: x.right,
                                };
                            }
                            region @ BoundingRegions::Two { .. } => {
                                // Due to the non-degeneracy requirement from
                                // the paper, this case is an impossibility. It
                                // simply can't happen, unless something is
                                // buggy.
                                //
                                // If the region had two neighbors above or
                                // below it, it's impossible for one of those to
                                // be split in x. That would have required the
                                // points of the splitting segment to be
                                // inserted, and since those can't be at the
                                // same height as the segment that splits the
                                // existing two regions, one of the following
                                // would have to be true:
                                // - One of the points of the new segment would
                                //   be closer than the closest point of the
                                //   existing segment, meaning the new region
                                //   created by the resulting y split is our
                                //   only neighbor.
                                // - Both points of the new segment are farther
                                //   away than the closest point of the existing
                                //   segment, in which case this is not a
                                //   neighbor of the new regions.
                                //
                                // In both cases, we shouldn't have ended up
                                // here.
                                panic!("Invalid neighbor: {:?}", region);
                            }
                        }
                    }
                }
                BoundingRegions::Two { .. } => {
                    // TASK: We're looking at the upper boundary of the split
                    //       region.
                    //
                    //       Possibilities:
                    //       1. New segment shares a point with the segment that
                    //          splits the neighboring regions. Each new region
                    //          has exactly one neighbor after this.
                    //       2. New segment's upper point is above the lower
                    //          point of the old segment, meaning the new
                    //          segment splits two regions. The lower one has
                    //          two upper neighbors, one of which is the one
                    //          split off by the upper point of the new segment.
                    //          (The upper one only has one upper neighbor.)
                    //
                    //       If the new segment's upper point is below the old
                    //       segment's lower point, the new regions only have
                    //       one upper neighbor, which is not the case we're
                    //       looking at.
                    todo!()
                }
            }
        }

        // Update lower neighbor.
        if let Some(boundary) = lower_boundary(x.left, graph) {
            match boundary.regions.clone() {
                BoundingRegions::One(lower_neighbor) => {
                    if let Some(boundary) =
                        upper_boundary(lower_neighbor, graph)
                    {
                        match boundary.regions.clone() {
                            BoundingRegions::One(_) => {
                                boundary.regions = BoundingRegions::Two {
                                    left: x.left,
                                    right: x.right,
                                };
                            }
                            region @ BoundingRegions::Two { .. } => {
                                // Due to the non-degeneracy requirement from
                                // the paper, this case is an impossibility. It
                                // simply can't happen, unless something is
                                // buggy.
                                //
                                // If the region had two neighbors above or
                                // below it, it's impossible for one of those to
                                // be split in x. That would have required the
                                // points of the splitting segment to be
                                // inserted, and since those can't be at the
                                // same height as the segment that splits the
                                // existing two regions, one of the following
                                // would have to be true:
                                // - One of the points of the new segment would
                                //   be closer than the closest point of the
                                //   existing segment, meaning the new region
                                //   created by the resulting y split is our
                                //   only neighbor.
                                // - Both points of the new segment are farther
                                //   away than the closest point of the existing
                                //   segment, in which case this is not a
                                //   neighbor of the new regions.
                                //
                                // In both cases, we shouldn't have ended up
                                // here.
                                panic!("Invalid neighbor: {:?}", region);
                            }
                        }
                    }
                }
                BoundingRegions::Two { .. } => {
                    // TASK: We're looking at the upper boundary of the split
                    //       region.
                    //
                    //       Possibilities:
                    //       1. New segment shares a point with the segment that
                    //          splits the neighboring regions. Each new region
                    //          has exactly one neighbor after this.
                    //       2. New segment's upper point is above the lower
                    //          point of the old segment, meaning the new
                    //          segment splits two regions. The lower one has
                    //          two upper neighbors, one of which is the one
                    //          split off by the upper point of the new segment.
                    //          (The upper one only has one upper neighbor.)
                    //
                    //       If the new segment's upper point is below the old
                    //       segment's lower point, the new regions only have
                    //       one upper neighbor, which is not the case we're
                    //       looking at.
                    todo!()
                }
            }
        }
    }

    // TASK: Implement:
    //       - Remove upper/lower boundary, if bounding point is on wrong side
    //         of the new segment. Mark affected regions for merging.
    //       - Merge all regions marked for merging that have the same left/
    //         right segment.
}

pub fn upper_boundary(
    id: Id,
    graph: &mut Graph,
) -> Option<&mut HorizontalBoundary> {
    graph
        .get_mut(id)
        .sink_mut()
        .unwrap()
        .upper_boundary
        .as_mut()
}

pub fn lower_boundary(
    id: Id,
    graph: &mut Graph,
) -> Option<&mut HorizontalBoundary> {
    graph
        .get_mut(id)
        .sink_mut()
        .unwrap()
        .lower_boundary
        .as_mut()
}

#[cfg(test)]
mod tests {
    use crate::geometry::triangulation::trapezoidation::{
        graph,
        insert::{point, segment},
        point::Point,
        region::{BoundingRegions, Get as _, Region},
        segment::Segment,
        update::y_split,
    };

    use super::update;

    // Looks useless, but actually makes sure that our calls to `Graph::new`
    // pick up the default type parameters, without us having to add an
    // additional type hint.
    type Graph = graph::Graph;

    #[test]
    fn update_should_update_new_boundary() {
        let mut graph = Graph::new();

        let id_x = segment::insert(
            Segment::new(Point::new(0.0, 0.0), Point::new(0.0, 1.0)).unwrap(),
            &mut graph,
        )
        .pop()
        .unwrap();

        update(&[id_x], &mut graph);

        let x = graph.get(id_x).x().unwrap().clone();
        assert_eq!(
            Region::get(x.left, &graph).right_segment.unwrap(),
            x.segment
        );
        assert_eq!(
            Region::get(x.right, &graph).left_segment.unwrap(),
            x.segment
        );
    }

    #[test]
    fn update_should_update_boundaries_of_vertical_neighbors() {
        let mut graph = Graph::new();

        let upper = Point::new(0.0, 1.0);
        let lower = Point::new(0.0, 0.0);

        let id = point::insert(upper, &mut graph).unwrap();
        y_split::update(id, &mut graph);
        let top_region = graph.get(id).y().unwrap().above;

        let id = point::insert(lower, &mut graph).unwrap();
        y_split::update(id, &mut graph);
        let bottom_region = graph.get(id).y().unwrap().below;

        let ids =
            segment::insert(Segment::new(upper, lower).unwrap(), &mut graph);
        update(&ids, &mut graph);

        let x = graph.get(ids[0].clone()).x().unwrap().clone();
        assert_eq!(
            Region::get(top_region, &graph)
                .lower_boundary
                .clone()
                .unwrap()
                .regions,
            BoundingRegions::Two {
                left: x.left,
                right: x.right
            }
        );
        assert_eq!(
            Region::get(bottom_region, &graph)
                .upper_boundary
                .clone()
                .unwrap()
                .regions,
            BoundingRegions::Two {
                left: x.left,
                right: x.right
            }
        );
    }
}
