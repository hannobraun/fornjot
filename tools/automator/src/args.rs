#[derive(clap::Parser)]
pub enum Args {
    Announcement,
    Sponsors(Sponsors),
}

impl Args {
    pub fn parse() -> Self {
        <Self as clap::Parser>::parse()
    }
}

#[derive(clap::Parser)]
pub struct Sponsors {
    #[clap(short, long)]
    pub for_readme: bool,
}
