use fj_math::Point;

use crate::{
    geometry::{Curve, Surface},
    topology::{Cycle, Edge, Face, Vertex},
};

use super::handle::Storage;

pub type Points = Store<Point<3>>;
pub type Curves = Store<Curve>;
pub type Surfaces = Store<Surface>;

pub type Vertices = Store<Vertex>;
pub type Edges = Store<Edge>;
pub type Cycles = Store<Cycle>;
pub type Faces = Store<Face>;

pub type Store<T> = Vec<Storage<T>>;
