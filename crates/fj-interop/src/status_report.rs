//! Struct to store and update status messages

/// Struct to store and update status messages
pub struct StatusReport {
    status: String,
}

impl StatusReport {
    /// Create a new ``StatusReport`` instance with a blank status
    pub fn new() -> Self {
        Self {
            status: String::new(),
        }
    }

    /// Update the status
    pub fn update_status(&mut self, status: &str) {
        self.status = status.to_string();
    }

    /// Get current status
    pub fn status(&self) -> &str {
        self.status.as_str()
    }
}
