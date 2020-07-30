use futures::executor::block_on;
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

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

    let mut swap_chain_desc = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8UnormSrgb,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Mailbox,
    };

    let mut swap_chain = device.create_swap_chain(&surface, &swap_chain_desc);

    event_loop.run(move |event, _, _| {
        println!("{:?}", event);

        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                swap_chain_desc.width = size.width;
                swap_chain_desc.height = size.height;

                swap_chain =
                    device.create_swap_chain(&surface, &swap_chain_desc);
            }
            Event::RedrawRequested(_) => {
                let output = swap_chain.get_next_texture().unwrap();

                let mut encoder = device.create_command_encoder(
                    &wgpu::CommandEncoderDescriptor { label: None },
                );

                let _ =
                    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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

                queue.submit(&[encoder.finish()]);
            }
            _ => {}
        }
    })
}
