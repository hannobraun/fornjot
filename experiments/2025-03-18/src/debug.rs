use std::{ops::Deref, sync::Mutex};

use fj_interop::TriMesh;
use fj_math::Point;

use crate::{
    approx::face::FaceApproxPoints,
    viewer::{ViewerHandle, WindowHandle},
};

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
    pub fn display_face_surface(&self, points: Vec<Point<2>>) {
        let inner = self.inner.lock().unwrap();

        let DebugWindowInner::Initialized { window } = inner.deref() else {
            panic!("Debug window has not been initialized.");
        };

        window.display_face_surface(points);
    }

    #[allow(unused)] // occasionally useful for debugging
    pub fn display_face_global(&self, face: &FaceApproxPoints) {
        let inner = self.inner.lock().unwrap();

        let DebugWindowInner::Initialized { window } = inner.deref() else {
            panic!("Debug window has not been initialized.");
        };

        let points =
            face.points.iter().map(|point| point.point_global).collect();

        window.display_face_global(points);
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
    pub fn display_point_global(&self, point: Point<3>) {
        let inner = self.inner.lock().unwrap();

        let DebugWindowInner::Initialized { window } = inner.deref() else {
            panic!("Debug window has not been initialized.");
        };

        window.display_point_global(point);
    }

    #[allow(unused)] // occasionally useful for debugging
    pub fn display_point_surface(&self, point: Point<2>) {
        let inner = self.inner.lock().unwrap();

        let DebugWindowInner::Initialized { window } = inner.deref() else {
            panic!("Debug window has not been initialized.");
        };

        window.display_point_surface(point);
    }

    #[allow(unused)] // occasionally useful for debugging
    pub fn clear(&self) {
        let inner = self.inner.lock().unwrap();

        let DebugWindowInner::Initialized { window } = inner.deref() else {
            panic!("Debug window has not been initialized.");
        };

        window.clear();
    }
}

enum DebugWindowInner {
    Uninitialized,
    Initialized { window: WindowHandle },
}
