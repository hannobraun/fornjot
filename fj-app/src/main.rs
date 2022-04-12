mod args;
mod camera;
mod config;
mod graphics;
mod input;
mod window;

use std::path::PathBuf;
use std::{collections::HashMap, time::Instant};

use fj_host::Model;
use fj_interop::{debug::DebugInfo, mesh::Mesh};
use fj_kernel::algorithms::{triangulate, Tolerance};
use fj_math::{Aabb, Point, Scalar};
use fj_operations::ToShape as _;
use futures::executor::block_on;
use tracing::{trace, warn};
use tracing_subscriber::fmt::format;
use tracing_subscriber::EnvFilter;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

use crate::{
    args::Args,
    camera::Camera,
    config::Config,
    graphics::{DrawConfig, Renderer},
    window::Window,
};

fn main() -> anyhow::Result<()> {
    // Respect `RUST_LOG`. If that's not defined or erroneous, log warnings and
    // above.
    //
    // It would be better to fail, if `RUST_LOG` is erroneous, but I don't know
    // how to distinguish between that and the "not defined" case.
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("WARN")),
        )
        .event_format(format().pretty())
        .init();

    let args = Args::parse();
    let config = Config::load()?;

    let mut path = config.default_path.unwrap_or_else(|| PathBuf::from(""));
    match args.model.or(config.default_model) {
        Some(model) => {
            path.push(model);
        }
        None => {
            anyhow::bail!(
                "No model specified, and no default model configured.\n\
                Specify a model by passing `--model path/to/model`."
            );
        }
    }

    let model = Model::from_path(path, config.target_dir)?;

    let mut parameters = HashMap::new();
    for parameter in args.parameters {
        let mut parameter = parameter.splitn(2, '=');

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

    let shape_processor = ShapeProcessor::new(args.tolerance)?;

    if let Some(path) = args.export {
        let shape = model.load_once(&parameters)?;
        let shape = shape_processor.process(&shape);

        let vertices =
            shape.mesh.vertices().map(|vertex| vertex.into()).collect();

        let indices: Vec<_> = shape.mesh.indices().collect();
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

    let watcher = model.load_and_watch(parameters)?;

    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop);

    let mut previous_time = Instant::now();

    let mut input_handler = input::Handler::new(previous_time);
    let mut renderer = block_on(Renderer::new(&window))?;

    let mut draw_config = DrawConfig::default();

    let mut shape = None;
    let mut camera = None;

    event_loop.run(move |event, _, control_flow| {
        trace!("Handling event: {:?}", event);

        let mut actions = input::Actions::new();

        let now = Instant::now();

        if let Some(new_shape) = watcher.receive() {
            let new_shape = shape_processor.process(&new_shape);
            new_shape.update_geometry(&mut renderer);

            if camera.is_none() {
                camera = Some(Camera::new(&new_shape.aabb));
            }

            shape = Some(new_shape);
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
                if let Some(camera) = &mut camera {
                    input_handler
                        .handle_cursor_moved(position, camera, &window);
                }
            }
            Event::WindowEvent {
                event: WindowEvent::MouseInput { state, button, .. },
                ..
            } => {
                if let (Some(shape), Some(camera)) = (&shape, &camera) {
                    let focus_point = camera.focus_point(
                        &window,
                        input_handler.cursor(),
                        &shape.mesh,
                    );

                    input_handler.handle_mouse_input(
                        button,
                        state,
                        focus_point,
                    );
                }
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

                if let (Some(shape), Some(camera)) = (&shape, &mut camera) {
                    input_handler.update(
                        delta_t.as_secs_f64(),
                        now,
                        camera,
                        &window,
                        &shape.mesh,
                    );
                }

                window.inner().request_redraw();
            }
            Event::RedrawRequested(_) => {
                if let (Some(shape), Some(camera)) = (&shape, &mut camera) {
                    camera.update_planes(&shape.aabb);

                    if let Err(err) = renderer.draw(camera, &draw_config) {
                        warn!("Draw error: {}", err);
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
        if actions.toggle_debug {
            draw_config.draw_debug = !draw_config.draw_debug;
        }
    });
}

struct ShapeProcessor {
    tolerance: Option<Tolerance>,
}

impl ShapeProcessor {
    fn new(tolerance: Option<f64>) -> anyhow::Result<Self> {
        if let Some(tolerance) = tolerance {
            if tolerance <= 0. {
                anyhow::bail!(
                    "Invalid user defined model deviation tolerance: {}.\n\
                    Tolerance must be larger than zero",
                    tolerance
                );
            }
        }

        let tolerance = tolerance
            .map(Scalar::from_f64)
            .map(Tolerance::from_scalar)
            .map(|result| result.unwrap());

        Ok(Self { tolerance })
    }

    fn process(&self, shape: &fj::Shape) -> ProcessedShape {
        let aabb = shape.bounding_volume();

        let tolerance = match self.tolerance {
            None => {
                // Compute a reasonable default for the tolerance value. To do
                // this, we just look at the smallest non-zero extent of the
                // bounding box and divide that by some value.
                let mut min_extent = Scalar::MAX;
                for extent in aabb.size().components {
                    if extent > Scalar::ZERO && extent < min_extent {
                        min_extent = extent;
                    }
                }

                let tolerance = min_extent / Scalar::from_f64(1000.);
                Tolerance::from_scalar(tolerance).unwrap()
            }
            Some(user_defined_tolerance) => user_defined_tolerance,
        };

        let mut debug_info = DebugInfo::new();
        let mesh = triangulate(
            shape.to_shape(tolerance, &mut debug_info),
            tolerance,
            &mut debug_info,
        );

        ProcessedShape {
            aabb,
            mesh,
            debug_info,
        }
    }
}

struct ProcessedShape {
    aabb: Aabb<3>,
    mesh: Mesh<Point<3>>,
    debug_info: DebugInfo,
}

impl ProcessedShape {
    fn update_geometry(&self, renderer: &mut Renderer) {
        renderer.update_geometry(
            (&self.mesh).into(),
            (&self.debug_info).into(),
            self.aabb,
        );
    }
}
