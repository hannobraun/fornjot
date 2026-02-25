use fj::core::{
    interop::{Color, MeshTriangle, TriMesh},
    math::{Point, Scalar, Vector},
    new::{
        geometry::{Line, Plane},
        operations::{Sketch, Sweep},
        topology::Topology,
        Model,
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

    let mut topology = Topology::new();
    let Topology {
        faces,
        half_edges,
        solids,
        vertices,
    } = &mut topology;

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
            vertices,
            half_edges,
            faces,
        );

    let cuboid = sweep.face_to_solid(
        bottom,
        &Line::to([Scalar::ZERO, Scalar::ZERO, z]),
        vertices,
        half_edges,
        faces,
        solids,
    );

    let model = Model {
        solid: cuboid,
        topology,
    };

    let triangles = model.topology.solids[cuboid]
        .boundary
        .iter()
        .flat_map(|&face| &model.topology.faces[face].approx);

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
