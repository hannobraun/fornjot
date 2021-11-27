mod args;
mod geometry;
mod graphics;
mod input;
mod math;
mod mesh;
mod model;

use std::{collections::HashMap, sync::mpsc, time::Instant};

use bvh::ray::Ray;
use futures::executor::block_on;
use nalgebra::Rotation3;
use notify::Watcher as _;
use tracing::trace;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::{
    args::Args,
    geometry::Shape as _,
    graphics::{DrawConfig, Renderer, Transform, FIELD_OF_VIEW},
    math::{Point, Vector},
    mesh::{HashVector, MeshMaker},
    model::Model,
};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let model = Model::new(args.model);

    let mut parameters = HashMap::new();
    for parameter in args.parameters {
        let mut parameter = parameter.splitn(2, "=");

        let key = parameter
            .next()
            .expect("model parameter: key not found")
            .to_owned();
        let value = parameter
            .next()
            .expect("model parameter: value not found")
            .to_owned();

        parameters.insert(key, value);
    }

    // TASK: Since we're loading the model before setting up the watcher below,
    //       there's a race condition, and a modification could be missed
    //       between those two events.
    //
    //       This can't be addressed with the current structure, since the
    //       watcher closure takes ownership of the model.
    let shape = model.load(&parameters)?;

    let (watcher_tx, watcher_rx) = mpsc::sync_channel(0);

    let watch_path = model.src_path();
    let mut watcher = notify::recommended_watcher(
        move |event: notify::Result<notify::Event>| {
            // TASK: Figure out when this error can happen, find a better way to
            //       handle it.
            let event = event.expect("Error handling watch event");

            if let notify::EventKind::Access(
                notify::event::AccessKind::Close(
                    notify::event::AccessMode::Write,
                ),
            ) = event.kind
            {
                let shape = match model.load(&parameters) {
                    Ok(shape) => shape,
                    Err(model::Error::Compile) => {
                        // TASK: Display error message in graphics window.
                        println!("Error compiling model");
                        return;
                    }
                    Err(err) => {
                        panic!("Error reloading model: {:?}", err);
                    }
                };

                // This will panic, if the other end is disconnected, which is
                // probably the result of a panic on that thread, or the
                // application is being shut down.
                //
                // Either way, not much we can do about it here, except maybe to
                // provide a better error message in the future.
                watcher_tx.send(shape).unwrap();
            }
        },
    )?;
    watcher.watch(&watch_path, notify::RecursiveMode::Recursive)?;

    let aabb = shape.bounding_volume();

    let tolerance = aabb.size().min() / 1000.;
    let mut triangles = shape.faces(tolerance);

    if let Some(path) = args.export {
        let mut mesh_maker = MeshMaker::new();

        for triangle in triangles.0 {
            for vertex in triangle.0 {
                mesh_maker.push(HashVector::from(vertex));
            }
        }

        let vertices =
            mesh_maker.vertices().map(|vertex| vertex.into()).collect();

        let indices: Vec<_> = mesh_maker.indices().collect();
        let triangles = indices
            .chunks(3)
            .map(|triangle| {
                [
                    triangle[0] as usize,
                    triangle[1] as usize,
                    triangle[2] as usize,
                ]
            })
            .collect();

        let mesh = threemf::TriangleMesh {
            vertices,
            triangles,
        };

        threemf::write(path, &mesh)?;

        return Ok(());
    }

    let initial_distance = {
        // Let's make sure we choose a distance, so that the model fills most of
        // the screen.
        //
        // To do that, first compute the model's highest point, as well as the
        // furthers point from the origin, in x and y.
        let highest_point = aabb.max.z;
        let furthest_point =
            [aabb.min.x.abs(), aabb.max.x, aabb.min.y.abs(), aabb.max.y]
                .into_iter()
                .reduce(|a, b| f64::max(a, b))
                // `reduce` can only return `None`, if there are no items in the
                // iterator. And since we're creating an array full of items
                // above, we know this can't panic.
                .unwrap();

        // The actual furthest point is not far enough. We don't want the model
        // to fill the whole screen.
        let furthest_point = furthest_point * 2.;

        // Having computed those points, figuring out how far the camera needs
        // to be from the model is just a bit of trigonometry.
        let distance_from_model =
            furthest_point / (FIELD_OF_VIEW as f64 / 2.).atan();

        // An finally, the distance from the origin is trivial now.
        highest_point + distance_from_model
    };

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Fornjot")
        .with_maximized(true)
        .with_decorations(true)
        .with_transparent(false)
        .build(&event_loop)
        .unwrap();

    let mut previous_time = Instant::now();

    let mut input_handler = input::Handler::new(previous_time);
    let mut renderer = block_on(Renderer::new(&window))?;
    renderer.update_geometry((&triangles.0).into());

    let mut draw_config = DrawConfig::default();
    let mut transform = Transform::new(initial_distance);

    event_loop.run(move |event, _, control_flow| {
        trace!("Handling event: {:?}", event);

        let mut actions = input::Actions::new();

        let now = Instant::now();

        match watcher_rx.try_recv() {
            Ok(shape) => {
                triangles = shape.faces(tolerance);
                renderer.update_geometry((&triangles.0).into());
            }
            Err(mpsc::TryRecvError::Empty) => {
                // Nothing to receive from the channel. We don't care.
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                // The other end has disconnected. This is probably the result
                // of a panic on the other thread, or a program shutdown in
                // progress. In any case, not much we can do here.
                panic!();
            }
        }

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                renderer.handle_resize(size);
            }
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                ..
            } => {
                input_handler.handle_keyboard_input(input, &mut actions);
            }
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                input_handler.handle_cursor_moved(position, &mut transform);

                if let Some(cursor) = input_handler.cursor() {
                    let [width, height] = renderer.surface_size();

                    let w_div_2 = width as f64 / 2.;
                    let h_div_2 = height as f64 / 2.;

                    // Offset of cursor from screen center.
                    let x = cursor.x - w_div_2;
                    let y = cursor.y - h_div_2;

                    // Transform camera position into model coordinates.
                    let origin = transform
                        .view_transform()
                        .inverse()
                        .transform_point(&Point::origin());

                    // For the following computations we need to know, what the
                    // vector from the camera to the model origin is, in model
                    // coordinates.
                    let camera_to_model = Point::origin() - origin;

                    // Furthermore, we need a vector that defines the direction
                    // of the ray, if we were looking down on the model, along
                    // the z-axis. We're not always doing that, of course, so
                    // we'll need to rotate that vector later.
                    //
                    // Dividing `y` by `w_div_2` is not a typo. Field of view is
                    // defined in terms of width, so the height of the screen is
                    // really not important for this calculation.
                    let direction_from_above =
                        Vector::new(x / w_div_2, y / w_div_2, -1.0);

                    // Okay, so we need to rotate `direction_from_above`, but
                    // by what angles? Let's figure that out.
                    //
                    // As mentioned above, the base we're working from is a
                    // vector looking down, so (0, 0, -1). We need to calculate
                    // how much to rotate `direction_from_above`, so it becomes
                    // relevant to the actual viewing direction.
                    //
                    // Let's start by calculating the angle between
                    // `camera_to_model` and our reference vector in the x-z
                    // plane.
                    let rot_x_z =
                        f64::atan2(camera_to_model.x, -camera_to_model.z);

                    // Now let's do that same for the y-z plane.
                    let rot_y_z =
                        f64::atan2(camera_to_model.y, -camera_to_model.z);

                    // Using these angles, we can rotate `direction_from_above`,
                    // so it becomes relative to `camera_to_model`, not its
                    // reference vector.
                    //
                    // Why do we rotate around positive x axis, but negative y
                    // axis? Because the `atan2` operations above are defined in
                    // terms of the x-z and y-z planes, such that z always
                    // points up, and x and y respectively always point right.
                    //
                    // When looking at the x-z plane that way, the y axis is
                    // pointing away from us. When looking at y-z, the x axis is
                    // pointing towards us. Hence the difference.
                    //
                    // If this doesn't make immediate sense, believe me, neither
                    // did it to me. I recommend liberal application of the
                    // right hand rule to sort that out. Maybe grab a partner. I
                    // had to do it alone, and really could have used another
                    // right hand (or two).
                    //
                    // TASK: This still doesn't work correctly. It works better
                    //       than the previous versions, in that I now get a
                    //       sensible-looking direction vector from all
                    //       perspectives, but "sensible-looking" is not quite
                    //       the same as "correct".
                    let rot_x_z =
                        Rotation3::from_axis_angle(&-Vector::y_axis(), rot_x_z);
                    let rot_y_z =
                        Rotation3::from_axis_angle(&Vector::x_axis(), rot_y_z);
                    let direction = rot_x_z.transform_vector(
                        &rot_y_z.transform_vector(&direction_from_above),
                    );

                    let origin = bvh::Point3::new(
                        origin.x as f32,
                        origin.y as f32,
                        origin.z as f32,
                    );
                    let direction = bvh::Vector3::new(
                        direction.x as f32,
                        direction.y as f32,
                        direction.z as f32,
                    );
                    let ray = Ray::new(origin, direction);

                    for triangle in &triangles.0 {
                        let a = triangle.0[0];
                        let b = triangle.0[1];
                        let c = triangle.0[2];

                        let intersection = ray.intersects_triangle(
                            &bvh::Point3::new(
                                a.x as f32, a.y as f32, a.z as f32,
                            ),
                            &bvh::Point3::new(
                                b.x as f32, b.y as f32, b.z as f32,
                            ),
                            &bvh::Point3::new(
                                c.x as f32, c.y as f32, c.z as f32,
                            ),
                        );

                        // TASK: Compute the point on the model where the cursor
                        //       points.
                        if intersection.distance.is_finite() {
                            // TASK: This doesn't show intersections where they
                            //       should be. Something is buggy.
                            dbg!((
                                intersection.distance,
                                intersection.u,
                                intersection.v,
                            ));
                        }
                    }
                }
            }
            Event::WindowEvent {
                event: WindowEvent::MouseInput { state, button, .. },
                ..
            } => {
                input_handler.handle_mouse_input(button, state);
            }
            Event::WindowEvent {
                event: WindowEvent::MouseWheel { delta, .. },
                ..
            } => {
                input_handler.handle_mouse_wheel(delta, now);
            }
            Event::MainEventsCleared => {
                let delta_t = now.duration_since(previous_time);
                previous_time = now;

                input_handler.update(
                    delta_t.as_secs_f64(),
                    now,
                    &mut transform,
                );

                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                match renderer.draw(&transform, &draw_config) {
                    Ok(()) => {}
                    Err(err) => {
                        panic!("Draw error: {}", err);
                    }
                }
            }
            _ => {}
        }

        if actions.exit {
            *control_flow = ControlFlow::Exit;
        }
        if actions.toggle_model {
            draw_config.draw_model = !draw_config.draw_model;
        }
        if actions.toggle_mesh {
            draw_config.draw_mesh = !draw_config.draw_mesh;
        }
    });
}
