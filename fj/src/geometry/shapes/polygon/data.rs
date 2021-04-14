use std::collections::{HashMap, HashSet};

use crate::geometry::shapes::{Pnt2, Seg2};

#[derive(Clone, Debug)]
pub struct PolygonData {
    edges: HashSet<Seg2>,
    vertices: Vertices,
}

impl PolygonData {
    pub fn new() -> Self {
        Self {
            edges: HashSet::new(),
            vertices: Vertices::new(),
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

    pub fn insert_edge(&mut self, edge: Seg2) {
        self.edges.insert(edge);

        self.vertices.up(edge.a);
        self.vertices.up(edge.b);
    }

    pub fn retain_edges(&mut self, mut f: impl FnMut(&Seg2) -> bool) {
        let edges = &mut self.edges;
        let vertices = &mut self.vertices;

        edges.retain(|edge| {
            let retain = f(edge);

            if !retain {
                vertices.down(edge.a);
                vertices.down(edge.b);
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

    pub fn down(&mut self, vertex: Pnt2) {
        *self.0.get_mut(&vertex).unwrap() -= 1;

        if *self.0.get(&vertex).unwrap() == 0 {
            self.0.remove(&vertex);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::shapes::{Pnt2, Seg2};

    use super::PolygonData;

    #[test]
    fn insert_edge_should_update_vertices() {
        let mut data = PolygonData::new();

        let a = Pnt2::from_f32s(0.0, 0.0);
        let b = Pnt2::from_f32s(1.0, 0.0);

        assert_eq!(data.contains_vertex(&a), false);
        assert_eq!(data.contains_vertex(&b), false);

        data.insert_edge(Seg2::new(a, b));

        assert_eq!(data.contains_vertex(&a), true);
        assert_eq!(data.contains_vertex(&b), true);
    }

    #[test]
    fn retain_edges_should_update_vertices() {
        let mut data = PolygonData::new();

        let a = Pnt2::from_f32s(0.0, 0.0);
        let b = Pnt2::from_f32s(1.0, 0.0);
        let c = Pnt2::from_f32s(0.0, 1.0);

        let ab = Seg2::new(a, b);
        let bc = Seg2::new(b, c);

        data.insert_edge(ab);
        data.insert_edge(bc);

        data.retain_edges(|&edge| edge == ab);

        assert_eq!(data.contains_vertex(&a), true);
        assert_eq!(data.contains_vertex(&b), true);
        assert_eq!(data.contains_vertex(&c), false);
    }
}
