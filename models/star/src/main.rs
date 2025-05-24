use clap::Parser;

#[derive(Parser)]
struct Parameters {
    /// Number of points in the star
    #[arg(long, default_value = "5")]
    points: u64,

    /// Inner radius of the star
    #[arg(long, default_value = "1.0")]
    inner_radius: f64,

    /// Outer radius of the star
    #[arg(long, default_value = "2.0")]
    outer_radius: f64,

    /// Height of the star
    #[arg(long, default_value = "1.0")]
    height: f64,

    #[command(flatten)]
    fj: fj::Args,
}

fn main() -> fj::Result {
    let mut fj = fj::Instance::new();
    let params = Parameters::parse();

    let model = star::model(
        params.points,
        params.inner_radius,
        params.outer_radius,
        params.height,
        &mut fj.core,
    );
    fj.process_model_args(&model, params.fj)?;

    Ok(())
}
