#[rustfmt::skip]
fn main() {
    let mut mesh = fj::Mesh::new();

    // left
    let i0  = mesh.vertex([-0.5, -0.5, -0.5], [-1.0,  0.0,  0.0]);
    let i1  = mesh.vertex([-0.5,  0.5, -0.5], [-1.0,  0.0,  0.0]);
    let i2  = mesh.vertex([-0.5, -0.5,  0.5], [-1.0,  0.0,  0.0]);
    let i3  = mesh.vertex([-0.5,  0.5,  0.5], [-1.0,  0.0,  0.0]);

    // right
    let i4  = mesh.vertex([ 0.5, -0.5, -0.5], [ 1.0,  0.0,  0.0]);
    let i5  = mesh.vertex([ 0.5,  0.5, -0.5], [ 1.0,  0.0,  0.0]);
    let i6  = mesh.vertex([ 0.5, -0.5,  0.5], [ 1.0,  0.0,  0.0]);
    let i7  = mesh.vertex([ 0.5,  0.5,  0.5], [ 1.0,  0.0,  0.0]);

    // front
    let i8  = mesh.vertex([-0.5, -0.5, -0.5], [ 0.0, -1.0,  0.0]);
    let i9  = mesh.vertex([ 0.5, -0.5, -0.5], [ 0.0, -1.0,  0.0]);
    let i10 = mesh.vertex([-0.5, -0.5,  0.5], [ 0.0, -1.0,  0.0]);
    let i11 = mesh.vertex([ 0.5, -0.5,  0.5], [ 0.0, -1.0,  0.0]);

    // back
    let i12 = mesh.vertex([-0.5,  0.5, -0.5], [ 0.0,  1.0,  0.0]);
    let i13 = mesh.vertex([ 0.5,  0.5, -0.5], [ 0.0,  1.0,  0.0]);
    let i14 = mesh.vertex([-0.5,  0.5,  0.5], [ 0.0,  1.0,  0.0]);
    let i15 = mesh.vertex([ 0.5,  0.5,  0.5], [ 0.0,  1.0,  0.0]);

    // bottom
    let i16 = mesh.vertex([-0.5, -0.5, -0.5], [ 0.0,  0.0, -1.0]);
    let i17 = mesh.vertex([ 0.5, -0.5, -0.5], [ 0.0,  0.0, -1.0]);
    let i18 = mesh.vertex([-0.5,  0.5, -0.5], [ 0.0,  0.0, -1.0]);
    let i19 = mesh.vertex([ 0.5,  0.5, -0.5], [ 0.0,  0.0, -1.0]);

    // top
    let i20 = mesh.vertex([-0.5, -0.5,  0.5], [ 0.0,  0.0,  1.0]);
    let i21 = mesh.vertex([ 0.5, -0.5,  0.5], [ 0.0,  0.0,  1.0]);
    let i22 = mesh.vertex([-0.5,  0.5,  0.5], [ 0.0,  0.0,  1.0]);
    let i23 = mesh.vertex([ 0.5,  0.5,  0.5], [ 0.0,  0.0,  1.0]);

    mesh.indices.extend_from_slice(&[
        // left
        i0, i2, i1,
        i1, i2, i3,

        // right
        i4, i5, i6,
        i5, i7, i6,

        // front
        i8, i9, i10,
        i9, i11, i10,

        // back
        i12, i14, i13,
        i13, i14, i15,

        // bottom
        i16, i18, i19,
        i16, i19, i17,

        // top
        i20, i21, i23,
        i20, i23, i22,
    ]);

    fj::run(mesh);
}
