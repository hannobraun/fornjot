use crate::geometry::triangulation::trapezoidation::{
    graph::Graph,
    ids::Id,
    region::{BoundingRegions, Get as _, Region},
};

pub fn update(ids: &[Id], graph: &mut Graph) {
    for &id in ids {
        let x = graph.get(id).x().unwrap().clone();

        // Update left region
        let left = Region::get_mut(x.left, graph);
        left.right_segment = Some(x.segment);

        // TASK: Add left region to a list of regions left of the new segment.
        //       Those are candidates for merging.

        // Let's store these here and now. We're just reading them, so making a
        // copy is fine, and we can't keep `left` around forever, as it mutable
        // borrows `graph`, which we're going to need again soon.
        //
        // That we're taking the boundaries from `left` is arbitrary. It could
        // just as well be `right`, as they both have the same un-updated
        // horizontal boundaries at this point.
        let lower_boundary = left.lower_boundary.clone();
        let upper_boundary = left.upper_boundary.clone();

        // Update right region
        let right = Region::get_mut(x.right, graph);
        right.left_segment = Some(x.segment);

        // TASK: Add right region to a list of regions right of the new segment.
        //       Those are candidates for merging.

        // Update upper boundary
        if let Some(boundary) = upper_boundary {
            match boundary.regions {
                BoundingRegions::One(upper_neighbor) => {
                    Region::get_mut(upper_neighbor, graph)
                        .lower_boundary_mut()
                        .update_after_neighbor_split(x.left, x.right);
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

        // Update lower boundary
        if let Some(boundary) = lower_boundary {
            match boundary.regions {
                BoundingRegions::One(lower_neighbor) => {
                    Region::get_mut(lower_neighbor, graph)
                        .upper_boundary_mut()
                        .update_after_neighbor_split(x.left, x.right);
                }
                BoundingRegions::Two { .. } => {
                    // TASK: Implement. See comment above for directions.
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
    fn update_should_update_boundaries_of_single_vertical_neighbors() {
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
