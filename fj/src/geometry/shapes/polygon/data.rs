use std::{
    collections::{HashMap, HashSet},
    ops::Deref as _,
};

use itertools::Itertools as _;
use nalgebra::Vector2;

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

    /// Returns true, if edge is known to be inside of polygon
    ///
    /// Returns `Some(true)`, if the edge is known to be within the polygon.
    ///
    /// A return value of `Some(false)` doesn't necessarily indicate that the
    /// edge is outside of the polygon. It just means that its inside-ness
    /// couldn't be determined.
    ///
    /// Returns `None`, if the vertices of the edge are not part of the polygon.
    pub fn is_inside(&self, edge: &Seg2) -> Option<bool> {
        if self.edges.contains(edge) || self.edges.contains(&edge.reverse()) {
            return Some(true);
        }

        let a_outgoing = self.outgoing_edges(&edge.a)?;
        let a_incoming = self.incoming_edges(&edge.a)?;
        let b_outgoing = self.outgoing_edges(&edge.b)?;
        let b_incoming = self.incoming_edges(&edge.b)?;

        let edges = a_outgoing
            .into_iter()
            .cartesian_product(b_incoming)
            .chain(b_outgoing.into_iter().cartesian_product(a_incoming))
            .filter_map(|(outgoing, incoming)| {
                if outgoing.b == incoming.a {
                    Some((outgoing, incoming))
                } else {
                    None
                }
            })
            .next();

        let (outgoing, incoming) = match edges {
            Some(edges) => edges,
            None => return Some(false),
        };

        let outgoing_v = outgoing.b.deref() - outgoing.a.deref();
        let incoming_v = incoming.b.deref() - incoming.a.deref();

        let dot_product =
            Vector2::new(-incoming_v.y, incoming_v.x).dot(&outgoing_v);
        if dot_product < 0.0 {
            return Some(true);
        }
        if dot_product > 0.0 {
            return Some(false);
        }

        panic!(
            "Invalid polygon. Vertex ({:?}) is on straight line between two \
            other vertices ({:?}, {:?})",
            incoming.b, incoming.a, outgoing.b,
        );
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
    fn is_inside_should_tell_whether_edge_is_inside() {
        let mut data = PolygonData::new();

        let a = Pnt2::new(0.0, 0.0);
        let b = Pnt2::new(1.0, 1.0);
        let c = Pnt2::new(2.0, 0.0);
        let d = Pnt2::new(1.0, 2.0);

        data.insert_edge(Seg2::new(a, b));
        data.insert_edge(Seg2::new(b, c));
        data.insert_edge(Seg2::new(c, d));
        data.insert_edge(Seg2::new(d, a));

        assert_eq!(data.is_inside(&Seg2::new(a, b)), Some(true));
        assert_eq!(data.is_inside(&Seg2::new(b, a)), Some(true));

        assert_eq!(data.is_inside(&Seg2::new(a, c)), Some(false));
        assert_eq!(data.is_inside(&Seg2::new(b, d)), Some(true));
    }

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
        assert_eq!(b_outgoing.len(), 0);

        let a_incoming: &HashSet<_> = data.incoming_edges(&a).unwrap();
        assert_eq!(a_incoming.len(), 0);

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
        assert_eq!(b_outgoing.len(), 0);

        let a_incoming: &HashSet<_> = data.incoming_edges(&a).unwrap();
        assert_eq!(a_incoming.len(), 0);

        let b_incoming: &HashSet<_> = data.incoming_edges(&b).unwrap();
        assert_eq!(b_incoming.len(), 1);
        assert!(b_incoming.contains(&ab));

        // Remove last remaining edge
        data.retain_edges(|_| false);

        assert!(data.outgoing_edges(&a).is_none());
        assert!(data.outgoing_edges(&b).is_none());
        assert!(data.incoming_edges(&a).is_none());
        assert!(data.incoming_edges(&b).is_none());
    }
}
