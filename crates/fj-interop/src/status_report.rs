//! Struct to store and update status messages

use std::collections::VecDeque;

use chrono::Local;

/// Struct to store and update status messages
#[derive(Default)]
pub struct StatusReport {
    status: VecDeque<String>,
}

impl StatusReport {
    /// Create a new ``StatusReport`` instance with a blank status
    pub fn new() -> Self {
        Self::default()
    }

    /// Update the status
    pub fn update_status(&mut self, status: &str) {
        let date = {
            let date = Local::now();
            format!("{}", date.format("[%H:%M:%S]"))
        };
        let empty_space = " ".repeat(date.chars().count());

        let mut rendered = String::new();
        for (i, line) in status.lines().enumerate() {
            let prefix = if i == 0 { &date } else { &empty_space };
            rendered.push_str(&format!("\n{prefix} {line}"));
        }

        self.status.push_back(rendered);
        if self.status.len() > 5 {
            for _ in 0..(self.status.len() - 5) {
                self.status.pop_front();
            }
        }
    }

    /// Get current status
    pub fn status(&self) -> String {
        self.status
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<String>()
    }

    /// Reset status
    pub fn clear_status(&mut self) {
        self.status.clear();
    }
}
