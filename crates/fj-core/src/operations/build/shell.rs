use std::collections::BTreeMap;

use fj_interop::ext::ArrayExt;
use fj_math::Point;

use crate::{
    geometry::{CurveBoundary, HalfEdgeGeom},
    operations::{
        build::{BuildFace, BuildHalfEdge, BuildSurface, Polygon},
        geometry::{UpdateCurveGeometry, UpdateHalfEdgeGeometry},
        insert::{Insert, IsInserted, IsInsertedNo, IsInsertedYes},
        join::JoinCycle,
        reverse::ReverseCurveCoordinateSystems,
        update::{
            UpdateCycle, UpdateFace, UpdateHalfEdge, UpdateRegion, UpdateShell,
        },
    },
    topology::{Curve, Face, HalfEdge, Shell, Surface, Vertex},
    Core,
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
        core: &mut Core,
    ) -> Shell {
        let vertices = vertices
            .into_iter()
            .enumerate()
            .map(|(index, position)| {
                let vertex = Vertex::new().insert(core);
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
                    core,
                );

                let curves_and_boundaries =
                    [[a, b], [b, c], [c, a]].map(|vertices| {
                        let vertices = vertices.map(Clone::clone);
                        let vertices = CurveBoundary::<Vertex>::from(vertices);

                        curves
                            .get(&vertices.clone().reverse())
                            .cloned()
                            .unwrap_or_else(|| {
                                let curve = Curve::new().insert(core);
                                let boundary = CurveBoundary::default();

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
                            let boundary = boundary.reverse();

                            let curve = curve.make_line_on_surface(
                                positions,
                                boundary,
                                surface.clone(),
                                &mut core.layers.geometry,
                            );

                            HalfEdge::unjoined(core)
                                .update_start_vertex(|_, _| vertex, core)
                                .update_curve(|_, _| curve.clone(), core)
                                .insert(core)
                                .set_geometry(
                                    HalfEdgeGeom { boundary },
                                    &mut core.layers.geometry,
                                )
                        })
                };

                Face::unbound(surface, core).update_region(
                    |region, core| {
                        region.update_exterior(
                            |cycle, core| {
                                cycle.add_half_edges(half_edges, core)
                            },
                            core,
                        )
                    },
                    core,
                )
            })
            .collect::<Vec<_>>();

        Shell::empty().add_faces(faces, core)
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
        core: &mut Core,
    ) -> TetrahedronShell {
        let [a, b, c, d] = points.map(Into::into);

        let abc = Face::triangle([a, b, c], core);
        let bad = {
            let bad = Face::triangle([b, a, d], core);
            bad.update_region(
                |region, core| {
                    region.update_exterior(
                        |cycle, core| {
                            cycle
                                .update_half_edge(
                                    cycle.half_edges().nth_circular(0),
                                    |edge, core| {
                                        [(edge, bad.face.surface())
                                            .reverse_curve_coordinate_systems(
                                                core,
                                            )]
                                    },
                                    core,
                                )
                                .join_to(
                                    abc.face.region().exterior(),
                                    0..=0,
                                    0..=0,
                                    bad.face.surface().clone(),
                                    core,
                                )
                        },
                        core,
                    )
                },
                core,
            )
        };
        let dac =
            {
                let dac = Face::triangle([d, a, c], core);
                dac.update_region(
                    |region, core| {
                        region.update_exterior(
                            |cycle, core| {
                                cycle
                                    .update_half_edge(
                                        cycle.half_edges().nth_circular(1),
                                        |edge, core| {
                                            [(edge, dac.face.surface())
                                        .reverse_curve_coordinate_systems(core)]
                                        },
                                        core,
                                    )
                                    .join_to(
                                        abc.face.region().exterior(),
                                        1..=1,
                                        2..=2,
                                        dac.face.surface().clone(),
                                        core,
                                    )
                                    .update_half_edge(
                                        cycle.half_edges().nth_circular(0),
                                        |edge, core| {
                                            [(edge, dac.face.surface())
                                        .reverse_curve_coordinate_systems(core)]
                                        },
                                        core,
                                    )
                                    .join_to(
                                        bad.face.region().exterior(),
                                        0..=0,
                                        1..=1,
                                        dac.face.surface().clone(),
                                        core,
                                    )
                            },
                            core,
                        )
                    },
                    core,
                )
            };
        let cbd =
            {
                let cbd = Face::triangle([c, b, d], core);
                cbd.update_region(
                    |region, core| {
                        region.update_exterior(
                            |cycle, core| {
                                cycle
                                    .update_half_edge(
                                        cycle.half_edges().nth_circular(0),
                                        |edge, core| {
                                            [(edge, cbd.face.surface())
                                        .reverse_curve_coordinate_systems(core)]
                                        },
                                        core,
                                    )
                                    .update_half_edge(
                                        cycle.half_edges().nth_circular(1),
                                        |edge, core| {
                                            [(edge, cbd.face.surface())
                                        .reverse_curve_coordinate_systems(core)]
                                        },
                                        core,
                                    )
                                    .update_half_edge(
                                        cycle.half_edges().nth_circular(2),
                                        |edge, core| {
                                            [(edge, cbd.face.surface())
                                        .reverse_curve_coordinate_systems(core)]
                                        },
                                        core,
                                    )
                                    .join_to(
                                        abc.face.region().exterior(),
                                        0..=0,
                                        1..=1,
                                        cbd.face.surface().clone(),
                                        core,
                                    )
                                    .join_to(
                                        bad.face.region().exterior(),
                                        1..=1,
                                        2..=2,
                                        cbd.face.surface().clone(),
                                        core,
                                    )
                                    .join_to(
                                        dac.face.region().exterior(),
                                        2..=2,
                                        2..=2,
                                        cbd.face.surface().clone(),
                                        core,
                                    )
                            },
                            core,
                        )
                    },
                    core,
                )
            };

        let triangles =
            [abc, bad, dac, cbd].map(|triangle| triangle.insert(core));
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
