use nalgebra::Point2;

/// A polygon is a list of points
///
/// Points are expected to be ordered counter-clockwise for a normal polygon. A
/// polygon with its points ordered clockwise is interpreted as a hole in
/// another polygon.
pub struct Polygon(Vec<Point2<f32>>);

impl Polygon {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Insert new point into polygon
    ///
    /// Polygons must not be self-intersecting. This is currently not verified.
    pub fn insert(&mut self, vertex: Point2<f32>) {
        self.0.push(vertex);
    }

    pub fn edges(&self) -> Vec<(Point2<f32>, Point2<f32>)> {
        let mut edges = Vec::new();

        edges.extend(self.0.windows(2).map(|window| (window[0], window[1])));

        let first = *self.0.first().unwrap();
        let last = *self.0.last().unwrap();
        edges.push((last, first));

        edges
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::Point2;

    use super::Polygon;

    #[test]
    fn polygon_should_return_its_edges() {
        let a = Point2::new(0.0, 0.0);
        let b = Point2::new(1.0, 0.0);
        let c = Point2::new(0.0, 1.0);

        let mut polygon = Polygon::new();
        polygon.insert(a);
        polygon.insert(b);
        polygon.insert(c);

        let edges = polygon.edges();

        assert_eq!(edges, vec![(a, b), (b, c), (c, a)]);
    }
}
