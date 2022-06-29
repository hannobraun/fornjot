use crate::{
    objects::{Cycle, CyclesInFace, Edge, Face},
    shape::LocalForm,
};

/// Reverse the direction of a face
pub fn reverse_face(face: &Face) -> Face {
    let face = match face {
        Face::Face(face) => face,
        Face::Triangles(_) => {
            panic!("Reversing tri-rep faces is not supported")
        }
    };

    let surface = face.surface().reverse();

    let exteriors = reverse_local_coordinates_in_cycle(&face.exteriors);
    let interiors = reverse_local_coordinates_in_cycle(&face.interiors);

    Face::new(
        surface,
        exteriors.as_local_form().cloned(),
        interiors.as_local_form().cloned(),
        face.color,
    )
}

fn reverse_local_coordinates_in_cycle(cycles: &CyclesInFace) -> CyclesInFace {
    let cycles = cycles.as_local_form().map(|cycle| {
        let edges = cycle
            .local()
            .edges
            .iter()
            .map(|edge| {
                let curve = LocalForm::new(
                    // This is wrong. We have reversed the direction of the
                    // surface, thereby modifying its coordinate system. So we
                    // can't just use the local form of the curve, which is
                    // expressed in surface coordinates, as-is.
                    //
                    // This is a coherence issue, but since coherence validation
                    // is not complete, and the whole local form stuff is still
                    // a work in progress, this doesn't lead to any observable
                    // bugs.
                    *edge.local().curve.local(),
                    *edge.local().curve.canonical(),
                );
                let vertices = edge.local().vertices.clone().map(|vertex| {
                    LocalForm::new(*vertex.local(), *vertex.canonical())
                });
                let local = Edge { curve, vertices };
                LocalForm::new(local, edge.canonical().clone())
            })
            .collect();
        let local = Cycle { edges };
        LocalForm::new(local, cycle.canonical().clone())
    });

    CyclesInFace::new(cycles)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::objects::{Face, Surface};

    #[test]
    fn reverse_face() {
        let original = Face::builder(Surface::xy_plane())
            .with_exterior_polygon([[0., 0.], [1., 0.], [0., 1.]])
            .build();

        let reversed = super::reverse_face(&original);

        let expected = Face::builder(Surface::xy_plane().reverse())
            .with_exterior_polygon([[0., 0.], [1., 0.], [0., -1.]])
            .build();

        assert_eq!(expected, reversed);
    }
}
