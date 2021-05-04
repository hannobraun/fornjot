use super::GridIndex;

#[derive(Debug, PartialEq)]
pub struct Edge {
    pub a: Value,
    pub b: Value,
}

#[derive(Debug, PartialEq)]
pub struct Value {
    pub index: GridIndex,
    pub value: f32,
}
