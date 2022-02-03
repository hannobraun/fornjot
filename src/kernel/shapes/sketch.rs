use nalgebra::point;
use parry3d_f64::bounding_volume::AABB;

use crate::{
    debug::DebugInfo,
    kernel::{
        geometry::{self, Curve, Line, Surface},
        topology::{
            edges::{Edge, Edges},
            faces::{Face, Faces},
        },
        Shape,
    },
    math::Point,
};

impl Shape for fj::Sketch {
    fn bounding_volume(&self) -> AABB {
        AABB::from_points(&self.vertices())
    }

    fn faces(
        &self,
        _: f64,
        cache: &mut geometry::Cache,
        _: &mut DebugInfo,
    ) -> Faces {
        let edges = self.edges(cache);
        let face = Face::Face {
            edges,
            surface: Surface::x_y_plane(),
        };
        Faces(vec![face])
    }

    fn edges(&self, cache: &mut geometry::Cache) -> Edges {
        let v = match self.vertices() {
            vertices if vertices.is_empty() => vertices,
            mut vertices => {
                // Add the first vertex at the end again, to close the loop.
                //
                // This can't panic. This `match` expression makes sure that
                // there are vertices.
                vertices.push(vertices[0]);
                vertices
            }
        };

        let mut edges = Vec::new();
        for window in v.windows(2) {
            // Can't panic, we passed `2` to `windows`.
            //
            // Can be cleaned up, once `array_windows` is stable.
            let a = window[0];
            let b = window[1];

            // TASK: Insert `a` and `b` into cache.
            // TASK: Provides handles to `Line` here instead of the points
            //       themselves.
            let line = Curve::Line(Line { a, b });

            // TASK: Add these as alternative representations of previously
            //       created 3D points.
            let a = cache.insert(point![0.]);
            let b = cache.insert(point![1.]);

            let edge = Edge::new(line, a, b);

            edges.push(edge);
        }

        Edges::single_cycle(edges)
    }

    // TASK: This isn't right. We have 2-dimensional points, but here we return
    //       3-dimensional points. In the approximations, we started going back
    //       to lower dimensions.
    //
    //       Because for triangulation, we want to operate in surface
    //       coordinates again. That was required to make the triangulation
    //       simple/flexible enough to support transformations of b-rep faces.
    fn vertices(&self) -> Vec<Point<3>> {
        self.to_points()
            .into_iter()
            .map(|[x, y]| Point::from([x, y, 0.]))
            .collect()
    }
}
