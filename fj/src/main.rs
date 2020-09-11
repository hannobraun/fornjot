mod graphics;
mod input;
mod run;
mod transform;
mod vertices;

use crate::{graphics::Geometry, run::run};

fn main() {
    let mut geometry = Geometry::new();
    geometry.vertices.extend_from_slice(vertices::VERTICES);
    geometry.indices.extend_from_slice(vertices::INDICES);

    run(geometry);
}
