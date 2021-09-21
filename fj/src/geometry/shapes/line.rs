use crate::{
    geometry::{operations::Sweep, shapes::Vertex},
    math::Vector,
};

pub type Line = Sweep<Vertex, Vector<1>>;
