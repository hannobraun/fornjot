#[derive(clap::Parser)]
pub enum Args {
    Announcement,
    Sponsors,
}

impl Args {
    pub fn parse() -> Self {
        <Self as clap::Parser>::parse()
    }
}
