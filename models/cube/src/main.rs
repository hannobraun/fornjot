#[rustfmt::skip]
fn main() {
    let mut mesh = fj::Mesh::new();

    // left
    let i0  = mesh.vertex([-0.5, -0.5, -0.5]);
    let i1  = mesh.vertex([-0.5,  0.5, -0.5]);
    let i2  = mesh.vertex([-0.5, -0.5,  0.5]);
    let i3  = mesh.vertex([-0.5,  0.5,  0.5]);

    // right
    let i4  = mesh.vertex([ 0.5, -0.5, -0.5]);
    let i5  = mesh.vertex([ 0.5,  0.5, -0.5]);
    let i6  = mesh.vertex([ 0.5, -0.5,  0.5]);
    let i7  = mesh.vertex([ 0.5,  0.5,  0.5]);

    // front
    let i8  = mesh.vertex([-0.5, -0.5, -0.5]);
    let i9  = mesh.vertex([ 0.5, -0.5, -0.5]);
    let i10 = mesh.vertex([-0.5, -0.5,  0.5]);
    let i11 = mesh.vertex([ 0.5, -0.5,  0.5]);

    // back
    let i12 = mesh.vertex([-0.5,  0.5, -0.5]);
    let i13 = mesh.vertex([ 0.5,  0.5, -0.5]);
    let i14 = mesh.vertex([-0.5,  0.5,  0.5]);
    let i15 = mesh.vertex([ 0.5,  0.5,  0.5]);

    // bottom
    let i16 = mesh.vertex([-0.5, -0.5, -0.5]);
    let i17 = mesh.vertex([ 0.5, -0.5, -0.5]);
    let i18 = mesh.vertex([-0.5,  0.5, -0.5]);
    let i19 = mesh.vertex([ 0.5,  0.5, -0.5]);

    // top
    let i20 = mesh.vertex([-0.5, -0.5,  0.5]);
    let i21 = mesh.vertex([ 0.5, -0.5,  0.5]);
    let i22 = mesh.vertex([-0.5,  0.5,  0.5]);
    let i23 = mesh.vertex([ 0.5,  0.5,  0.5]);

    // left
    mesh.triangle(i0, i2, i1);
    mesh.triangle(i1, i2, i3);

    // right
    mesh.triangle(i4, i5, i6);
    mesh.triangle(i5, i7, i6);

    // front
    mesh.triangle(i8, i9, i10);
    mesh.triangle(i9, i11, i10);

    // back
    mesh.triangle(i12, i14, i13);
    mesh.triangle(i13, i14, i15);

    // bottom
    mesh.triangle(i16, i18, i19);
    mesh.triangle(i16, i19, i17);

    // top
    mesh.triangle(i20, i21, i23);
    mesh.triangle(i20, i23, i22);

    fj::run(mesh);
}
