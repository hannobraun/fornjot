pub type RawId = u32;

pub struct Ids {
    next_id: RawId,
}

impl Ids {
    pub fn new() -> Self {
        Self { next_id: 0 }
    }

    pub fn next(&mut self) -> RawId {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
}

#[cfg(test)]
mod tests {
    use super::Ids;

    #[test]
    fn it_should_generate_ids() {
        let mut ids = Ids::new();

        let a = ids.next();
        let b = ids.next();

        assert_ne!(a, b);
    }
}
