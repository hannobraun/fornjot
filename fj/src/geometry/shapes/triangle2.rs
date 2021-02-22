use nalgebra::Point2;

pub struct Triangle2 {
    pub a: Point2<f32>,
    pub b: Point2<f32>,
    pub c: Point2<f32>,
}

impl Triangle2 {
    pub fn new(a: Point2<f32>, b: Point2<f32>, c: Point2<f32>) -> Self {
        Self { a, b, c }
    }
}
