use crate::new::{
    geometry::Plane,
    topology::{Face, HalfFace, Orientation, Topology},
};

/// # A new sketch operation
#[derive(Default)]
pub struct Sketch2 {}

impl Sketch2 {
    /// # Construct an empty sketch
    pub fn new() -> Self {
        Self::default()
    }

    /// # Convert the sketch into a half-face
    ///
    /// A sketch is purely a 2-dimensional construct, with no notion of where
    /// that sketch might be located in 3D space. In calling this method, the
    /// caller provides the surface on which the sketch is to be located,
    /// enabling its conversion into a half-face.
    pub fn into_half_face(
        self,
        surface: Plane,
        topology: &mut Topology,
    ) -> HalfFace {
        let _ = surface;

        let face = topology.faces.push(Face { approx: Vec::new() });

        HalfFace {
            boundary: Vec::new(),
            face,
            orientation: Orientation::Nominal,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::new::{
        geometry::Plane,
        operations::Sketch2,
        topology::{Face, Orientation, Topology},
    };

    #[test]
    fn empty() {
        let mut topology = Topology::new();

        let half_face =
            Sketch2::new().into_half_face(Plane::xy(), &mut topology);

        assert_eq!(half_face.boundary, Vec::new());
        assert_eq!(topology.faces[half_face.face], Face { approx: Vec::new() });
        assert_eq!(half_face.orientation, Orientation::Nominal);
    }
}
