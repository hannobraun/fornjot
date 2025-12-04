fn main() -> anyhow::Result<()> {
    let vertices = [
        [0., 0., 0.], // v0
        [0., 0., 1.], // v1
        [0., 1., 0.], // v2
        [0., 1., 1.], // v3
        [1., 0., 0.], // v4
        [1., 0., 1.], // v5
        [1., 1., 0.], // v6
        [1., 1., 1.], // v7
    ];

    let triangles = [
        [0, 4, 5], // t0
    ];

    dbg!(vertices, triangles);

    Ok(())
}
