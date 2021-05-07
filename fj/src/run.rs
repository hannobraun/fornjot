use futures::executor::block_on;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::{
    graphics::{DrawError, Renderer, Transform},
    input::InputHandler,
    mesh::ToMesh,
};

pub fn run<M>(mesh: M)
where
    M: ToMesh,
{
    tracing_subscriber::fmt::init();

    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Fornjot")
        .with_maximized(true)
        .with_decorations(true)
        .with_transparent(false)
        .build(&event_loop)
        .unwrap();

    let mesh = mesh.to_mesh(0.01);
    let mesh = mesh.into_graphics_mesh();

    let mut input_handler = InputHandler::new();
    let mut transform = Transform::new();
    let mut renderer = block_on(Renderer::new(&window, mesh)).unwrap();

    event_loop.run(move |event, _, control_flow| match event {
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
            input_handler.handle_keyboard_input(input, control_flow);
        }
        Event::WindowEvent {
            event: WindowEvent::CursorMoved { position, .. },
            ..
        } => {
            input_handler.handle_cursor_moved(position, &mut transform);
        }
        Event::WindowEvent {
            event: WindowEvent::MouseInput { state, button, .. },
            ..
        } => {
            input_handler.handle_mouse_input(state, button);
        }
        Event::WindowEvent {
            event: WindowEvent::MouseWheel { delta, .. },
            ..
        } => {
            input_handler.handle_mouse_wheel(delta, &mut transform);
        }
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        Event::RedrawRequested(_) => {
            match renderer.draw(&transform) {
                Ok(()) => {}
                err @ Err(DrawError(wgpu::SwapChainError::Outdated)) => {
                    // I'm getting this from time to time when resizing the
                    // window. It's not catastrophic.
                    println!("Draw error: {:?}", err);
                }
                Err(err) => {
                    panic!("Draw error: {:?}", err);
                }
            }
        }
        _ => {}
    })
}
