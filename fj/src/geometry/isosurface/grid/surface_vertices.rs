use std::collections::BTreeMap;

use nalgebra::Point;

use super::Index;

#[derive(Debug)]
pub struct SurfaceVertices(pub BTreeMap<Index, Point<f32, 3>>);
