use parry2d::shape::Segment;

use super::VertexChain;

pub struct Polygon(Vec<VertexChain>);

impl Polygon {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn insert_chain(&mut self, chain: VertexChain) {
        self.0.push(chain)
    }

    pub fn edges(&self) -> Vec<Segment> {
        let mut edges = Vec::new();

        for chain in &self.0 {
            edges.extend_from_slice(&chain.segments());
        }

        edges
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::Point2;
    use parry2d::shape::Segment;

    use crate::geometry::shapes::VertexChain;

    use super::Polygon;

    #[test]
    fn polygon_chain_return_its_edges() {
        let mut polygon = Polygon::new();

        let a = Point2::new(0.0, 0.0);
        let b = Point2::new(2.0, 0.0);
        let c = Point2::new(0.0, 2.0);

        let p = Point2::new(1.0, 1.0);
        let q = Point2::new(1.0, 1.5);
        let r = Point2::new(1.5, 1.0);

        let chain_a = VertexChain::from(&[a, b, c][..]);
        let chain_b = VertexChain::from(&[p, q, r][..]);

        polygon.insert_chain(chain_a);
        polygon.insert_chain(chain_b);

        assert_eq!(
            polygon.edges(),
            vec![
                Segment::new(a, b),
                Segment::new(b, c),
                Segment::new(c, a),
                Segment::new(p, q),
                Segment::new(q, r),
                Segment::new(r, p)
            ]
        );
    }
}
