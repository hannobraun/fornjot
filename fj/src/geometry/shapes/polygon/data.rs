use std::collections::{HashMap, HashSet};

use crate::geometry::shapes::{Pnt2, Seg2};

#[derive(Clone, Debug)]
pub struct PolygonData {
    edges: HashSet<Seg2>,
    vertices: Vertices,

    outgoing_edges: HashMap<Pnt2, HashSet<Seg2>>,
    incoming_edges: HashMap<Pnt2, HashSet<Seg2>>,
}

impl PolygonData {
    pub fn new() -> Self {
        Self {
            edges: HashSet::new(),
            vertices: Vertices::new(),

            outgoing_edges: HashMap::new(),
            incoming_edges: HashMap::new(),
        }
    }

    pub fn edges(&self) -> &HashSet<Seg2> {
        &self.edges
    }

    pub fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }

    pub fn contains_vertex(&self, vertex: &Pnt2) -> bool {
        self.vertices.0.contains_key(vertex)
    }

    pub fn outgoing_edges(&self, vertex: &Pnt2) -> Option<&HashSet<Seg2>> {
        self.outgoing_edges.get(vertex)
    }

    pub fn incoming_edges(&self, vertex: &Pnt2) -> Option<&HashSet<Seg2>> {
        self.incoming_edges.get(vertex)
    }

    pub fn insert_edge(&mut self, edge: Seg2) {
        self.edges.insert(edge);

        self.vertices.up(edge.a);
        self.vertices.up(edge.b);

        self.incoming_edges.entry(edge.a).or_insert(HashSet::new());
        self.outgoing_edges.entry(edge.b).or_insert(HashSet::new());
        self.outgoing_edges
            .entry(edge.a)
            .or_insert(HashSet::new())
            .insert(edge);
        self.incoming_edges
            .entry(edge.b)
            .or_insert(HashSet::new())
            .insert(edge);
    }

    pub fn retain_edges(&mut self, mut f: impl FnMut(&Seg2) -> bool) {
        let edges = &mut self.edges;
        let vertices = &mut self.vertices;
        let outgoing_edges = &mut self.outgoing_edges;
        let incoming_edges = &mut self.incoming_edges;

        edges.retain(|edge| {
            let retain = f(edge);

            if !retain {
                let removed_a = vertices.down(edge.a);
                let removed_b = vertices.down(edge.b);

                outgoing_edges.get_mut(&edge.a).unwrap().remove(edge);
                incoming_edges.get_mut(&edge.b).unwrap().remove(edge);

                if removed_a {
                    incoming_edges.remove(&edge.a);
                    outgoing_edges.remove(&edge.a);
                }
                if removed_b {
                    incoming_edges.remove(&edge.b);
                    outgoing_edges.remove(&edge.b);
                }
            }

            retain
        });
    }
}

#[derive(Clone, Debug)]
struct Vertices(HashMap<Pnt2, u32>);

impl Vertices {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    pub fn up(&mut self, vertex: Pnt2) {
        *self.0.entry(vertex).or_insert(0) += 1;
    }

