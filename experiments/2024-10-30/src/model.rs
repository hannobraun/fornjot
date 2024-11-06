use crate::geometry::Operations;

pub fn model() -> anyhow::Result<Operations> {
    let mut mesh = Operations::default();

    let (a, b, c, d, e, f, g, h) = mesh
        .vertex([-0.5, -0.5, -0.5])
        .vertex([0.5, -0.5, -0.5])
        .vertex([-0.5, 0.5, -0.5])
        .vertex([0.5, 0.5, -0.5])
        .vertex([-0.5, -0.5, 0.5])
        .vertex([0.5, -0.5, 0.5])
        .vertex([-0.5, 0.5, 0.5])
        .vertex([0.5, 0.5, 0.5])
        .results();

    [
        [a, e, g], // left
        [a, g, c],
        [b, d, h], // right
        [b, h, f],
        [a, b, f], // front
        [a, f, e],
        [c, h, d], // back
        [c, g, h],
        [a, c, b], // bottom
        [b, c, d],
        [e, f, h], // top
        [e, h, g],
    ]
    .map(|triangle| mesh.triangle(triangle));

    Ok(mesh)
}
