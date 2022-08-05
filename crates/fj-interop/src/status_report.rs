//! Struct to store and update status messages

/// Struct to store and update status messages
pub struct StatusReport {
    status: Vec<String>,
}

impl StatusReport {
    /// Create a new ``StatusReport`` instance with a blank status
    pub fn new() -> Self {
        Self { status: Vec::new() }
    }

    /// Update the status
    pub fn update_status(&mut self, status: &str) {
        self.status.push(status.to_string());
    }

    /// Get current status
    pub fn status(&self) -> String {
        self.status.join("\n")
    }

    /// Reset status
    pub fn clear_status(&mut self) {
        self.status.clear();
    }
}

impl Default for StatusReport {
    fn default() -> Self {
        Self::new()
    }
}
