//! Struct to store and update status messages

pub struct StatusReport {
    status: String,
}

impl StatusReport {
    pub fn new() -> Self {
        StatusReport {
            status: String::new(),
        }
    }

    pub fn update_status(&mut self, status: &str) {
        self.status = status.to_string();
    }

    pub fn status(&self) -> &str {
        self.status.as_str()
    }
}
