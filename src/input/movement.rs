pub struct Movement {
    pub started: bool,
}

impl Movement {
    pub fn new() -> Self {
        Self { started: false }
    }
}
