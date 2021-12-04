pub struct Movement {
    pub started: bool,
}

impl Movement {
    pub fn new() -> Self {
        Self { started: false }
    }

    pub fn start(&mut self) {
        self.started = true;
    }

    pub fn stop(&mut self) {
        self.started = false;
    }
}
