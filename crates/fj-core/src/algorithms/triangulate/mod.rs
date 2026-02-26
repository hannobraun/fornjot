//! Shape triangulation

mod delaunay;
mod polygon;

use crate::{
    Core,
    approx::Tolerance,
    interop::{MeshTriangle, TriMesh},
    operations::presentation::GetColor,
};

use self::polygon::Polygon;

use super::approx::{Approx, face::FaceApprox};

/// Triangulate a shape
pub trait Triangulate: Sized {
    /// Triangulate the shape
    fn triangulate(self, core: &mut Core) -> TriMesh {
        let mut mesh = TriMesh::new();
        self.triangulate_into_mesh(&mut mesh, core);
        mesh
    }

    /// Triangulate a partial shape into the provided mesh
    ///
    /// This is a low-level method, intended for implementation of
    /// `Triangulate`. Most callers should prefer [`Triangulate::triangulate`].
    fn triangulate_into_mesh(self, mesh: &mut TriMesh, core: &mut Core);
}

impl<T> Triangulate for (T, Tolerance)
where
    T: Approx,
    T::Approximation: IntoIterator<Item = FaceApprox>,
{
    fn triangulate_into_mesh(self, mesh: &mut TriMesh, core: &mut Core) {
        let (approx, tolerance) = self;

        let approx = approx.approx(tolerance, &core.layers.geometry);

        for approx in approx {
            approx.triangulate_into_mesh(mesh, core);
        }
    }
}

