use clap::Parser;

#[derive(Parser)]
struct Parameters {
    /// Size of the cuboid along the x-axis
    #[arg(long, default_value = "3.0")]
    x: f64,

    /// Size of the cuboid along the y-axis
    #[arg(long, default_value = "2.0")]
    y: f64,

    /// Size of the cuboid along the z-axis
    #[arg(long, default_value = "1.0")]
    z: f64,

    #[command(flatten)]
    fj: fj::Args,
}

fn main() -> fj::Result {
    let params = Parameters::parse();

    let tri_mesh = cuboid::model([params.x, params.y, params.z]);

    if let Some(path) = params.fj.export {
        fj::export::export(tri_mesh.external_triangles(), path)?;
    } else {
        fj::viewer::make_viewer_and_spawn_thread({
            let tri_mesh = tri_mesh.clone();

            |viewer| {
                fj::DEBUG_WINDOW.initialize(&viewer);
                viewer.open_window().display_mesh(tri_mesh);
            }
        })?;
    }

    Ok(())
}
