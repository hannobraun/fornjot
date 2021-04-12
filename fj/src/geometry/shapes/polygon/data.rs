use std::collections::HashSet;

use crate::geometry::segment::Seg2;

#[derive(Clone, Debug)]
pub struct PolygonData {
    edges: HashSet<Seg2>,
    // TASK: Add field `vertices` that contains all polygon vertices. This is
    //       required to easily and efficiently query whether a vertex is part
    //       of the polygon and will probably be useful for other things too.
}

impl PolygonData {
    pub fn new() -> Self {
        Self {
            edges: HashSet::new(),
        }
    }

    pub fn edges(&self) -> &HashSet<Seg2> {
        &self.edges
    }

    pub fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }

    pub fn insert_edge(&mut self, edge: Seg2) {
        self.edges.insert(edge);
    }

    pub fn retain_edges(&mut self, f: impl FnMut(&Seg2) -> bool) {
        self.edges.retain(f);
    }
}
