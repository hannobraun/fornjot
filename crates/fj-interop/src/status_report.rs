//! Struct to store and update status messages

pub struct StatusReport {
    status: String,
}

impl StatusReport {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn update_status(&mut self, status: &str) {
        self.status = status.to_string();
    }

    pub fn status(&self) -> &str {
        self.status.as_str()
    }

    fn default() -> Self {
        Self {
            status: String::new(),
        }
    }
}
