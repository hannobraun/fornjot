use parry3d_f64::query::Ray;

/// Debug info from the CAD kernel that can be visualized
///
/// At this point, this is a placeholder that will be filled with life later.
pub struct DebugInfo {
    /// Rays being used during face triangulation
    pub rays: Vec<Ray>,
}

impl DebugInfo {
    pub fn new() -> Self {
        Self { rays: Vec::new() }
    }
}
