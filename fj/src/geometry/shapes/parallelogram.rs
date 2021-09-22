use crate::{
    geometry::{operations::Sweep, shapes::Edge},
    math::Vector,
};

pub type Parallelogram = Sweep<Edge, Vector<2>>;
