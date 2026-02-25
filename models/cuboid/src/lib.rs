use fj::core::{
    interop::{Color, MeshTriangle, TriMesh},
    math::{Point, Scalar, Vector},
    new::{
        geometry::{Line, Plane},
        operations::{Sketch, Sweep},
        topology::Topology,
    },
    operations::{
        build::{BuildRegion, BuildSketch},
        sweep::SweepSketch,
        update::UpdateSketch,
    },
    topology::{Region, Sketch as SketchOld, Solid},
};

pub fn model(size: impl Into<Vector<3>>) -> TriMesh {
    let [x, y, z] = size.into().components;

    let Topology {
        mut faces,
        mut half_edges,
        mut solids,
        mut vertices,
    } = Topology::new();

    let mut sweep = Sweep::new();

    let bottom = Sketch::new()
        .line_to([-x / 2., -y / 2.])
        .line_to([x / 2., -y / 2.])
        .line_to([x / 2., y / 2.])
        .line_to([-x / 2., y / 2.])
        .into_face(
            Plane {
                origin: Point::from([0., 0., 0.]),
                axes: [Vector::from([0., 1., 0.]), Vector::from([1., 0., 0.])],
            },
            &mut vertices,
            &mut half_edges,
            &mut faces,
        );

    let cuboid = sweep.face_to_solid(
        bottom,
        &Line::to([Scalar::ZERO, Scalar::ZERO, z]),
        &mut vertices,
        &mut half_edges,
        &mut faces,
        &mut solids,
    );

    let triangles = solids[cuboid]
        .boundary
        .iter()
        .flat_map(|&face| &faces[face].approx);

    let mut tri_mesh = TriMesh::new();

    for &triangle in triangles {
        tri_mesh.triangles.push(MeshTriangle {
            inner: triangle,
            is_internal: false,
            color: Color::default(),
        });
    }

    tri_mesh
}

pub fn model_old(
    size: impl Into<Vector<3>>,
    core: &mut fj::core::Core,
) -> Solid {
    let [x, y, z] = size.into().components;

    let bottom_surface = core.layers.topology.surfaces.xy_plane();
    let sweep_path = Vector::from([Scalar::ZERO, Scalar::ZERO, -z]);

    SketchOld::empty(&core.layers.topology)
        .add_regions(
            [Region::polygon(
                [
                    [-x / 2., -y / 2.],
                    [x / 2., -y / 2.],
                    [x / 2., y / 2.],
                    [-x / 2., y / 2.],
                ],
                core.layers.topology.surfaces.space_2d(),
                core,
            )],
            core,
        )
        .sweep_sketch(bottom_surface, sweep_path, core)
}
