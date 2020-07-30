use winit::{dpi::PhysicalSize, window::Window};

pub struct Renderer {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    swap_chain_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
}

impl Renderer {
    pub async fn new(window: &Window) -> Result<Self, AdapterRequestError> {
        let surface = wgpu::Surface::create(window);

        let adapter = wgpu::Adapter::request(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: Some(&surface),
            },
            wgpu::BackendBit::VULKAN,
        )
        .await
        .ok_or(AdapterRequestError)?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                extensions: wgpu::Extensions {
                    anisotropic_filtering: false,
                },
                limits: wgpu::Limits::default(),
            })
            .await;

        let size = window.inner_size();

        let swap_chain_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Mailbox,
        };

        let swap_chain = device.create_swap_chain(&surface, &swap_chain_desc);

        Ok(Self {
            surface,
            device,
            queue,
            swap_chain_desc,
            swap_chain,
        })
    }

    pub fn handle_resize(&mut self, size: PhysicalSize<u32>) {
        self.swap_chain_desc.width = size.width;
        self.swap_chain_desc.height = size.height;

        self.swap_chain = self
            .device
            .create_swap_chain(&self.surface, &self.swap_chain_desc);
    }

    pub fn draw(&mut self) -> Result<(), DrawError> {
        let output = self
            .swap_chain
            .get_next_texture()
            .map_err(|err| DrawError(err))?;

        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor { label: None },
        );

        let _ = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: &output.view,
                resolve_target: None,
                load_op: wgpu::LoadOp::Clear,
                store_op: wgpu::StoreOp::Store,
                clear_color: wgpu::Color::WHITE,
            }],
            depth_stencil_attachment: None,
        });

        self.queue.submit(&[encoder.finish()]);

        Ok(())
    }
}

#[derive(Debug)]
pub struct AdapterRequestError;

#[derive(Debug)]
pub struct DrawError(wgpu::TimeOut);
