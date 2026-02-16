use std::{ops::Deref, sync::Mutex};

use crate::{
    interop::TriMesh,
    math::Point,
    viewer::{ViewerHandle, WindowHandle},
};

pub static DEBUG_WINDOW: DebugWindow = DebugWindow::new();

pub struct DebugWindow {
    mutex: DebugWindowMutex,
}

impl DebugWindow {
    pub const fn new() -> Self {
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

    #[allow(unused)] // occasionally useful for debugging
    pub fn display_mesh(&self, tri_mesh: TriMesh) {
        let inner = self.mutex.inner.lock().unwrap();
        let window = inner.deref().expect_initialized();

        window.display_mesh(tri_mesh);
    }

    #[allow(unused)] // occasionally useful for debugging
    pub fn display_point_surface(&self, point: Point<2>) {
        let inner = self.mutex.inner.lock().unwrap();
        let window = inner.deref().expect_initialized();

        window.display_point_surface(point);
    }

    #[allow(unused)] // occasionally useful for debugging
    pub fn display_point_global(&self, point: Point<3>) {
        let inner = self.mutex.inner.lock().unwrap();
        let window = inner.deref().expect_initialized();

        window.display_point_global(point);
    }

    #[allow(unused)] // occasionally useful for debugging
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
            panic!("Debug window has not been initialized.");
        };

        window
    }
}
