use chrono::{Datelike, Utc};

pub fn date() -> String {
    let now = Utc::now();
    let year = now.year();
    format!("{year}-{:02}-{:02}", now.month(), now.day())
}
