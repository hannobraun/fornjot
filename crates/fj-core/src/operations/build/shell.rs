use std::collections::BTreeMap;

use fj_interop::ext::ArrayExt;
use fj_math::Point;

use crate::{
    geometry::CurveBoundary,
    objects::{Curve, Face, HalfEdge, Shell, Surface, Vertex},
    operations::{
        build::{BuildFace, BuildHalfEdge, BuildSurface, Polygon},
        insert::{Insert, IsInserted, IsInsertedNo, IsInsertedYes},
        join::JoinCycle,
        reverse::ReverseCurveCoordinateSystems,
        update::{
            UpdateCycle, UpdateFace, UpdateHalfEdge, UpdateRegion, UpdateShell,
        },
    },
    Instance,
};

/// Build a [`Shell`]
///
/// See [module-level documentation] for context.
///
/// [module-level documentation]: super
pub trait BuildShell {
    /// Build an empty shell
    fn empty() -> Shell {
        Shell::new([])
    }

    /// Build a polyhedron by specifying its vertices and indices
    fn from_vertices_and_indices(
        vertices: impl IntoIterator<Item = impl Into<Point<3>>>,
        indices: impl IntoIterator<Item = [usize; 3]>,
        core: &mut Instance,
    ) -> Shell {
        let vertices = vertices
            .into_iter()
            .enumerate()
            .map(|(index, position)| {
                let vertex = Vertex::new().insert(&mut core.services);
                let position = position.into();

                (index, (vertex, position))
            })
            .collect::<BTreeMap<_, _>>();

        let mut curves = BTreeMap::new();

        let faces = indices
            .into_iter()
            .map(|indices| {
                let [(a, a_pos), (b, b_pos), (c, c_pos)] = indices
                    .map(|index| vertices.get(&index).expect("Invalid index"));

                let (surface, _) = Surface::plane_from_points(
                    [a_pos, b_pos, c_pos].map(Clone::clone),
                );
                let surface = surface.insert(&mut core.services);

                let curves_and_boundaries =
                    [[a, b], [b, c], [c, a]].map(|vertices| {
                        let vertices = vertices.map(Clone::clone);
                        let vertices = CurveBoundary::<Vertex>::from(vertices);

                        curves
                            .get(&vertices.clone().reverse())
                            .cloned()
                            .unwrap_or_else(|| {
                                let curve =
                                    Curve::new().insert(&mut core.services);
                                let boundary =
                                    CurveBoundary::<Point<1>>::from([
                                        [0.],
                                        [1.],
                                    ]);

                                curves.insert(
                                    vertices,
                                    (curve.clone(), boundary),
                                );

                                (curve, boundary.reverse())
                            })
                    });

                let half_edges = {
                    let vertices = [a, b, c].map(Clone::clone);
                    let [a, b, c] = [[0., 0.], [1., 0.], [0., 1.]];
                    vertices
                        .zip_ext([[a, b], [b, c], [c, a]])
                        .zip_ext(curves_and_boundaries)
                        .map(|((vertex, positions), (curve, boundary))| {
                            HalfEdge::line_segment(
                                positions,
                                Some(boundary.reverse().inner),
                                core,
                            )
                            .update_start_vertex(|_| vertex)
                            .update_curve(|_| curve)
                            .insert(&mut core.services)
                        })
                };

                Face::unbound(surface, core)
                    .update_region(|region| {
                        region
                            .update_exterior(|cycle| {
                                cycle
                                    .add_half_edges(half_edges)
                                    .insert(&mut core.services)
                            })
                            .insert(&mut core.services)
                    })
                    .insert(&mut core.services)
            })
            .collect::<Vec<_>>();

        Shell::empty().add_faces(faces)
    }

