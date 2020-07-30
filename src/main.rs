mod renderer;

use futures::executor::block_on;
use winit::{
    event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use self::renderer::Renderer;

fn main() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Fornjot")
        .with_maximized(true)
        .with_decorations(true)
        .with_transparent(false)
        .build(&event_loop)
        .unwrap();

    let surface = wgpu::Surface::create(&window);

    let adapter = block_on(wgpu::Adapter::request(
        &wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::Default,
            compatible_surface: Some(&surface),
        },
        wgpu::BackendBit::VULKAN,
    ))
    .unwrap();

    let (device, queue) =
        block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            extensions: wgpu::Extensions {
                anisotropic_filtering: false,
            },
            limits: wgpu::Limits::default(),
        }));

    let size = window.inner_size();

    let swap_chain_desc = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8UnormSrgb,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Mailbox,
    };

    let swap_chain = device.create_swap_chain(&surface, &swap_chain_desc);

    let mut renderer = Renderer {
        surface,
        device,
        queue,
        swap_chain_desc,
        swap_chain,
    };

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
            renderer.swap_chain_desc.width = size.width;
            renderer.swap_chain_desc.height = size.height;

            renderer.swap_chain = renderer.device.create_swap_chain(
                &renderer.surface,
                &renderer.swap_chain_desc,
            );
        }
        Event::WindowEvent {
            event:
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                },
            ..
        } => {
            *control_flow = ControlFlow::Exit;
        }
        Event::RedrawRequested(_) => {
            let output = renderer.swap_chain.get_next_texture().unwrap();

            let mut encoder = renderer.device.create_command_encoder(
                &wgpu::CommandEncoderDescriptor { label: None },
            );

            let _ = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[
                    wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &output.view,
                        resolve_target: None,
                        load_op: wgpu::LoadOp::Clear,
                        store_op: wgpu::StoreOp::Store,
                        clear_color: wgpu::Color::WHITE,
                    },
                ],
                depth_stencil_attachment: None,
            });

            renderer.queue.submit(&[encoder.finish()]);
        }
        _ => {}
    })
}
