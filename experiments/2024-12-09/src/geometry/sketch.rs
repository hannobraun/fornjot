use crate::math::Point;

pub struct Sketch {
    pub points: Vec<Point<2>>,
}

impl<I, P> From<I> for Sketch
where
    I: IntoIterator<Item = P>,
    P: Into<Point<2>>,
{
    fn from(points: I) -> Self {
        let points = points.into_iter().map(Into::into).collect();
        Self { points }
    }
}
