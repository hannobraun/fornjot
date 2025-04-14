use tracing::{debug, error};

#[derive(Debug)]
pub struct Device {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

impl Device {
    pub async fn from_preferred_adapter(
        instance: &wgpu::Instance,
        surface: &wgpu::Surface<'_>,
    ) -> Result<(Self, wgpu::Adapter, wgpu::Features), DeviceError> {
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::None,
                force_fallback_adapter: false,
                compatible_surface: Some(surface),
            })
            .await
            .map_err(|_| DeviceError::RequestAdapter)?;

        debug!("Using adapter: {:?}", adapter.get_info());

        let (device, features) = Device::new(&adapter).await?;

        Ok((device, adapter, features))
    }

    pub async fn try_from_all_adapters(
        instance: &wgpu::Instance,
    ) -> Result<(Self, wgpu::Adapter, wgpu::Features), DeviceError> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let mut all_adapters = instance
                .enumerate_adapters(wgpu::Backends::all())
                .into_iter();

            let result = loop {
                let Some(adapter) = all_adapters.next() else {
                    debug!("No more adapters to try");
                    break None;
                };

                let (device, features) = match Device::new(&adapter).await {
                    Ok((device, adapter)) => (device, adapter),
                    Err(err) => {
                        error!(
                            "Failed to get device from adapter {:?}: {:?}",
                            adapter.get_info(),
                            err,
                        );
                        continue;
                    }
                };

                break Some((device, adapter, features));
            };

            for adapter in all_adapters {
                debug!(
                    "Remaining adapter that wasn't tried: {:?}",
                    adapter.get_info()
                );
            }

            result.ok_or(DeviceError::FoundNoWorkingAdapter)
        }
        #[cfg(target_arch = "wasm32")]
        {
            _ = instance;
            Err(DeviceError::FoundNoWorkingAdapter)
        }
    }

    pub async fn new(
        adapter: &wgpu::Adapter,
    ) -> Result<(Self, wgpu::Features), DeviceError> {
        let required_features = {
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

        let required_limits = {
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
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features,
                required_limits,

                // Here we give a memory hint to preserve memory usage.
                // This should allow us to run in as much devices as
                // possible.
                memory_hints: wgpu::MemoryHints::MemoryUsage,

                trace: wgpu::Trace::Off,
            })
            .await?;

        Ok((Device { device, queue }, required_features))
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

    /// Found no working adapter to get a device from
    #[error("Found no working adapter to get a device from")]
    FoundNoWorkingAdapter,
}
