use crate::{
    geometry::{operations::Sweep, shapes::Vertex},
    math::Vector,
};

pub type Edge = Sweep<Vertex, Vector<1>>;
