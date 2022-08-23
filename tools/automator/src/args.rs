use chrono::{Date, NaiveDate, Utc};

#[derive(clap::Parser)]
pub enum Args {
    CreateReleaseAnnouncement(CreateReleaseAnnouncement),
}

impl Args {
    pub fn parse() -> Self {
        <Self as clap::Parser>::parse()
    }
}

#[derive(clap::Parser)]
pub struct CreateReleaseAnnouncement {
    pub last_release_date: NaiveDate,
    pub version: String,
}

impl CreateReleaseAnnouncement {
    pub fn last_release_date(&self) -> Date<Utc> {
        Date::from_utc(self.last_release_date, Utc)
    }
}