    /// Build a tetrahedron from the provided points
    ///
    /// Accepts 4 points, naturally. For the purposes of the following
    /// discussion, let's call those `a`, `b`, `c`, and `d`, and assume that the
    /// order they are listed in here matches the order they are provided in
    /// within the array.
    ///
    /// Assumes that `a`, `b`, and `c` form a triangle in counter-clockwise
    /// order, when arranging the viewpoint such that it is on the opposite side
    /// of the triangle from `d`. If this assumption is met, the orientation of
    /// all faces of the tetrahedron will be valid, meaning their
    /// counter-clockwise sides are outside.
    ///
    /// # Implementation Note
    ///
    /// In principle, this method doesn't need to make assumptions about the
    /// order of the points provided. It could, given some extra effort, just
    /// build a correct tetrahedron, regardless of that order.
    fn tetrahedron(
        points: [impl Into<Point<3>>; 4],
        core: &mut Instance,
    ) -> TetrahedronShell {
        let [a, b, c, d] = points.map(Into::into);

        let abc = Face::triangle([a, b, c], core);
        let bad = Face::triangle([b, a, d], core).update_region(|region| {
            region
                .update_exterior(|cycle| {
                    cycle
                        .update_half_edge(
                            cycle.half_edges().nth_circular(0),
                            |edge| {
                                edge.reverse_curve_coordinate_systems(
                                    &mut core.services,
                                )
                                .insert(&mut core.services)
                            },
                        )
                        .join_to(
                            abc.face.region().exterior(),
                            0..=0,
                            0..=0,
                            &mut core.services,
                        )
                        .insert(&mut core.services)
                })
                .insert(&mut core.services)
        });
        let dac = Face::triangle([d, a, c], core).update_region(|region| {
            region
                .update_exterior(|cycle| {
                    cycle
                        .update_half_edge(
                            cycle.half_edges().nth_circular(1),
                            |edge| {
                                edge.reverse_curve_coordinate_systems(
                                    &mut core.services,
                                )
                                .insert(&mut core.services)
                            },
                        )
                        .join_to(
                            abc.face.region().exterior(),
                            1..=1,
                            2..=2,
                            &mut core.services,
                        )
                        .update_half_edge(
                            cycle.half_edges().nth_circular(0),
                            |edge| {
                                edge.reverse_curve_coordinate_systems(
                                    &mut core.services,
                                )
                                .insert(&mut core.services)
                            },
                        )
                        .join_to(
                            bad.face.region().exterior(),
                            0..=0,
                            1..=1,
                            &mut core.services,
                        )
                        .insert(&mut core.services)
                })
                .insert(&mut core.services)
        });
        let cbd = Face::triangle([c, b, d], core).update_region(|region| {
            region
                .update_exterior(|cycle| {
                    cycle
                        .update_half_edge(
                            cycle.half_edges().nth_circular(0),
                            |edge| {
                                edge.reverse_curve_coordinate_systems(
                                    &mut core.services,
                                )
                                .insert(&mut core.services)
                            },
                        )
                        .update_half_edge(
                            cycle.half_edges().nth_circular(1),
                            |edge| {
                                edge.reverse_curve_coordinate_systems(
                                    &mut core.services,
                                )
                                .insert(&mut core.services)
                            },
                        )
                        .update_half_edge(
                            cycle.half_edges().nth_circular(2),
                            |edge| {
                                edge.reverse_curve_coordinate_systems(
                                    &mut core.services,
                                )
                                .insert(&mut core.services)
                            },
                        )
                        .join_to(
                            abc.face.region().exterior(),
                            0..=0,
                            1..=1,
                            &mut core.services,
                        )
                        .join_to(
                            bad.face.region().exterior(),
                            1..=1,
                            2..=2,
                            &mut core.services,
                        )
                        .join_to(
                            dac.face.region().exterior(),
                            2..=2,
                            2..=2,
                            &mut core.services,
                        )
                        .insert(&mut core.services)
                })
                .insert(&mut core.services)
        });

        let triangles = [abc, bad, dac, cbd]
            .map(|triangle| triangle.insert(&mut core.services));
        let shell =
            Shell::new(triangles.iter().map(|triangle| triangle.face.clone()));

        let [abc, bad, dac, cbd] = triangles;

        TetrahedronShell {
            shell,
            abc,
            bad,
            dac,
            cbd,
        }
    }
}

impl BuildShell for Shell {}

/// A tetrahedron
///
/// A tetrahedron is constructed from 4 points and has 4 faces. For the purpose
/// of naming the fields of this struct, the points are named `a`, `b`, `c`, and
/// `d`, in the order in which they are passed.
///
/// Returned by [`BuildShell::tetrahedron`].
pub struct TetrahedronShell<I: IsInserted = IsInsertedNo> {
    /// The shell that forms the tetrahedron
    pub shell: I::T<Shell>,

    /// The face formed by the points `a`, `b`, and `c`.
    pub abc: Polygon<3, IsInsertedYes>,

    /// The face formed by the points `b`, `a`, and `d`.
    pub bad: Polygon<3, IsInsertedYes>,

    /// The face formed by the points `d`, `a`, and `c`.
    pub dac: Polygon<3, IsInsertedYes>,

    /// The face formed by the points `c`, `b`, and `d`.
    pub cbd: Polygon<3, IsInsertedYes>,
}
