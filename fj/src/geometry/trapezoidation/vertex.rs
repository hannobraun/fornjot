use nalgebra::Point2;

#[derive(Clone, Copy, Debug)]
pub struct Vertex(pub Point2<f32>);

impl Vertex {
    pub fn is_higher_than(&self, other: &Vertex) -> bool {
        if self.0 == other.0 {
            panic!(
                "{:?}.is_higher_than({:?}: Both are equal. This is a bug.",
                self, other
            );
        }

        match (self.0.y, other.0.y) {
            (a, b) if a > b => true,
            (a, b) if a < b => false,
            (a, b) if a == b => self.0.x < other.0.x,
            (a, b) => {
                panic!("Invalid y coordinates: {}, {}", a, b);
            }
        }
    }

    pub fn is_lower_than(&self, other: &Vertex) -> bool {
        other.is_higher_than(self)
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::Point2;

    use super::Vertex;

    #[test]
    fn vertex_with_high_y_should_be_higher_than_vertex_with_low_y() {
        let high = Vertex(Point2::new(0.0, 1.0));
        let low = Vertex(Point2::new(0.0, 0.0));

        assert!(high.is_higher_than(&low));
        assert!(low.is_lower_than(&high));
    }

    #[test]
    fn vertex_with_equal_y_but_left_x_should_be_higher_than_right_x() {
        let high = Vertex(Point2::new(0.0, 0.0));
        let low = Vertex(Point2::new(1.0, 0.0));

        assert!(high.is_higher_than(&low));
        assert!(low.is_lower_than(&high));
    }

    #[test]
    #[should_panic]
    fn vertex_should_neither_be_higher_nor_lower_if_equal() {
        let vertex = Vertex(Point2::new(0.0, 0.0));
        vertex.is_higher_than(&vertex);
    }
}
