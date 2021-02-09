pub mod x_split;
pub mod y_split;

use super::{
    graph::{Graph, Node},
    ids::Id,
};

pub fn update(id: Id, graph: &mut Graph) {
    match graph.get(id) {
        Node::X(x) => x_split::update(*x, graph),
        Node::Y(y) => y_split::update(*y, graph),
        Node::Sink(_) => {
            panic!("Split node can't be a sink.")
        }
    }
}
