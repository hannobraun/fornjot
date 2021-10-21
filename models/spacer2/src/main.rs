use fj::geometry::attributes::SurfaceMesh as _;

fn main() -> anyhow::Result<()> {
    let outer = 50.0;
    let inner = 25.0;
    let height = 25.0;

    let cross_section = fj::Quad::from_points([
        [inner, height],
        [outer, height],
        [outer, 0.0],
        [inner, 0.0],
    ])?;
    let spacer = fj::Toroid::from_shape(cross_section);

    fj::run_mesh(spacer.surface_mesh(100))
}
