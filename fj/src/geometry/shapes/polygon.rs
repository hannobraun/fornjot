use nalgebra::Point2;

/// A polygon is a list of points
///
/// Points are expected to be ordered counter-clockwise for a normal polygon. A
/// polygon with its points ordered clockwise is interpreted as a hole in
/// another polygon.
pub struct Polygon(Vec<Point2<f32>>);
