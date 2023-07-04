#[derive(clap::Parser)]
pub enum Args {
    #[command(subcommand)]
    Blog(Blog),
    Sponsors(Sponsors),
}

impl Args {
    pub fn parse() -> Self {
        <Self as clap::Parser>::parse()
    }
}

#[derive(clap::Subcommand)]
pub enum Blog {
    Release,
    SponsorUpdate,
}

#[derive(clap::Parser)]
pub struct Sponsors {
    #[clap(short, long)]
    pub for_readme: bool,
}
