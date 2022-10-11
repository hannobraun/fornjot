use fj_math::{Line, Point, Scalar, Vector};

use crate::{
    objects::{
        Curve, GlobalCurve, GlobalEdge, GlobalVertex, HalfEdge, Objects,
        Surface, SurfaceVertex, Vertex,
    },
    path::SurfacePath,
    storage::Handle,
};

use super::Sweep;

impl Sweep for (Vertex, Handle<Surface>) {
    type Swept = HalfEdge;

    fn sweep(
        self,
        path: impl Into<Vector<3>>,
        objects: &Objects,
    ) -> Self::Swept {
        let (vertex, surface) = self;
        let path = path.into();

        // The result of sweeping a `Vertex` is an `Edge`. Seems
        // straight-forward at first, but there are some subtleties we need to
        // understand:
        //
        // 1. To create an `Edge`, we need the `Curve` that defines it. A
        //    `Curve` is defined in a `Surface`, and we're going to need that to
        //    create the `Curve`. Which is why this `Sweep` implementation is
        //    for `(Vertex, Surface)`, and not just for `Vertex`.
        // 2. Please note that, while the output `Edge` has two vertices, our
        //    input `Vertex` is not one of them! It can't be, unless the `Curve`
        //    of the output `Edge` happens to be the same `Curve` that the input
        //    `Vertex` is defined on. That would be an edge case that probably
        //    can't result in anything valid, and we're going to ignore it for
        //    now.
        // 3. This means, we have to compute everything that defines the
        //    output `Edge`: The `Curve`, the vertices, and the `GlobalCurve`.
        //
        // Before we get to that though, let's make sure that whoever called
        // this didn't give us bad input.

        // So, we're supposed to create the `Edge` by sweeping a `Vertex` using
        // `path`. Unless `path` is identical to the path that created the
        // `Surface`, this doesn't make any sense. Let's make sure this
        // requirement is met.
        //
        // Further, the `Curve` that was swept to create the `Surface` needs to
        // be the same `Curve` that the input `Vertex` is defined on. If it's
        // not, we have no way of knowing the surface coordinates of the input
        // `Vertex` on the `Surface`, and we're going to need to do that further
        // down. There's no way to check for that, unfortunately.
        assert_eq!(path, surface.v());

        // With that out of the way, let's start by creating the `GlobalEdge`,
        // as that is the most straight-forward part of this operations, and
        // we're going to need it soon anyway.
        let (edge_global, vertices_global) =
            vertex.global_form().clone().sweep(path, objects);

        // Next, let's compute the surface coordinates of the two vertices of
        // the output `Edge`, as we're going to need these for the rest of this
        // operation.
        //
        // They both share a u-coordinate, which is the t-coordinate of our
        // input `Vertex`. Remember, we validated above, that the `Curve` of the
        // `Surface` and the curve of the input `Vertex` are the same, so we can
        // do that.
        //
        // Now remember what we also validated above: That `path`, which we're
        // using to create the output `Edge`, also created the `Surface`, and
        // thereby defined its coordinate system. That makes the v-coordinates
        // straight-forward: The start of the edge is at zero, the end is at
        // one.
        let points_surface = [
            Point::from([vertex.position().t, Scalar::ZERO]),
            Point::from([vertex.position().t, Scalar::ONE]),
        ];

        // Armed with those coordinates, creating the `Curve` of the output
        // `Edge` is straight-forward.
        let curve = {
            let line = Line::from_points(points_surface);

            Curve::new(
                surface.clone(),
                SurfacePath::Line(line),
                edge_global.curve().clone(),
                objects,
            )
        };

        // And now the vertices. Again, nothing wild here.
        let vertices = {
            // Can be cleaned up, once `zip` is stable:
            // https://doc.rust-lang.org/std/primitive.array.html#method.zip
            let [a_surface, b_surface] = points_surface;
            let [a_global, b_global] = vertices_global;
            let vertices_surface =
                [(a_surface, a_global), (b_surface, b_global)].map(
                    |(point_surface, vertex_global)| {
                        SurfaceVertex::new(
                            point_surface,
                            surface.clone(),
                            vertex_global,
                        )
                    },
                );

            vertices_surface.map(|surface_form| {
                Vertex::new(
                    [surface_form.position().v],
                    curve.clone(),
                    surface_form,
                )
            })
        };

        // And finally, creating the output `Edge` is just a matter of
        // assembling the pieces we've already created.
        HalfEdge::new(vertices, edge_global)
    }
}

impl Sweep for Handle<GlobalVertex> {
    type Swept = (GlobalEdge, [Handle<GlobalVertex>; 2]);

    fn sweep(
        self,
        path: impl Into<Vector<3>>,
        objects: &Objects,
    ) -> Self::Swept {
        let curve = GlobalCurve::new(objects);

        let a = self.clone();
        let b =
            GlobalVertex::from_position(self.position() + path.into(), objects);

        let vertices = [a, b];
        let global_edge = GlobalEdge::new(curve, vertices.clone());

        // The vertices of the returned `GlobalEdge` are in normalized order,
        // which means the order can't be relied upon by the caller. Return the
        // ordered vertices in addition.
        (global_edge, vertices)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::{
        algorithms::sweep::Sweep,
        objects::{Curve, HalfEdge, Objects, Vertex},
        partial::HasPartial,
        storage::Handle,
    };

    #[test]
    fn vertex_surface() {
        let objects = Objects::new();

        let surface = objects.surfaces.xz_plane();
        let curve = Handle::<Curve>::partial()
            .with_surface(Some(surface.clone()))
            .as_u_axis()
            .build(&objects);
        let vertex = Vertex::partial()
            .with_position(Some([0.]))
            .with_curve(Some(curve))
            .build(&objects);

        let half_edge = (vertex, surface.clone()).sweep([0., 0., 1.], &objects);

        let expected_half_edge = HalfEdge::partial()
            .with_surface(Some(surface))
            .as_line_segment_from_points([[0., 0.], [0., 1.]])
            .build(&objects);
        assert_eq!(half_edge, expected_half_edge);
    }
}
