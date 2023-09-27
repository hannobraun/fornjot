#[derive(Debug)]
pub struct Device {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}
