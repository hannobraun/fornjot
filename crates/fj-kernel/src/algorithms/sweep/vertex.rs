use fj_interop::mesh::Color;
use fj_math::{Line, Point, Scalar};

use crate::{
    algorithms::approx::Tolerance,
    objects::{
        Curve, CurveKind, Edge, GlobalCurve, GlobalEdge, GlobalVertex, Surface,
        SweptCurve, Vertex, VerticesOfEdge,
    },
};

use super::{Path, Sweep};

impl Sweep for (Vertex, Surface) {
    type Swept = Edge;

    fn sweep(
        self,
        path: impl Into<Path>,
        tolerance: impl Into<Tolerance>,
        color: Color,
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
        // 2. Please note that, while the result `Edge` has two vertices, our
        //    input `Vertex` is not one of them! It can't be, unless the `Curve`
        //    of the resulting `Edge` happens to be the same `Curve` that the
        //    input `Vertex` is defined on. That would be an edge case that
        //    probably can't result in anything valid, and we're going to ignore
        //    it for now.
        // 3. This means, we have to compute everything that defines the
        //    resulting `Edge`: The `Curve`, the vertices, and the
        //    `GlobalCurve`.
        //
        // Before we get to that though, let's make sure that whoever called
        // this didn't give us bad input.

        // So, we're supposed to create the `Edge` by sweeping a `Vertex` using
        // `path`. Unless `path` is identical to the path that created the
        // `Surface`, this doesn't make any sense.
        //
        // Further, the `Curve` that was swept to create the `Surface` needs to
        // be the same `Curve` that the input `Vertex` is defined on. If it's
        // not, we have no way of knowing the surface coordinates of the input
        // `Vertex` on the `Surface`, and we're going to need to do that further
        // down.
        //
        // Let's make sure that these requirements are met.
        {
            let Surface::SweptCurve(SweptCurve {
                curve: surface_curve,
                path: surface_path,
            }) = surface;

            assert_eq!(vertex.curve().global_form().kind(), &surface_curve);
            assert_eq!(path.inner(), surface_path);
        }

        // With that out of the way, let's start by creating the `GlobalEdge`,
        // as that is the most straight-forward part of this operations, and
        // we're going to need it soon anyway.
        let edge_global = vertex.global_form().sweep(path, tolerance, color);

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
        let a_surface = Point::from([vertex.position().t, Scalar::ZERO]);
        let b_surface = Point::from([vertex.position().t, Scalar::ONE]);

        // Armed with those coordinates, creating the `Curve` of the output
        // `Edge` becomes straight-forward.
        let curve = {
            let line = Line::from_points([a_surface, b_surface]);

            Curve::new(surface, CurveKind::Line(line), *edge_global.curve())
        };

        // And now the vertices. Again, nothing wild here.
        let vertices = {
            let [&a_global, &b_global] = edge_global.vertices().get_or_panic();

            let a = Vertex::new([a_surface.v], curve, a_global);
            let b = Vertex::new([b_surface.v], curve, b_global);

            VerticesOfEdge::from_vertices([a, b])
        };

        // And finally, creating the output `Edge` is just a matter of
        // assembling the pieces we've already created.
        Edge::new(curve, vertices, edge_global)
    }
}

impl Sweep for GlobalVertex {
    type Swept = GlobalEdge;

    fn sweep(
        self,
        path: impl Into<Path>,
        _: impl Into<Tolerance>,
        _: Color,
    ) -> Self::Swept {
        let a = self;
        let b =
            GlobalVertex::from_position(self.position() + path.into().inner());

        let curve =
            GlobalCurve::build().line_from_points([a.position(), b.position()]);

        GlobalEdge::new(curve, VerticesOfEdge::from_vertices([a, b]))
    }
}
