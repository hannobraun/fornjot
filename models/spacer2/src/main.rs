use std::f32::consts::FRAC_PI_2;

use fj::{geometry::attributes::SurfaceMesh as _, prelude::*};
use nalgebra::{vector, Rotation3};

fn main() -> anyhow::Result<()> {
    let outer = 50.0;
    let inner = 25.0;
    let height = 25.0;

    let cross_section = fj::Quad::from_points([
        [inner, height],
        [outer, height],
        [outer, 0.0],
        [inner, 0.0],
    ])?;
    let spacer = fj::Toroid::from_shape(cross_section);

    let spacer = spacer.rotate(Rotation3::new(vector![FRAC_PI_2, 0., 0.]));

    fj::run_mesh(spacer.surface_mesh(360))
}
