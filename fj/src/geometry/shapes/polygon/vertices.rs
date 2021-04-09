use crate::geometry::{point::Pnt2, shapes::vertex_chain::Neighbors};

use super::Polygon;

pub struct Vertices<'r>(pub(super) &'r mut Polygon);

impl Vertices<'_> {
    pub fn neighbors_of(&self, vertex: impl Into<Pnt2>) -> Option<Neighbors> {
        // TASK: Convert to use `self.edges`.

        // TASK: Support zero or multiple vertex chains.
        assert_eq!(self.0.chains.len(), 1);
        self.0.chains[0].neighbors_of(vertex)
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::{
        point::Pnt2,
        shapes::{Polygon, VertexChain},
    };

    #[test]
    fn neighbors_of_should_return_neighbors_of_vertex() {
        let mut polygon = Polygon::new();

        let a = Pnt2::from_f32s(0.0, 0.0);
        let b = Pnt2::from_f32s(1.0, 0.0);
        let c = Pnt2::from_f32s(0.0, 1.0);
        polygon.insert_chain(VertexChain::from(&[a, b, c][..]));

        let neighbors_of_a = polygon.vertices().neighbors_of(a).unwrap();
        let neighbors_of_b = polygon.vertices().neighbors_of(b).unwrap();
        let neighbors_of_c = polygon.vertices().neighbors_of(c).unwrap();

        assert!(neighbors_of_a.contains(b));
        assert!(neighbors_of_a.contains(c));

        assert!(neighbors_of_b.contains(a));
        assert!(neighbors_of_b.contains(c));

        assert!(neighbors_of_c.contains(a));
        assert!(neighbors_of_c.contains(b));
    }
}