    pub fn down(&mut self, vertex: Pnt2) -> bool {
        *self.0.get_mut(&vertex).unwrap() -= 1;

        if *self.0.get(&vertex).unwrap() == 0 {
            self.0.remove(&vertex);
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::geometry::shapes::{Pnt2, Seg2};

    use super::PolygonData;

    #[test]
    fn insert_edge_should_update_vertices() {
        let mut data = PolygonData::new();

        let a = Pnt2::new(0.0, 0.0);
        let b = Pnt2::new(1.0, 0.0);

        assert_eq!(data.contains_vertex(&a), false);
        assert_eq!(data.contains_vertex(&b), false);

        data.insert_edge(Seg2::new(a, b));

        assert_eq!(data.contains_vertex(&a), true);
        assert_eq!(data.contains_vertex(&b), true);
    }

    #[test]
    fn insert_edge_should_update_edge_counts() {
        let mut data = PolygonData::new();

        let a = Pnt2::new(0.0, 0.0);
        let b = Pnt2::new(1.0, 0.0);

        let ab = Seg2::new(a, b);
        data.insert_edge(ab);

        let a_outgoing: &HashSet<_> = data.outgoing_edges(&a).unwrap();
        assert_eq!(a_outgoing.len(), 1);
        assert!(a_outgoing.contains(&ab));

        let b_outgoing: &HashSet<_> = data.outgoing_edges(&b).unwrap();
        assert!(b_outgoing.is_empty());

        let a_incoming: &HashSet<_> = data.incoming_edges(&a).unwrap();
        assert!(a_incoming.is_empty());

        let b_incoming: &HashSet<_> = data.incoming_edges(&b).unwrap();
        assert_eq!(b_incoming.len(), 1);
        assert!(b_incoming.contains(&ab));

        let ba = Seg2::new(b, a);
        data.insert_edge(ba);

        let a_outgoing: &HashSet<_> = data.outgoing_edges(&a).unwrap();
        assert_eq!(a_outgoing.len(), 1);
        assert!(a_outgoing.contains(&ab));

        let b_outgoing: &HashSet<_> = data.outgoing_edges(&b).unwrap();
        assert_eq!(b_outgoing.len(), 1);
        assert!(b_outgoing.contains(&ba));

        let a_incoming: &HashSet<_> = data.incoming_edges(&a).unwrap();
        assert_eq!(a_incoming.len(), 1);
        assert!(a_incoming.contains(&ba));

        let b_incoming: &HashSet<_> = data.incoming_edges(&b).unwrap();
        assert_eq!(b_incoming.len(), 1);
        assert!(b_incoming.contains(&ab));
    }

    #[test]
    fn retain_edges_should_update_vertices() {
        let mut data = PolygonData::new();

        let a = Pnt2::new(0.0, 0.0);
        let b = Pnt2::new(1.0, 0.0);
        let c = Pnt2::new(0.0, 1.0);

        let ab = Seg2::new(a, b);
        let bc = Seg2::new(b, c);

        data.insert_edge(ab);
        data.insert_edge(bc);

        data.retain_edges(|&edge| edge == ab);

        assert_eq!(data.contains_vertex(&a), true);
        assert_eq!(data.contains_vertex(&b), true);
        assert_eq!(data.contains_vertex(&c), false);
    }

    #[test]
    fn retain_edges_should_update_edge_counts() {
        let mut data = PolygonData::new();

        let a = Pnt2::new(0.0, 0.0);
        let b = Pnt2::new(1.0, 0.0);

        let ab = Seg2::new(a, b);
        let ba = Seg2::new(b, a);

        data.insert_edge(ab);
        data.insert_edge(ba);

        // Keep a -> b
        data.retain_edges(|&edge| edge == ab);

        let a_outgoing: &HashSet<_> = data.outgoing_edges(&a).unwrap();
        assert_eq!(a_outgoing.len(), 1);
        assert!(a_outgoing.contains(&ab));

        let b_outgoing: &HashSet<_> = data.outgoing_edges(&b).unwrap();
        assert!(b_outgoing.is_empty());

        let a_incoming: &HashSet<_> = data.incoming_edges(&a).unwrap();
        assert!(a_incoming.is_empty());

        let b_incoming: &HashSet<_> = data.incoming_edges(&b).unwrap();
        assert_eq!(b_incoming.len(), 1);
        assert!(b_incoming.contains(&ab));

        // Remove last remaining edge
        data.retain_edges(|_| false);

        let a_outgoing = data.outgoing_edges(&a);
        let b_outgoing = data.outgoing_edges(&b);
        let a_incoming = data.incoming_edges(&a);
        let b_incoming = data.incoming_edges(&b);

        assert!(a_outgoing.is_none());
        assert!(b_outgoing.is_none());
        assert!(a_incoming.is_none());
        assert!(b_incoming.is_none());
    }
}
