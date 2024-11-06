use crate::geometry::Operations;

pub fn model() -> anyhow::Result<Operations> {
    let mut mesh = Operations::default();

    mesh.vertex([-0.5, -0.5, -0.5]) // 0
        .vertex([0.5, -0.5, -0.5]) // 1
        .vertex([-0.5, 0.5, -0.5]) // 2
        .vertex([0.5, 0.5, -0.5]) // 3
        .vertex([-0.5, -0.5, 0.5]) // 4
        .vertex([0.5, -0.5, 0.5]) // 5
        .vertex([-0.5, 0.5, 0.5]) // 6
        .vertex([0.5, 0.5, 0.5]); // 7

    [
        [0, 4, 6], // left
        [0, 6, 2],
        [1, 3, 7], // right
        [1, 7, 5],
        [0, 1, 5], // front
        [0, 5, 4],
        [2, 7, 3], // back
        [2, 6, 7],
        [0, 2, 1], // bottom
        [1, 2, 3],
        [4, 5, 7], // top
        [4, 7, 6],
    ]
    .map(|triangle| {
        let triangle = triangle.map(|index| mesh.vertices[index]);
        mesh.triangle(triangle)
    });

    Ok(mesh)
}
