mod args;
mod pull_requests;
mod release;
mod run;
mod sponsors;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    run::run().await
}
