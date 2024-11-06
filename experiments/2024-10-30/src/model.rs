use crate::geometry::Operations;

pub fn model() -> anyhow::Result<Operations> {
    let mut ops = Operations::default();

    let (a, b, c, d, e, f, g, h) = ops
        .vertex([-0.5, -0.5, -0.5])
        .vertex([0.5, -0.5, -0.5])
        .vertex([-0.5, 0.5, -0.5])
        .vertex([0.5, 0.5, -0.5])
        .vertex([-0.5, -0.5, 0.5])
        .vertex([0.5, -0.5, 0.5])
        .vertex([-0.5, 0.5, 0.5])
        .vertex([0.5, 0.5, 0.5])
        .results();

    ops.triangle([a, e, g]) // left
        .triangle([a, g, c])
        .triangle([b, d, h]) // right
        .triangle([b, h, f])
        .triangle([a, b, f]) // front
        .triangle([a, f, e])
        .triangle([c, h, d]) // back
        .triangle([c, g, h])
        .triangle([a, c, b]) // bottom
        .triangle([b, c, d])
        .triangle([e, f, h]) // top
        .triangle([e, h, g]);

    Ok(ops)
}
