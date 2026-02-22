use fj_math::Point;
use itertools::Itertools;

use crate::{
    Core,
    geometry::LocalVertexGeom,
    operations::{
        build::{BuildCycle, BuildHalfEdge},
        derive::DeriveFrom,
        insert::Insert,
        split::SplitEdge,
        update::{
            UpdateCycle, UpdateFace, UpdateHalfEdge, UpdateRegion, UpdateShell,
        },
    },
    storage::Handle,
    topology::{Cycle, Face, HalfEdge, Shell},
};

/// Split a face into two
pub trait SplitFace: Sized {
    /// Split the face into two
    ///
    /// The line that splits the face is defined by two points, each specified
    /// in local coordinates of an edge.
    ///
    /// # Panics
    ///
    /// Panics, if the half-edges are not part of the boundary of the provided
    /// face.
    ///
    /// # Implementation Note
    ///
    /// The way the split line is specified is rather inconvenient, and not very
    /// flexible. This is an artifact of the current implementation, and more
    /// flexible and convenient ways to split the face (like an arbitrary curve)
    /// can be provided later.
    #[must_use]
    fn split_face(
        &self,
        face: &Handle<Face>,
        line: [(&Handle<HalfEdge>, impl Into<Point<1>>); 2],
        core: &mut Core,
    ) -> (Self, [Handle<Face>; 2]);
}

impl SplitFace for Shell {
    fn split_face(
        &self,
        face: &Handle<Face>,
        line: [(&Handle<HalfEdge>, impl Into<Point<1>>); 2],
        core: &mut Core,
    ) -> (Self, [Handle<Face>; 2]) {
        // The code below might assume that the half-edges that define the line
        // are part of the face's exterior. Let's make that explicit here.
        //
        // This is actually the only time we're using `face` in this method, as
        // it's going to get replaced with a new version as soon as we split the
        // edges. We could probably do without it, but not taking it would
        // probably make validating that both half-edges belong to the same face
        // more difficult, as well as make the method signature less intuitive.
        //
        // Something to think about though!
        {
            let [(a, _), (b, _)] = line.each_ref();

            let exterior = face.region().exterior();

            assert!(exterior.half_edges().contains(a));
            assert!(exterior.half_edges().contains(b));
        }

        let mut self_ = self.clone();

        let [[a, b], [c, d]] = line.map(|(half_edge, point)| {
            let (shell, [[a, b], _]) = self_.split_edge(half_edge, point, core);
            self_ = shell;
            [a, b]
        });

        // The original face doesn't exist in the updated shell, as it's been
        // replaced by a new version due to the edge splitting. Let's find the
        // face that replaced it.
        let mut updated_face_after_split_edges = None;
        for f in self_.faces() {
            let half_edges = f.region().exterior().half_edges();

            if half_edges.contains(&a)
                && half_edges.contains(&b)
                && half_edges.contains(&c)
                && half_edges.contains(&d)
            {
                assert!(
                    updated_face_after_split_edges.is_none(),
                    "There should never be two faces that share half-edges"
                );
                updated_face_after_split_edges = Some(f);
            }
        }
        let updated_face_after_split_edges = updated_face_after_split_edges
            .expect("Updated shell must contain updated face");

        // Build the edge that's going to divide the new faces.
        let dividing_half_edge_a_to_d = {
            let start = core
                .layers
                .geometry
                .of_curve(b.curve())
                .unwrap()
                .local_on(face.surface())
                .unwrap()
                .path
                .point_from_path_coords(
                    core.layers
                        .geometry
                        .of_vertex(b.start_vertex())
                        .unwrap()
                        .local_on(b.curve())
                        .unwrap()
                        .position,
                );
            let end = core
                .layers
                .geometry
                .of_curve(d.curve())
                .unwrap()
                .local_on(face.surface())
                .unwrap()
                .path
                .point_from_path_coords(
                    core.layers
                        .geometry
                        .of_vertex(d.start_vertex())
                        .unwrap()
                        .local_on(d.curve())
                        .unwrap()
                        .position,
                );

            let (half_edge, boundary) = HalfEdge::line_segment(
                [start, end],
                face.surface().clone(),
                core,
            );

            core.layers.geometry.define_vertex(
                b.start_vertex().clone(),
                half_edge.curve().clone(),
                LocalVertexGeom {
                    position: boundary.inner[0],
                },
            );
            core.layers.geometry.define_vertex(
                d.start_vertex().clone(),
                half_edge.curve().clone(),
                LocalVertexGeom {
                    position: boundary.inner[1],
                },
            );

            half_edge
                .update_start_vertex(|_, _| b.start_vertex().clone(), core)
                .insert(core)
        };
        let dividing_half_edge_c_to_b = HalfEdge::from_sibling(
            &dividing_half_edge_a_to_d,
            d.start_vertex().clone(),
            core,
        );

        let mut half_edges_of_face_starting_at_b =
            updated_face_after_split_edges
                .region()
                .exterior()
                .half_edges()
                .iter()
                .cloned()
                .cycle()
                .skip_while(|half_edge| half_edge != &b);

        let half_edges_b_to_c_inclusive = half_edges_of_face_starting_at_b
            .take_while_ref(|half_edge| half_edge != &d);
        let split_face_a = updated_face_after_split_edges
            .update_region(
                |region, core| {
                    region.update_exterior(
                        |_, core| {
                            Cycle::empty()
                                .add_half_edges(
                                    half_edges_b_to_c_inclusive,
                                    core,
                                )
                                .add_half_edges(
                                    [dividing_half_edge_c_to_b],
                                    core,
                                )
                        },
                        core,
                    )
                },
                core,
            )
            .insert(core)
            .derive_from(updated_face_after_split_edges, core);

        // The previous operation has moved the iterator along.
        let half_edges_of_face_starting_at_d = half_edges_of_face_starting_at_b;

        let half_edges_d_to_a_inclusive = half_edges_of_face_starting_at_d
            .take_while(|half_edge| half_edge != &b);
        let split_face_b = updated_face_after_split_edges
            .update_region(
                |region, core| {
                    region.update_exterior(
                        |_, core| {
                            Cycle::empty()
                                .add_half_edges(
                                    half_edges_d_to_a_inclusive,
                                    core,
                                )
                                .add_half_edges(
                                    [dividing_half_edge_a_to_d],
                                    core,
                                )
                        },
                        core,
                    )
                },
                core,
            )
            .insert(core)
            .derive_from(updated_face_after_split_edges, core);

        let faces = [split_face_a, split_face_b];
        let self_ = self_.update_face(
            updated_face_after_split_edges,
            |_, _| faces.clone(),
            core,
        );

        (self_, faces)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Core,
        interop::Color,
        operations::{
            build::BuildShell,
            presentation::{GetColor, SetColor},
            split::SplitFace,
        },
        topology::Shell,
    };

    #[test]
    fn split_face_should_keep_color() {
        let mut core = Core::new();

        let tetrahedron = Shell::tetrahedron(
            [[0., 0., 0.], [1., 0., 0.], [0., 1., 0.], [0., 0., 1.]],
            &mut core,
        );
        let triangle = tetrahedron.abc;

        let color = Color::default();
        triangle.face.region().set_color(color, &mut core);

        let split_line = [
            (&triangle.half_edges[0], [0.5]),
            (&triangle.half_edges[1], [0.5]),
        ];
        let (_shell, [face_a, face_b]) =
            tetrahedron
                .shell
                .split_face(&triangle.face, split_line, &mut core);

        assert_eq!(face_a.region().get_color(&mut core), Some(color));
        assert_eq!(face_b.region().get_color(&mut core), Some(color));
    }
}
