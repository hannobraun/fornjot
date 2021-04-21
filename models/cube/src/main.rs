#[rustfmt::skip]
fn main() {
    let mut mesh = fj::Mesh::new();

    let i0  = mesh.vertex([-0.5, -0.5, -0.5]);
    let i1  = mesh.vertex([-0.5, -0.5,  0.5]);
    let i2  = mesh.vertex([-0.5,  0.5, -0.5]);
    let i3  = mesh.vertex([-0.5,  0.5,  0.5]);
    let i4  = mesh.vertex([ 0.5, -0.5, -0.5]);
    let i5  = mesh.vertex([ 0.5, -0.5,  0.5]);
    let i6  = mesh.vertex([ 0.5,  0.5, -0.5]);
    let i7  = mesh.vertex([ 0.5,  0.5,  0.5]);

    // left
    mesh.triangle(i0, i1, i2);
    mesh.triangle(i2, i1, i3);

    // right
    mesh.triangle(i4, i6, i5);
    mesh.triangle(i6, i7, i5);

    // front
    mesh.triangle(i0, i4, i1);
    mesh.triangle(i4, i5, i1);

    // back
    mesh.triangle(i2, i3, i6);
    mesh.triangle(i6, i3, i7);

    // bottom
    mesh.triangle(i0, i2, i6);
    mesh.triangle(i0, i6, i4);

    // top
    mesh.triangle(i1, i5, i7);
    mesh.triangle(i1, i7, i3);

    fj::run(mesh).unwrap();
}
