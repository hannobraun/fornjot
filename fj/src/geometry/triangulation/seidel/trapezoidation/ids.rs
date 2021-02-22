#[derive(Clone, Debug, PartialEq)]
pub struct Ids {
    next_id: u64,
}

impl Ids {
    pub fn new() -> Self {
        Self { next_id: 0 }
    }
    pub fn next(&mut self) -> Id {
        let id = Id(self.next_id);
        self.next_id += 1;
        id
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Id(u64);