impl Triangulate for FaceApprox {
    fn triangulate_into_mesh(self, mesh: &mut TriMesh, core: &mut Core) {
        let face_as_polygon = Polygon::new()
            .with_exterior(
                self.exterior
                    .points()
                    .into_iter()
                    .map(|point| point.local_form),
            )
            .with_interiors(self.interiors.iter().map(|interior| {
                interior.points().into_iter().map(|point| point.local_form)
            }));

        let cycles = [self.exterior].into_iter().chain(self.interiors);
        let mut triangles =
            delaunay::triangulate(cycles, self.coord_handedness);
        triangles.retain(|triangle| {
            face_as_polygon
                .contains_triangle(triangle.map(|point| point.point_surface))
        });

        let color = self.face.region().get_color(core).unwrap_or_default();

        for triangle in triangles {
            let points = triangle.map(|point| point.point_global);
            mesh.triangles.push(MeshTriangle {
                inner: points.into(),
                is_internal: false,
                color,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Core,
        algorithms::approx::{ApproxCache, face::approx_face},
        approx::Tolerance,
        geometry::repr::tri_mesh::convert_point_surface_to_global,
        interop::TriMesh,
        math::{Point, Scalar},
        operations::{
            build::{BuildCycle, BuildFace},
            insert::Insert,
            update::{UpdateFace, UpdateRegion},
        },
        storage::Handle,
        topology::{Cycle, Face},
    };

    use super::Triangulate;

    #[test]
    fn simple() -> anyhow::Result<()> {
        let mut core = Core::new();

        let a = [0., 0.];
        let b = [2., 0.];
        let c = [2., 2.];
        let d = [0., 1.];

        let surface = core.layers.topology.surfaces.xy_plane();

        let face = Face::unbound(surface.clone(), &mut core)
            .update_region(
                |region, core| {
                    region.update_exterior(
                        |_, core| Cycle::polygon([a, b, c, d], surface, core),
                        core,
                    )
                },
                &mut core,
            )
            .insert(&mut core);

        let a = Point::from(a).to_xyz();
        let b = Point::from(b).to_xyz();
        let c = Point::from(c).to_xyz();
        let d = Point::from(d).to_xyz();

        let triangles = triangulate(face, &mut core)?;

        assert!(triangles.contains_triangle([a, b, d]));
        assert!(triangles.contains_triangle([b, c, d]));
        assert!(!triangles.contains_triangle([a, b, c]));
        assert!(!triangles.contains_triangle([a, c, d]));

        Ok(())
    }

    #[test]
    fn simple_hole() -> anyhow::Result<()> {
        let mut core = Core::new();

        let a = [0., 0.];
        let b = [4., 0.];
        let c = [4., 4.];
        let d = [0., 4.];

        let e = [1., 1.];
        let f = [1., 2.];
        let g = [3., 3.];
        let h = [3., 1.];

        let surface = core.layers.topology.surfaces.xy_plane();

        let face = Face::unbound(surface.clone(), &mut core)
            .update_region(
                |region, core| {
                    region
                        .update_exterior(
                            |_, core| {
                                Cycle::polygon(
                                    [a, b, c, d],
                                    surface.clone(),
                                    core,
                                )
                            },
                            core,
                        )
                        .add_interiors(
                            [Cycle::polygon(
                                [e, f, g, h],
                                surface.clone(),
                                core,
                            )],
                            core,
                        )
                },
                &mut core,
            )
            .insert(&mut core);

        let triangles = triangulate(face, &mut core)?;

        let [a, b, e, f, g, h] = [a, b, e, f, g, h].map(|point| {
            convert_point_surface_to_global(
                &core
                    .layers
                    .geometry
                    .of_surface_2(&surface)
                    .unwrap()
                    .generator,
                point,
                core.tolerance(),
                &core.layers.geometry,
            )
        });

        // Let's test that some correct triangles are present. We don't need to
        // test them all.
        //
        // Please note that there are multiple valid triangulations of any given
        // polygon. So if you change the input data above, or the algorithm, the
        // following assertions might break.
        //
        // This limits the usefulness of this test. It would be better to have a
        // smarter way of verifying the triangulation.
        assert!(triangles.contains_triangle([a, b, e]));
        assert!(triangles.contains_triangle([b, e, h]));

        // Shouldn't contain any possible triangle from the hole.
        assert!(!triangles.contains_triangle([e, f, g]));
        assert!(!triangles.contains_triangle([e, g, h]));
        assert!(!triangles.contains_triangle([e, f, h]));
        assert!(!triangles.contains_triangle([f, g, h]));

        Ok(())
    }

    #[test]
    fn sharp_concave_shape() -> anyhow::Result<()> {
        let mut core = Core::new();

        //   e       c
        //   |\     /|
        //   \ \   / b
        //    \ \ / /
        //     \ d /
        //      \a/

        // Naive Delaunay triangulation will create a triangle (c, d, e), which
        // is not part of the polygon. The higher-level triangulation will
        // filter that out, but it will result in missing triangles.

        let a = [1., 0.];
        let b = [2., 8.];
        let c = [2., 9.];
        let d = [1., 1.];
        let e = [0., 9.];

        let surface = core.layers.topology.surfaces.xy_plane();

        let face = Face::unbound(surface.clone(), &mut core)
            .update_region(
                |region, core| {
                    region.update_exterior(
                        |_, core| {
                            Cycle::polygon(
                                [a, b, c, d, e],
                                surface.clone(),
                                core,
                            )
                        },
                        core,
                    )
                },
                &mut core,
            )
            .insert(&mut core);

        let triangles = triangulate(face, &mut core)?;

        let [a, b, c, d, e] = [a, b, c, d, e].map(|point| {
            convert_point_surface_to_global(
                &core
                    .layers
                    .geometry
                    .of_surface_2(&surface)
                    .unwrap()
                    .generator,
                point,
                core.tolerance(),
                &core.layers.geometry,
            )
        });

        assert!(triangles.contains_triangle([a, b, d]));
        assert!(triangles.contains_triangle([a, d, e]));
        assert!(triangles.contains_triangle([b, c, d]));

        Ok(())
    }

    fn triangulate(
        face: Handle<Face>,
        core: &mut Core,
    ) -> anyhow::Result<TriMesh> {
        let tolerance = Tolerance::from_scalar(Scalar::ONE)?;
        Ok(approx_face(
            face,
            tolerance,
            &mut ApproxCache::default(),
            &core.layers.geometry,
        )
        .triangulate(core))
    }
}
