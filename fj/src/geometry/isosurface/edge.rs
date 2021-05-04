use super::GridIndex;

#[derive(Debug, PartialEq)]
pub struct Edge {
    pub a: Value,
    pub b: Value,
}

impl Edge {
    pub fn reverse(self) -> Self {
        Self {
            a: self.b,
            b: self.a,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Value {
    pub index: GridIndex,
    pub value: f32,
}
