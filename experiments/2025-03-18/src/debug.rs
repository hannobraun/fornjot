use std::{ops::Deref, sync::Mutex};

use fj_interop::TriMesh;
use fj_math::Point;
use fj_viewer::{ViewerHandle, WindowHandle};

pub static DEBUG_WINDOW: DebugWindow = DebugWindow::new();

pub struct DebugWindow {
    inner: Mutex<DebugWindowInner>,
}

impl DebugWindow {
    pub const fn new() -> Self {
        Self {
            inner: Mutex::new(DebugWindowInner::Uninitialized),
        }
    }

    pub fn initialize(&self, viewer: &ViewerHandle) {
        let window = viewer.open_window();

        let mut inner = self.inner.lock().unwrap();
        *inner = DebugWindowInner::Initialized { window };
    }

    #[allow(unused)] // occasionally useful for debugging
    pub fn display_face(&self, points: Vec<Point<2>>) {
        let inner = self.inner.lock().unwrap();

        let DebugWindowInner::Initialized { window } = inner.deref() else {
            panic!("Debug window has not been initialized.");
        };

        window.display_face(points);
    }

    #[allow(unused)] // occasionally useful for debugging
    pub fn display_mesh(&self, tri_mesh: TriMesh) {
        let inner = self.inner.lock().unwrap();

        let DebugWindowInner::Initialized { window } = inner.deref() else {
            panic!("Debug window has not been initialized.");
        };

        window.display_mesh(tri_mesh);
    }

    #[allow(unused)] // occasionally useful for debugging
    pub fn display_point(&self, point: Point<3>) {
        let inner = self.inner.lock().unwrap();

        let DebugWindowInner::Initialized { window } = inner.deref() else {
            panic!("Debug window has not been initialized.");
        };

        window.display_point(point);
    }
}

enum DebugWindowInner {
    Uninitialized,
    Initialized { window: WindowHandle },
}
