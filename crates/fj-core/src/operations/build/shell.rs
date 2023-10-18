use fj_math::Point;

use crate::{
    objects::{Face, Shell},
    operations::{
        reverse::ReverseCurveCoordinateSystems, update::region::UpdateRegion,
        BuildFace, Insert, IsInserted, IsInsertedNo, IsInsertedYes, JoinCycle,
        Polygon, UpdateCycle, UpdateFace,
    },
    services::Services,
};

/// Build a [`Shell`]
pub trait BuildShell {
    /// Build an empty shell
    fn empty() -> Shell {
        Shell::new([])
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
        services: &mut Services,
    ) -> TetrahedronShell {
        let [a, b, c, d] = points.map(Into::into);

        let abc = Face::triangle([a, b, c], services);
        let bad = Face::triangle([b, a, d], services).update_region(|region| {
            region
                .update_exterior(|cycle| {
                    cycle
                        .update_half_edge(
                            cycle.half_edges().nth_circular(0),
                            |edge| {
                                edge.reverse_curve_coordinate_systems(services)
                                    .insert(services)
                            },
                        )
                        .join_to(
                            abc.face.region().exterior(),
                            0..=0,
                            0..=0,
                            services,
                        )
                        .insert(services)
                })
                .insert(services)
        });
        let dac = Face::triangle([d, a, c], services).update_region(|region| {
            region
                .update_exterior(|cycle| {
                    cycle
                        .update_half_edge(
                            cycle.half_edges().nth_circular(1),
                            |edge| {
                                edge.reverse_curve_coordinate_systems(services)
                                    .insert(services)
                            },
                        )
                        .join_to(
                            abc.face.region().exterior(),
                            1..=1,
                            2..=2,
                            services,
                        )
                        .update_half_edge(
                            cycle.half_edges().nth_circular(0),
                            |edge| {
                                edge.reverse_curve_coordinate_systems(services)
                                    .insert(services)
                            },
                        )
                        .join_to(
                            bad.face.region().exterior(),
                            0..=0,
                            1..=1,
                            services,
                        )
                        .insert(services)
                })
                .insert(services)
        });
        let cbd = Face::triangle([c, b, d], services).update_region(|region| {
            region
                .update_exterior(|cycle| {
                    cycle
                        .update_half_edge(
                            cycle.half_edges().nth_circular(0),
                            |edge| {
                                edge.reverse_curve_coordinate_systems(services)
                                    .insert(services)
                            },
                        )
                        .update_half_edge(
                            cycle.half_edges().nth_circular(1),
                            |edge| {
                                edge.reverse_curve_coordinate_systems(services)
                                    .insert(services)
                            },
                        )
                        .update_half_edge(
                            cycle.half_edges().nth_circular(2),
                            |edge| {
                                edge.reverse_curve_coordinate_systems(services)
                                    .insert(services)
                            },
                        )
                        .join_to(
                            abc.face.region().exterior(),
                            0..=0,
                            1..=1,
                            services,
                        )
                        .join_to(
                            bad.face.region().exterior(),
                            1..=1,
                            2..=2,
                            services,
                        )
                        .join_to(
                            dac.face.region().exterior(),
                            2..=2,
                            2..=2,
                            services,
                        )
                        .insert(services)
                })
                .insert(services)
        });

        let triangles =
            [abc, bad, dac, cbd].map(|triangle| triangle.insert(services));
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
