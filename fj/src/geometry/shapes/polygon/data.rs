use std::collections::HashSet;

use crate::geometry::segment::Seg2;

#[derive(Clone, Debug)]
pub struct PolygonData {
    // TASK: Make private.
    pub edges: HashSet<Seg2>,
    // TASK: Add field `vertices` that contains all polygon vertices. This is
    //       required to easily and efficiently query whether a vertex is part
    //       of the polygon and will probably be useful for other things too.
}

impl PolygonData {
    pub fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }

    pub fn insert_edge(&mut self, edge: Seg2) {
        self.edges.insert(edge);
    }

    pub fn edges(&self) -> HashSet<Seg2> {
        self.edges.clone()
    }
}
