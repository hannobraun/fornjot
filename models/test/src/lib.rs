use std::f64::consts::PI;

use fj::{syntax::*, Angle};

#[fj::model]
pub fn model() -> fj::Shape {
    let a = star(4, 1., [0, 255, 0, 200], Some(-30.));
    let b = star(5, -1., [255, 0, 0, 255], None)
        .rotate([1., 1., 1.], Angle::from_deg(45.))
        .translate([3., 3., 1.]);
    let c = spacer().translate([6., 6., 1.]);

    let group = a.group(&b).group(&c);

    group.into()
}

fn star(
    num_points: u64,
    height: f64,
    color: [u8; 4],
    arm_angle: Option<f64>,
) -> fj::Shape {
    let r1 = 1.;
    let r2 = 2.;

    // We need to figure out where to generate vertices, depending on the number
    // of points the star is supposed to have. Let's generate an iterator that
    // gives us the angle and radius for each vertex.
    let num_vertices = num_points * 2;
    let vertex_iter = (0..num_vertices).map(|i| {
        let angle = Angle::from_rad(2. * PI / num_vertices as f64 * i as f64);
        let radius = if i % 2 == 0 { r1 } else { r2 };
        (angle, radius)
    });

    // Now that we got that iterator prepared, generating the vertices is just a
    // bit of trigonometry.
    let mut outer = Vec::new();
    let mut inner = Vec::new();
    for (angle, radius) in vertex_iter {
        let (sin, cos) = angle.rad().sin_cos();

        let x = cos * radius;
        let y = sin * radius;

        if let Some(angle) = arm_angle {
            outer.push(fj::SketchSegment {
                endpoint: [x, y],
                route: fj::SketchSegmentRoute::Arc {
                    angle: fj::Angle::from_deg(angle),
                },
            });
            inner.push(fj::SketchSegment {
                endpoint: [x / 2., y / 2.],
                route: fj::SketchSegmentRoute::Arc {
                    angle: fj::Angle::from_deg(-angle),
                },
            });
        } else {
            outer.push(fj::SketchSegment {
                endpoint: [x, y],
                route: fj::SketchSegmentRoute::Direct,
            });
            inner.push(fj::SketchSegment {
                endpoint: [x / 2., y / 2.],
                route: fj::SketchSegmentRoute::Direct,
            });
        }
    }

    let outer = fj::Sketch::from_segments(outer).unwrap().with_color(color);
    let inner = fj::Sketch::from_segments(inner).unwrap();

    let footprint = fj::Difference2d::from_shapes([outer.into(), inner.into()]);

    let star = fj::Sweep::from_path(footprint.into(), [0., 0., height]);

    star.into()
}

fn spacer() -> fj::Shape {
    let outer = 2.;
    let inner = 1.;
    let height = 2.;

    let outer_edge = fj::Sketch::from_circle(fj::Circle::from_radius(outer))
        .with_color([0, 0, 255, 255]);
    let inner_edge = fj::Sketch::from_circle(fj::Circle::from_radius(inner));

    let footprint = outer_edge.difference(&inner_edge);
    let spacer = footprint.sweep([0., 0., height]);

    spacer.into()
}
