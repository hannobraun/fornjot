use std::rc::Rc;

use fj::{
    debug::DEBUG_WINDOW,
    geometry::{Line, Plane},
    helpers::Orientation,
    storage::Handle,
    topology::{
        Curve, Edge, Face, LocalCurve, LocalEdge, LocalFace, LocalVertex,
        Solid, Surface, Vertex,
    },
    viewer::ViewerHandle,
};
use fj_interop::{Color, MeshTriangle, TriMesh};
use fj_math::{Point, Triangle, Vector};

fn main() -> anyhow::Result<()> {
    let tri_mesh = fj::viewer::make_viewer_and_spawn_thread(|viewer| {
        DEBUG_WINDOW.initialize(&viewer);
        model(&viewer)
    })?;

    fj::export::export(tri_mesh.external_triangles(), "output.3mf")?;

    Ok(())
}

fn model(viewer: &ViewerHandle) -> TriMesh {
    let left_front_bottom = Handle::new(Vertex {});
    let left_front_top = Handle::new(Vertex {});
    let left_back_bottom = Handle::new(Vertex {});
    let left_back_top = Handle::new(Vertex {});
    let right_front_bottom = Handle::new(Vertex {});
    let right_front_top = Handle::new(Vertex {});
    let right_back_bottom = Handle::new(Vertex {});
    let right_back_top = Handle::new(Vertex {});

    let front_bottom_curve = Handle::new(Curve {});
    let right_bottom_curve = Handle::new(Curve {});
    let back_bottom_curve = Handle::new(Curve {});
    let left_bottom_curve = Handle::new(Curve {});
    let front_top_curve = Handle::new(Curve {});
    let right_top_curve = Handle::new(Curve {});
    let back_top_curve = Handle::new(Curve {});
    let left_top_curve = Handle::new(Curve {});
    let left_front_curve = Handle::new(Curve {});
    let right_front_curve = Handle::new(Curve {});
    let right_back_curve = Handle::new(Curve {});
    let left_back_curve = Handle::new(Curve {});

    let front_bottom_edge = Handle::new(Edge {
        curve: front_bottom_curve.clone(),
        boundary: [
            LocalVertex {
                vertex: left_front_bottom.clone(),
                position: Point::from([-1.]),
            },
            LocalVertex {
                vertex: right_front_bottom.clone(),
                position: Point::from([1.]),
            },
        ],
    });
    let right_bottom_edge = Handle::new(Edge {
        curve: right_bottom_curve.clone(),
        boundary: [
            LocalVertex {
                vertex: right_front_bottom.clone(),
                position: Point::from([-1.]),
            },
            LocalVertex {
                vertex: right_back_bottom.clone(),
                position: Point::from([1.]),
            },
        ],
    });
    let back_bottom_edge = Handle::new(Edge {
        curve: back_bottom_curve.clone(),
        boundary: [
            LocalVertex {
                vertex: left_back_bottom.clone(),
                position: Point::from([-1.]),
            },
            LocalVertex {
                vertex: right_back_bottom.clone(),
                position: Point::from([1.]),
            },
        ],
    });
    let left_bottom_edge = Handle::new(Edge {
        curve: left_bottom_curve.clone(),
        boundary: [
            LocalVertex {
                vertex: left_front_bottom.clone(),
                position: Point::from([-1.]),
            },
            LocalVertex {
                vertex: left_back_bottom.clone(),
                position: Point::from([1.]),
            },
        ],
    });
    let front_top_edge = Handle::new(Edge {
        curve: front_top_curve.clone(),
        boundary: [
            LocalVertex {
                vertex: left_front_top.clone(),
                position: Point::from([-1.]),
            },
            LocalVertex {
                vertex: right_front_top.clone(),
                position: Point::from([1.]),
            },
        ],
    });
    let right_top_edge = Handle::new(Edge {
        curve: right_top_curve.clone(),
        boundary: [
            LocalVertex {
                vertex: right_front_top.clone(),
                position: Point::from([-1.]),
            },
            LocalVertex {
                vertex: right_back_top.clone(),
                position: Point::from([1.]),
            },
        ],
    });
    let back_top_edge = Handle::new(Edge {
        curve: back_top_curve.clone(),
        boundary: [
            LocalVertex {
                vertex: left_back_top.clone(),
                position: Point::from([-1.]),
            },
            LocalVertex {
                vertex: right_back_top.clone(),
                position: Point::from([1.]),
            },
        ],
    });
    let left_top_edge = Handle::new(Edge {
        curve: left_top_curve.clone(),
        boundary: [
            LocalVertex {
                vertex: left_front_top.clone(),
                position: Point::from([-1.]),
            },
            LocalVertex {
                vertex: left_back_top.clone(),
                position: Point::from([1.]),
            },
        ],
    });
    let left_front_edge = Handle::new(Edge {
        curve: left_front_curve.clone(),
        boundary: [
            LocalVertex {
                vertex: left_front_bottom,
                position: Point::from([-1.]),
            },
            LocalVertex {
                vertex: left_front_top,
                position: Point::from([1.]),
            },
        ],
    });
    let right_front_edge = Handle::new(Edge {
        curve: right_front_curve.clone(),
        boundary: [
            LocalVertex {
                vertex: right_front_bottom,
                position: Point::from([-1.]),
            },
            LocalVertex {
                vertex: right_front_top,
                position: Point::from([1.]),
            },
        ],
    });
    let right_back_edge = Handle::new(Edge {
        curve: right_back_curve.clone(),
        boundary: [
            LocalVertex {
                vertex: right_back_bottom,
                position: Point::from([-1.]),
            },
            LocalVertex {
                vertex: right_back_top,
                position: Point::from([1.]),
            },
        ],
    });
    let left_back_edge = Handle::new(Edge {
        curve: left_back_curve.clone(),
        boundary: [
            LocalVertex {
                vertex: left_back_bottom,
                position: Point::from([-1.]),
            },
            LocalVertex {
                vertex: left_back_top,
                position: Point::from([1.]),
            },
        ],
    });

    let solid = Solid {
        boundary: vec![
            LocalFace {
                face: Handle::new(Face {
                    surface: Handle::new(Surface {
                        origin: Point::from([0., -1., 0.]),
                        geometry: Handle {
                            inner: Rc::new(Plane {
                                u: Vector::from([1., 0., 0.]),
                                v: Vector::from([0., 0., 1.]),
                            }),
                        },
                    }),
                    boundary: vec![
                        LocalEdge {
                            edge: front_bottom_edge.clone(),
                            curve: Handle::new(LocalCurve {
                                curve: front_bottom_curve.clone(),
                                origin: Point::from([0., -1.]),
                                geometry: Handle {
                                    inner: Rc::new(Line {
                                        t: Vector::from([1., 0.]),
                                    }),
                                },
                            }),
                            orientation: Orientation::Nominal,
                        },
                        LocalEdge {
                            edge: right_front_edge.clone(),
                            curve: Handle::new(LocalCurve {
                                curve: right_front_curve.clone(),
                                origin: Point::from([1., 0.]),
                                geometry: Handle {
                                    inner: Rc::new(Line {
                                        t: Vector::from([0., 1.]),
                                    }),
                                },
                            }),
                            orientation: Orientation::Nominal,
                        },
                        LocalEdge {
                            edge: front_top_edge.clone(),
                            curve: Handle::new(LocalCurve {
                                curve: front_top_curve.clone(),
                                origin: Point::from([0., 1.]),
                                geometry: Handle {
                                    inner: Rc::new(Line {
                                        t: Vector::from([-1., 0.]),
                                    }),
                                },
                            }),
                            orientation: Orientation::AntiNominal,
                        },
                        LocalEdge {
                            edge: left_front_edge.clone(),
                            curve: Handle::new(LocalCurve {
                                curve: left_front_curve.clone(),
                                origin: Point::from([-1., 0.]),
                                geometry: Handle {
                                    inner: Rc::new(Line {
                                        t: Vector::from([0., -1.]),
                                    }),
                                },
                            }),
                            orientation: Orientation::AntiNominal,
                        },
                    ],
                }),
                orientation: Orientation::Nominal,
            },
            LocalFace {
                face: Handle::new(Face {
                    surface: Handle::new(Surface {
                        origin: Point::from([1., 0., 0.]),
                        geometry: Handle {
                            inner: Rc::new(Plane {
                                u: Vector::from([0., 1., 0.]),
                                v: Vector::from([0., 0., 1.]),
                            }),
                        },
                    }),
                    boundary: vec![
                        LocalEdge {
                            edge: right_bottom_edge.clone(),
                            curve: Handle::new(LocalCurve {
                                curve: right_bottom_curve.clone(),
                                origin: Point::from([0., -1.]),
                                geometry: Handle {
                                    inner: Rc::new(Line {
                                        t: Vector::from([1., 0.]),
                                    }),
                                },
                            }),
                            orientation: Orientation::Nominal,
                        },
                        LocalEdge {
                            edge: right_back_edge.clone(),
                            curve: Handle::new(LocalCurve {
                                curve: right_back_curve.clone(),
                                origin: Point::from([1., 0.]),
                                geometry: Handle {
                                    inner: Rc::new(Line {
                                        t: Vector::from([0., 1.]),
                                    }),
                                },
                            }),
                            orientation: Orientation::Nominal,
                        },
                        LocalEdge {
                            edge: right_top_edge.clone(),
                            curve: Handle::new(LocalCurve {
                                curve: right_top_curve.clone(),
                                origin: Point::from([0., 1.]),
                                geometry: Handle {
                                    inner: Rc::new(Line {
                                        t: Vector::from([-1., 0.]),
                                    }),
                                },
                            }),
                            orientation: Orientation::AntiNominal,
                        },
                        LocalEdge {
                            edge: right_front_edge,
                            curve: Handle::new(LocalCurve {
                                curve: right_front_curve,
                                origin: Point::from([-1., 0.]),
                                geometry: Handle {
                                    inner: Rc::new(Line {
                                        t: Vector::from([0., -1.]),
                                    }),
                                },
                            }),
                            orientation: Orientation::AntiNominal,
                        },
                    ],
                }),
                orientation: Orientation::Nominal,
            },
            LocalFace {
                face: Handle::new(Face {
                    surface: Handle::new(Surface {
                        origin: Point::from([0., 1., 0.]),
                        geometry: Handle {
                            inner: Rc::new(Plane {
                                u: Vector::from([-1., 0., 0.]),
                                v: Vector::from([0., 0., 1.]),
                            }),
                        },
                    }),
                    boundary: vec![
                        LocalEdge {
                            edge: back_bottom_edge.clone(),
                            curve: Handle::new(LocalCurve {
                                curve: back_bottom_curve.clone(),
                                origin: Point::from([0., -1.]),
                                geometry: Handle {
                                    inner: Rc::new(Line {
                                        t: Vector::from([1., 0.]),
                                    }),
                                },
                            }),
                            orientation: Orientation::AntiNominal,
                        },
                        LocalEdge {
                            edge: left_back_edge.clone(),
                            curve: Handle::new(LocalCurve {
                                curve: left_back_curve.clone(),
                                origin: Point::from([1., 0.]),
                                geometry: Handle {
                                    inner: Rc::new(Line {
                                        t: Vector::from([0., 1.]),
                                    }),
                                },
                            }),
                            orientation: Orientation::Nominal,
                        },
                        LocalEdge {
                            edge: back_top_edge.clone(),
                            curve: Handle::new(LocalCurve {
                                curve: back_top_curve.clone(),
                                origin: Point::from([0., 1.]),
                                geometry: Handle {
                                    inner: Rc::new(Line {
                                        t: Vector::from([-1., 0.]),
                                    }),
                                },
                            }),
                            orientation: Orientation::Nominal,
                        },
                        LocalEdge {
                            edge: right_back_edge,
                            curve: Handle::new(LocalCurve {
                                curve: right_back_curve,
                                origin: Point::from([-1., 0.]),
                                geometry: Handle {
                                    inner: Rc::new(Line {
                                        t: Vector::from([0., -1.]),
                                    }),
                                },
                            }),
                            orientation: Orientation::AntiNominal,
                        },
                    ],
                }),
                orientation: Orientation::Nominal,
            },
            LocalFace {
                face: Handle::new(Face {
                    surface: Handle::new(Surface {
                        origin: Point::from([-1., 0., 0.]),
                        geometry: Handle {
                            inner: Rc::new(Plane {
                                u: Vector::from([0., -1., 0.]),
                                v: Vector::from([0., 0., 1.]),
                            }),
                        },
                    }),
                    boundary: vec![
                        LocalEdge {
                            edge: left_bottom_edge.clone(),
                            curve: Handle::new(LocalCurve {
                                curve: left_bottom_curve.clone(),
                                origin: Point::from([0., -1.]),
                                geometry: Handle {
                                    inner: Rc::new(Line {
                                        t: Vector::from([1., 0.]),
                                    }),
                                },
                            }),
                            orientation: Orientation::AntiNominal,
                        },
                        LocalEdge {
                            edge: left_front_edge,
                            curve: Handle::new(LocalCurve {
                                curve: left_front_curve,
                                origin: Point::from([1., 0.]),
                                geometry: Handle {
                                    inner: Rc::new(Line {
                                        t: Vector::from([0., 1.]),
                                    }),
                                },
                            }),
                            orientation: Orientation::Nominal,
                        },
                        LocalEdge {
                            edge: left_top_edge.clone(),
                            curve: Handle::new(LocalCurve {
                                curve: left_top_curve.clone(),
                                origin: Point::from([0., 1.]),
                                geometry: Handle {
                                    inner: Rc::new(Line {
                                        t: Vector::from([-1., 0.]),
                                    }),
                                },
                            }),
                            orientation: Orientation::Nominal,
                        },
                        LocalEdge {
                            edge: left_back_edge,
                            curve: Handle::new(LocalCurve {
                                curve: left_back_curve,
                                origin: Point::from([-1., 0.]),
                                geometry: Handle {
                                    inner: Rc::new(Line {
                                        t: Vector::from([0., -1.]),
                                    }),
                                },
                            }),
                            orientation: Orientation::AntiNominal,
                        },
                    ],
                }),
                orientation: Orientation::Nominal,
            },
            LocalFace {
                face: Handle::new(Face {
                    surface: Handle::new(Surface {
                        origin: Point::from([0., 0., -1.]),
                        geometry: Handle {
                            inner: Rc::new(Plane {
                                u: Vector::from([1., 0., 0.]),
                                v: Vector::from([0., -1., 0.]),
                            }),
                        },
                    }),
                    boundary: vec![
                        LocalEdge {
                            edge: back_bottom_edge,
                            curve: Handle::new(LocalCurve {
                                curve: back_bottom_curve,
                                origin: Point::from([0., -1.]),
                                geometry: Handle {
                                    inner: Rc::new(Line {
                                        t: Vector::from([1., 0.]),
                                    }),
                                },
                            }),
                            orientation: Orientation::Nominal,
                        },
                        LocalEdge {
                            edge: right_bottom_edge,
                            curve: Handle::new(LocalCurve {
                                curve: right_bottom_curve,
                                origin: Point::from([1., 0.]),
                                geometry: Handle {
                                    inner: Rc::new(Line {
                                        t: Vector::from([0., 1.]),
                                    }),
                                },
                            }),
                            orientation: Orientation::AntiNominal,
                        },
                        LocalEdge {
                            edge: front_bottom_edge,
                            curve: Handle::new(LocalCurve {
                                curve: front_bottom_curve,
                                origin: Point::from([0., 1.]),
                                geometry: Handle {
                                    inner: Rc::new(Line {
                                        t: Vector::from([-1., 0.]),
                                    }),
                                },
                            }),
                            orientation: Orientation::AntiNominal,
                        },
                        LocalEdge {
                            edge: left_bottom_edge,
                            curve: Handle::new(LocalCurve {
                                curve: left_bottom_curve.clone(),
                                origin: Point::from([-1., 0.]),
                                geometry: Handle {
                                    inner: Rc::new(Line {
                                        t: Vector::from([0., -1.]),
                                    }),
                                },
                            }),
                            orientation: Orientation::Nominal,
                        },
                    ],
                }),
                orientation: Orientation::Nominal,
            },
            LocalFace {
                face: Handle::new(Face {
                    surface: Handle::new(Surface {
                        origin: Point::from([0., 0., 1.]),
                        geometry: Handle {
                            inner: Rc::new(Plane {
                                u: Vector::from([1., 0., 0.]),
                                v: Vector::from([0., 1., 0.]),
                            }),
                        },
                    }),
                    boundary: vec![
                        LocalEdge {
                            edge: front_top_edge,
                            curve: Handle::new(LocalCurve {
                                curve: front_top_curve,
                                origin: Point::from([0., -1.]),
                                geometry: Handle {
                                    inner: Rc::new(Line {
                                        t: Vector::from([1., 0.]),
                                    }),
                                },
                            }),
                            orientation: Orientation::Nominal,
                        },
                        LocalEdge {
                            edge: right_top_edge,
                            curve: Handle::new(LocalCurve {
                                curve: right_top_curve,
                                origin: Point::from([1., 0.]),
                                geometry: Handle {
                                    inner: Rc::new(Line {
                                        t: Vector::from([0., 1.]),
                                    }),
                                },
                            }),
                            orientation: Orientation::Nominal,
                        },
                        LocalEdge {
                            edge: back_top_edge,
                            curve: Handle::new(LocalCurve {
                                curve: back_top_curve,
                                origin: Point::from([0., 1.]),
                                geometry: Handle {
                                    inner: Rc::new(Line {
                                        t: Vector::from([-1., 0.]),
                                    }),
                                },
                            }),
                            orientation: Orientation::AntiNominal,
                        },
                        LocalEdge {
                            edge: left_top_edge,
                            curve: Handle::new(LocalCurve {
                                curve: left_top_curve.clone(),
                                origin: Point::from([-1., 0.]),
                                geometry: Handle {
                                    inner: Rc::new(Line {
                                        t: Vector::from([0., -1.]),
                                    }),
                                },
                            }),
                            orientation: Orientation::AntiNominal,
                        },
                    ],
                }),
                orientation: Orientation::Nominal,
            },
        ],
    };

    // The b-rep definition above should match the triangle mesh below. Next up,
    // generate that triangle mesh from the b-rep.
    let _ = solid;

    let tri_mesh = TriMesh {
        triangles: vec![
            MeshTriangle {
                inner: Triangle::from_points([
                    [-1., -1., -1.],
                    [1., -1., -1.],
                    [-1., -1., 1.],
                ]),
                is_internal: false,
                color: Color::default(),
            },
            MeshTriangle {
                inner: Triangle::from_points([
                    [1., -1., -1.],
                    [1., -1., 1.],
                    [-1., -1., 1.],
                ]),
                is_internal: false,
                color: Color::default(),
            },
            MeshTriangle {
                inner: Triangle::from_points([
                    [1., -1., -1.],
                    [1., 1., -1.],
                    [1., -1., 1.],
                ]),
                is_internal: false,
                color: Color::default(),
            },
            MeshTriangle {
                inner: Triangle::from_points([
                    [1., 1., -1.],
                    [1., 1., 1.],
                    [1., -1., 1.],
                ]),
                is_internal: false,
                color: Color::default(),
            },
            MeshTriangle {
                inner: Triangle::from_points([
                    [1., 1., -1.],
                    [-1., 1., -1.],
                    [1., 1., 1.],
                ]),
                is_internal: false,
                color: Color::default(),
            },
            MeshTriangle {
                inner: Triangle::from_points([
                    [-1., 1., -1.],
                    [-1., 1., 1.],
                    [1., 1., 1.],
                ]),
                is_internal: false,
                color: Color::default(),
            },
            MeshTriangle {
                inner: Triangle::from_points([
                    [-1., 1., -1.],
                    [-1., -1., -1.],
                    [-1., 1., 1.],
                ]),
                is_internal: false,
                color: Color::default(),
            },
            MeshTriangle {
                inner: Triangle::from_points([
                    [-1., -1., -1.],
                    [-1., -1., 1.],
                    [-1., 1., 1.],
                ]),
                is_internal: false,
                color: Color::default(),
            },
            MeshTriangle {
                inner: Triangle::from_points([
                    [-1., 1., -1.],
                    [1., 1., -1.],
                    [-1., -1., -1.],
                ]),
                is_internal: false,
                color: Color::default(),
            },
            MeshTriangle {
                inner: Triangle::from_points([
                    [1., 1., -1.],
                    [1., -1., -1.],
                    [-1., -1., -1.],
                ]),
                is_internal: false,
                color: Color::default(),
            },
            MeshTriangle {
                inner: Triangle::from_points([
                    [-1., -1., 1.],
                    [1., -1., 1.],
                    [-1., 1., 1.],
                ]),
                is_internal: false,
                color: Color::default(),
            },
            MeshTriangle {
                inner: Triangle::from_points([
                    [1., -1., 1.],
                    [1., 1., 1.],
                    [-1., 1., 1.],
                ]),
                is_internal: false,
                color: Color::default(),
            },
        ],
    };

    viewer.open_window().display_mesh(tri_mesh.clone());

    tri_mesh
}
