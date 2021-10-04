#[rustfmt::skip]
fn main() -> anyhow::Result<()> {
    let mut mesh = fj::Mesh::new();

    let d = 50.0;

    let v0  = [-d, -d, -d];
    let v1  = [-d, -d,  d];
    let v2  = [-d,  d, -d];
    let v3  = [-d,  d,  d];
    let v4  = [ d, -d, -d];
    let v5  = [ d, -d,  d];
    let v6  = [ d,  d, -d];
    let v7  = [ d,  d,  d];

    // left
    mesh.triangle(fj::Triangle::new([v0, v1, v2]).unwrap());
    mesh.triangle(fj::Triangle::new([v2, v1, v3]).unwrap());

    // right
    mesh.triangle(fj::Triangle::new([v4, v6, v5]).unwrap());
    mesh.triangle(fj::Triangle::new([v6, v7, v5]).unwrap());

    // front
    mesh.triangle(fj::Triangle::new([v0, v4, v1]).unwrap());
    mesh.triangle(fj::Triangle::new([v4, v5, v1]).unwrap());

    // back
    mesh.triangle(fj::Triangle::new([v2, v3, v6]).unwrap());
    mesh.triangle(fj::Triangle::new([v6, v3, v7]).unwrap());

    // bottom
    mesh.triangle(fj::Triangle::new([v0, v2, v6]).unwrap());
    mesh.triangle(fj::Triangle::new([v0, v6, v4]).unwrap());

    // top
    mesh.triangle(fj::Triangle::new([v1, v5, v7]).unwrap());
    mesh.triangle(fj::Triangle::new([v1, v7, v3]).unwrap());

    fj::run_mesh(mesh)?;

    Ok(())
}
