use std::f64::consts::PI;

use fj::{
    core::{
        objects::{Cycle, Region, Sketch, Solid},
        operations::{
            build::{BuildCycle, BuildRegion, BuildSketch},
            insert::Insert,
            reverse::Reverse,
            sweep::SweepSketch,
            update::{UpdateRegion, UpdateSketch},
        },
    },
    math::Vector,
};

pub fn model(
    num_points: u64,
    r1: f64,
    r2: f64,
    h: f64,
    core: &mut fj::core::Instance,
) -> Solid {
    let num_vertices = num_points * 2;
    let vertex_iter = (0..num_vertices).map(|i| {
        let angle_rad = 2. * PI / num_vertices as f64 * i as f64;
        let radius = if i % 2 == 0 { r1 } else { r2 };
        (angle_rad, radius)
    });

    let mut outer_points = Vec::new();
    let mut inner_points = Vec::new();

    for (angle_rad, radius) in vertex_iter {
        let (sin, cos) = angle_rad.sin_cos();

        let x = cos * radius;
        let y = sin * radius;

        outer_points.push([x, y]);
        inner_points.push([x / 2., y / 2.]);
    }

    let bottom_surface = core.services.objects.surfaces.xy_plane();
    let sweep_path = Vector::from([0., 0., h]);

    Sketch::empty()
        .add_region(
            Region::polygon(outer_points, &mut core.services)
                .add_interiors([Cycle::polygon(
                    inner_points,
                    &mut core.services,
                )
                .reverse(&mut core.services)
                .insert(&mut core.services)])
                .insert(&mut core.services),
        )
        .sweep_sketch(bottom_surface, sweep_path, &mut core.services)
}
