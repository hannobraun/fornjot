use std::sync::Arc;

use winit::event_loop::ActiveEventLoop;

/// A window that can be used with `fj-viewer`
pub struct Window {
    inner: Arc<winit::window::Window>,
}

impl Window {
    /// Create an instance of `Window` from the given `EventLoop`
    pub fn new(event_loop: &ActiveEventLoop) -> Result<Self, WindowError> {
        let window = event_loop.create_window(
            winit::window::Window::default_attributes()
                .with_title("Fornjot")
                .with_maximized(true)
                // When the window decorations are enabled, I'm seeing the
                // following error on Gnome/Wayland, in response to a
                // `ScaleFactorChange` event:
                // ```
                // wl_surface@24: error 2: Buffer size (1940x45) must be an integer multiple of the buffer_scale (2).
                // ```
                //
                // This is happening most of the time. Very rarely, the window
                // will open as expected.
                //
                // I believe that there is a race condition somewhere low in the
                // stack, that will cause the buffer size for the window
                // decorations to not be updated before the check that produces
                // the above error. I failed to track down where this is
                // happening, so I decided to deploy this workaround instead of
                // spending more time.
                //
                // Window decorations should be re-enabled once possible. This
                // is being tracked in this issue:
                // https://github.com/hannobraun/fornjot/issues/1848
                .with_decorations(false)
                .with_transparent(false),
        )?;

        Ok(Self {
            inner: Arc::new(window),
        })
    }

    pub fn size(&self) -> ScreenSize {
        let size = self.inner.inner_size();

        ScreenSize {
            width: size.width,
            height: size.height,
        }
    }

    pub fn winit_window(&self) -> Arc<winit::window::Window> {
        self.inner.clone()
    }
}

/// Error initializing window
#[derive(Debug, thiserror::Error)]
#[error("Error initializing window")]
pub struct WindowError(#[from] pub winit::error::OsError);

/// Cursor position in normalized coordinates (-1 to +1)
///
/// The center of the screen is at (0, 0). The aspect ratio is taken into
/// account.
#[derive(Clone, Copy, Debug)]
pub struct NormalizedScreenPosition {
    /// The x coordinate of the position [-1, 1]
    pub x: f64,

    /// The y coordinate of the position [-1, 1]
    pub y: f64,
}

/// The size of the screen
#[derive(Clone, Copy, Debug)]
pub struct ScreenSize {
    /// The width of the screen
    pub width: u32,

    /// The height of the screen
    pub height: u32,
}

impl ScreenSize {
    /// # Indicate whether the screen size is valid
    ///
    /// A screen size is valid, if neither of its dimensions is zero. But it can
    /// be reported as zero by spurious screen resize events.
    pub fn is_valid(&self) -> bool {
        self.width > 0 && self.height > 0
    }

    /// Convert size to `f64`
    pub fn as_f64(&self) -> [f64; 2] {
        [self.width, self.height].map(Into::into)
    }
}
