#[derive(clap::Parser)]
pub enum Args {
    CreateReleaseAnnouncement,
    Sponsors,
}

impl Args {
    pub fn parse() -> Self {
        <Self as clap::Parser>::parse()
    }
}
