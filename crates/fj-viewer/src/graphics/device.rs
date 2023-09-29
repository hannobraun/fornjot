use tracing::debug;

#[derive(Debug)]
pub struct Device {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

impl Device {
    pub async fn from_preferred_adapter(
        instance: &wgpu::Instance,
        surface: &wgpu::Surface,
    ) -> Result<(Self, wgpu::Adapter, wgpu::Features), DeviceError> {
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::None,
                force_fallback_adapter: false,
                compatible_surface: Some(surface),
            })
            .await
            .ok_or(DeviceError::RequestAdapter)?;

        debug!("Using adapter: {:?}", adapter.get_info());

        let (device, features) = Device::new(&adapter).await?;

        Ok((device, adapter, features))
    }

    pub async fn new(
        adapter: &wgpu::Adapter,
    ) -> Result<(Self, wgpu::Features), DeviceError> {
        let features = {
            let desired_features = wgpu::Features::POLYGON_MODE_LINE;
            let available_features = adapter.features();

            // By requesting the intersection of desired and available features,
            // we prevent two things:
            //
            // 1. That requesting the device panics, which would happen if we
            //    requested unavailable features.
            // 2. That a developer ends up accidentally using features that
            //    happen to be available on their machine, but that aren't
            //    necessarily available for all the users.
            desired_features.intersection(available_features)
        };

        let limits = {
            // This is the lowest of the available defaults. It should guarantee
            // that we can run pretty much everywhere.
            let lowest_limits = wgpu::Limits::downlevel_webgl2_defaults();

            // However, these lowest limits aren't necessarily capable of
            // supporting the screen resolution of our current platform, so
            // let's amend them.
            let supported_limits = adapter.limits();
            lowest_limits.using_resolution(supported_limits)
        };

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features,
                    limits,
                },
                None,
            )
            .await?;

        Ok((Device { device, queue }, features))
    }
}

/// Render device initialization error
#[derive(Debug, thiserror::Error)]
pub enum DeviceError {
    /// Failed to request adapter
    #[error("Failed to request adapter")]
    RequestAdapter,

    /// Failed to request device
    #[error("Failed to request device")]
    RequestDevice(#[from] wgpu::RequestDeviceError),
}
