use parry3d_f64::query::Ray;

/// Debug info from the CAD kernel that can be visualized
///
/// At this point, this is a placeholder that will be filled with life later.
pub struct DebugInfo {
    /// Rays being used during face triangulation
    pub triangle_edge_checks: Vec<TriangleEdgeCheck>,
}

impl DebugInfo {
    pub fn new() -> Self {
        Self {
            triangle_edge_checks: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.triangle_edge_checks.clear();
    }
}

/// Record of a check to determine if a triangle edge is within a face
pub struct TriangleEdgeCheck {
    pub ray: Ray,
}
