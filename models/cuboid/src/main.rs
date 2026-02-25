use clap::Parser;
use fj::core::interop::{Color, MeshTriangle, TriMesh};

#[derive(Parser)]
struct CuboidArgs {
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
    fj: fj::Arguments,
}

fn main() -> fj::Result {
    let args = CuboidArgs::parse();

    let model = cuboid::model([args.x, args.y, args.z]);

    let triangles = model.topology.solids[model.solid]
        .boundary
        .iter()
        .flat_map(|&face| &model.topology.faces[face].approx);

    let mut tri_mesh = TriMesh::new();

    for &triangle in triangles {
        tri_mesh.triangles.push(MeshTriangle {
            inner: triangle,
            is_internal: false,
            color: Color::default(),
        });
    }

    fj::process_model(tri_mesh, args.fj)?;

    Ok(())
}
