//! # The hardcoded geometry (a cube)
//!
//! This is just the hardcoded geometry that I'm using as a test case for this
//! experiment. I kept it simple, as there wasn't much point in trying to
//! exercise the code in [`geometry`](crate::geometry) with anything complex.
//! It's pretty clear that it's limited.
//!
//! I expect follow-up experiments to have more interesting geometry.

use crate::geometry::OpsLog;

pub fn model(ops: &mut OpsLog) {
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
}
