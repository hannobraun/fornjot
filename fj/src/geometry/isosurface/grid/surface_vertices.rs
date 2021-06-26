use std::collections::BTreeMap;

use nalgebra::Point;

use super::Index;

pub type SurfaceVertices = BTreeMap<Index, Point<f32, 3>>;
