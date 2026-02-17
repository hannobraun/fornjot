use std::{ops::Deref, sync::Mutex};

use crate::{
    interop::TriMesh,
    math::Point,
    viewer::{ViewerHandle, WindowHandle},
};

/// # Provides easy access to a window, for debugging purposes
///
/// This gives you an easy window that you can just display stuff in, accessible
/// from anywhere. If you want to use Fornjot's viewer to display geometry as a
/// regular part of your code's function, you should not use this, and pass a
/// [`ViewerHandle`] or [`WindowHandle`] instead.
///
/// But if you need to quickly display something, somewhere in the depths of
/// your code, without wanting to rewrite your whole call chain, this is the
/// right tool for the job.
///
/// The debug window is initially uninitialized. You must call
/// [`DebugWindow::initialize`] before calling any other methods on this
/// instance.
pub static DEBUG_WINDOW: DebugWindow = DebugWindow::new();

/// # Provides easy access to a window, for debugging purposes
///
/// You can't construct this type yourself. Please use it via [`DEBUG_WINDOW`].
pub struct DebugWindow {
    mutex: DebugWindowMutex,
}

impl DebugWindow {
    const fn new() -> Self {
        Self {
            mutex: DebugWindowMutex {
                inner: Mutex::new(DebugWindowState::Uninitialized),
            },
        }
    }

    pub fn initialize(&self, viewer: &ViewerHandle) {
        let window = viewer.open_window();

        let mut inner = self.mutex.inner.lock().unwrap();
        *inner = DebugWindowState::Initialized { window };
    }

    pub fn display_mesh(&self, tri_mesh: TriMesh) {
        let inner = self.mutex.inner.lock().unwrap();
        let window = inner.deref().expect_initialized();

        window.display_mesh(tri_mesh);
    }

    pub fn display_point_surface(&self, point: Point<2>) {
        let inner = self.mutex.inner.lock().unwrap();
        let window = inner.deref().expect_initialized();

        window.display_point_surface(point);
    }

    pub fn display_point_global(&self, point: Point<3>) {
        let inner = self.mutex.inner.lock().unwrap();
        let window = inner.deref().expect_initialized();

        window.display_point_global(point);
    }

    pub fn clear(&self) {
        let inner = self.mutex.inner.lock().unwrap();
        let window = inner.deref().expect_initialized();

        window.clear();
    }
}

struct DebugWindowMutex {
    inner: Mutex<DebugWindowState>,
}

enum DebugWindowState {
    Uninitialized,
    Initialized { window: WindowHandle },
}

impl DebugWindowState {
    pub fn expect_initialized(&self) -> &WindowHandle {
        let DebugWindowState::Initialized { window } = self else {
            panic!(
                "You must call `DebugWindow::initialize` before calling any of \
                its other methods.",
            );
        };

        window
    }
}
