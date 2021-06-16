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
    mesh.triangle(v0, v1, v2);
    mesh.triangle(v2, v1, v3);

    // right
    mesh.triangle(v4, v6, v5);
    mesh.triangle(v6, v7, v5);

    // front
    mesh.triangle(v0, v4, v1);
    mesh.triangle(v4, v5, v1);

    // back
    mesh.triangle(v2, v3, v6);
    mesh.triangle(v6, v3, v7);

    // bottom
    mesh.triangle(v0, v2, v6);
    mesh.triangle(v0, v6, v4);

    // top
    mesh.triangle(v1, v5, v7);
    mesh.triangle(v1, v7, v3);

    fj::run(mesh)?;

    Ok(())
}
