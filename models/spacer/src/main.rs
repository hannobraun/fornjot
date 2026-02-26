use clap::Parser;

#[derive(Parser)]
struct SpacerArgs {
    /// Outer radius of the spacer
    #[arg(long, default_value = "1.0")]
    outer_radius: f64,

    /// Inner radius of the spacer
    #[arg(long, default_value = "0.5")]
    inner_radius: f64,

    /// Height of the spacer
    #[arg(long, default_value = "1.0")]
    height: f64,

    #[command(flatten)]
    fj: fj::Arguments,
}

fn main() -> fj::Result {
    let mut fj = fj::Instance::new();
    let args = SpacerArgs::parse();

    let model = spacer::model(
        args.outer_radius,
        args.inner_radius,
        args.height,
        &mut fj.core,
    );
    fj.process_model_args(&model, args.fj)?;

    Ok(())
}
