use crate::{
    objects::{
        geometry::{Triangles, Vertex},
        topology::{Face, Faces, HalfEdge},
    },
    store::{Index, Store},
};

pub fn from_half_edge_and_two_vertices(
    e01: Index<HalfEdge>,
    [v2, v3]: [Index<Vertex>; 2],
    vertices: &Store<Vertex>,
    triangles: &mut Triangles,
    half_edges: &mut Store<HalfEdge>,
    faces: &mut Faces,
) -> Index<Face> {
    Sketch::new()
        .push_half_edge(e01)
        .push_vertex(v2, half_edges)
        .push_vertex(v3, half_edges)
        .close_with_half_edge(half_edges)
        .build(vertices, half_edges, triangles, faces)
}

pub struct Sketch<const SIZE: usize> {
    boundary: [Index<HalfEdge>; SIZE],
}

impl Sketch<0> {
    pub fn new() -> Self {
        Self { boundary: [] }
    }

    pub fn push_half_edge(self, e01: Index<HalfEdge>) -> Sketch<1> {
        let [] = self.boundary;
        Sketch { boundary: [e01] }
    }
}

impl Sketch<1> {
    pub fn push_half_edge(self, e12: Index<HalfEdge>) -> Sketch<2> {
        let [e01] = self.boundary;

        Sketch {
            boundary: [e01, e12],
        }
    }

    pub fn push_vertex(
        self,
        v2: Index<Vertex>,
        half_edges: &mut Store<HalfEdge>,
    ) -> Sketch<2> {
        let [e01] = self.boundary;

        let [_, v1] = half_edges[e01].boundary;
        let e12 = half_edges.push(HalfEdge { boundary: [v1, v2] });

        Sketch {
            boundary: [e01, e12],
        }
    }
}

impl Sketch<2> {
    pub fn push_half_edge(self, e23: Index<HalfEdge>) -> Sketch<3> {
        let [e01, e12] = self.boundary;

        Sketch {
            boundary: [e01, e12, e23],
        }
    }

    pub fn push_vertex(
        self,
        v3: Index<Vertex>,
        half_edges: &mut Store<HalfEdge>,
    ) -> Sketch<3> {
        let [e01, e12] = self.boundary;

        let [_, v2] = half_edges[e12].boundary;
        let e23 = half_edges.push(HalfEdge { boundary: [v2, v3] });

        Sketch {
            boundary: [e01, e12, e23],
        }
    }
}

impl Sketch<3> {
    pub fn close_with_half_edge(
        self,
        half_edges: &mut Store<HalfEdge>,
    ) -> Sketch<4> {
        let [e01, e12, e23] = self.boundary;

        let [v0, _] = half_edges[e01].boundary;
        let [_, v3] = half_edges[e23].boundary;

        let e30 = half_edges.push(HalfEdge { boundary: [v3, v0] });

        Sketch {
            boundary: [e01, e12, e23, e30],
        }
    }

    pub fn push_half_edge(self, e30: Index<HalfEdge>) -> Sketch<4> {
        let [e01, e12, e23] = self.boundary;

        Sketch {
            boundary: [e01, e12, e23, e30],
        }
    }
}

impl Sketch<4> {
    pub fn build(
        self,
        vertices: &Store<Vertex>,
        half_edges: &Store<HalfEdge>,
        triangles: &mut Triangles,
        faces: &mut Faces,
    ) -> Index<Face> {
        let [e01, e12, e23, e30] = self.boundary;

        let [v0, v1b] = half_edges[e01].boundary;
        let [v1, v2b] = half_edges[e12].boundary;
        let [v2, v3b] = half_edges[e23].boundary;
        let [v3, v0b] = half_edges[e30].boundary;

        assert_eq!(v0, v0b);
        assert_eq!(v1, v1b);
        assert_eq!(v2, v2b);
        assert_eq!(v3, v3b);

        let t012 = triangles.push([v0, v1, v2], vertices);
        let t123 = triangles.push([v0, v2, v3], vertices);

        faces.push(Face {
            boundary: vec![e01, e12, e23, e30],
            triangles: vec![t012, t123],
        })
    }
}
