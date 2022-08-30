//! API for iterating over the objects of a shape, or part of a shape

use std::collections::VecDeque;

use crate::objects::{
    Curve, Cycle, Edge, Face, GlobalCurve, GlobalVertex, Shell, Sketch, Solid,
    Surface, Vertex,
};

/// Access iterators over all objects of a shape, or part of it
///
/// Implemented for all object types. An implementation must return itself, in
/// addition to any other objects it references.
pub trait ObjectIters<'r> {
    /// Return all objects that this one references
    fn referenced_objects(&'r self) -> Vec<&'r dyn ObjectIters>;

    /// Iterate over all curves
    fn curve_iter(&'r self) -> Iter<&'r Curve> {
        let mut iter = Iter::empty();

        for object in self.referenced_objects() {
            iter = iter.with(object.curve_iter());
        }

        iter
    }

    /// Iterate over all cycles
    fn cycle_iter(&'r self) -> Iter<&'r Cycle> {
        let mut iter = Iter::empty();

        for object in self.referenced_objects() {
            iter = iter.with(object.cycle_iter());
        }

        iter
    }

    /// Iterate over all edges
    fn edge_iter(&'r self) -> Iter<&'r Edge> {
        let mut iter = Iter::empty();

        for object in self.referenced_objects() {
            iter = iter.with(object.edge_iter());
        }

        iter
    }

    /// Iterate over all faces
    fn face_iter(&'r self) -> Iter<&'r Face> {
        let mut iter = Iter::empty();

        for object in self.referenced_objects() {
            iter = iter.with(object.face_iter());
        }

        iter
    }

    /// Iterate over all global curves
    fn global_curve_iter(&'r self) -> Iter<&'r GlobalCurve> {
        let mut iter = Iter::empty();

        for object in self.referenced_objects() {
            iter = iter.with(object.global_curve_iter());
        }

        iter
    }

    /// Iterate over all global vertices
    fn global_vertex_iter(&'r self) -> Iter<&'r GlobalVertex> {
        let mut iter = Iter::empty();

        for object in self.referenced_objects() {
            iter = iter.with(object.global_vertex_iter());
        }

        iter
    }

    /// Iterate over all shells
    fn shell_iter(&'r self) -> Iter<&'r Shell> {
        let mut iter = Iter::empty();

        for object in self.referenced_objects() {
            iter = iter.with(object.shell_iter());
        }

        iter
    }

    /// Iterate over all sketches
    fn sketch_iter(&'r self) -> Iter<&'r Sketch> {
        let mut iter = Iter::empty();

        for object in self.referenced_objects() {
            iter = iter.with(object.sketch_iter());
        }

        iter
    }

    /// Iterate over all solids
    fn solid_iter(&'r self) -> Iter<&'r Solid> {
        let mut iter = Iter::empty();

        for object in self.referenced_objects() {
            iter = iter.with(object.solid_iter());
        }

        iter
    }

    /// Iterate over all surfaces
    fn surface_iter(&'r self) -> Iter<&'r Surface> {
        let mut iter = Iter::empty();

        for object in self.referenced_objects() {
            iter = iter.with(object.surface_iter());
        }

        iter
    }

    /// Iterator over all vertices
    fn vertex_iter(&'r self) -> Iter<&'r Vertex> {
        let mut iter = Iter::empty();

        for object in self.referenced_objects() {
            iter = iter.with(object.vertex_iter());
        }

        iter
    }
}

impl<'r> ObjectIters<'r> for Curve {
    fn referenced_objects(&'r self) -> Vec<&'r dyn ObjectIters> {
        vec![self.global() as &dyn ObjectIters]
    }

    fn curve_iter(&'r self) -> Iter<&'r Curve> {
        Iter::from_object(self)
    }
}

impl<'r> ObjectIters<'r> for Cycle {
    fn referenced_objects(&'r self) -> Vec<&'r dyn ObjectIters> {
        let mut objects = Vec::new();

        for edge in self.edges() {
            objects.push(edge as &dyn ObjectIters);
        }

        objects
    }

    fn cycle_iter(&'r self) -> Iter<&'r Cycle> {
        Iter::from_object(self)
    }
}

impl<'r> ObjectIters<'r> for Edge {
    fn referenced_objects(&'r self) -> Vec<&'r dyn ObjectIters> {
        let mut objects = vec![self.curve() as &dyn ObjectIters];

        for vertex in self.vertices().iter() {
            objects.push(vertex);
        }

        objects
    }

    fn edge_iter(&'r self) -> Iter<&'r Edge> {
        Iter::from_object(self)
    }
}

impl<'r> ObjectIters<'r> for Face {
    fn referenced_objects(&'r self) -> Vec<&'r dyn ObjectIters> {
        if self.triangles().is_some() {
            return Vec::new();
        }

        let mut objects = vec![self.surface() as &dyn ObjectIters];

        for cycle in self.all_cycles() {
            objects.push(cycle);
        }

        objects
    }

    fn face_iter(&'r self) -> Iter<&'r Face> {
        Iter::from_object(self)
    }
}

impl<'r> ObjectIters<'r> for GlobalCurve {
    fn referenced_objects(&'r self) -> Vec<&'r dyn ObjectIters> {
        Vec::new()
    }

    fn global_curve_iter(&'r self) -> Iter<&'r GlobalCurve> {
        Iter::from_object(self)
    }
}

impl<'r> ObjectIters<'r> for GlobalVertex {
    fn referenced_objects(&'r self) -> Vec<&'r dyn ObjectIters> {
        Vec::new()
    }

    fn global_vertex_iter(&'r self) -> Iter<&'r GlobalVertex> {
        Iter::from_object(self)
    }
}

impl<'r> ObjectIters<'r> for Shell {
    fn referenced_objects(&'r self) -> Vec<&'r dyn ObjectIters> {
        let mut objects = Vec::new();

        for face in self.faces() {
            objects.push(face as &dyn ObjectIters);
        }

        objects
    }

    fn shell_iter(&'r self) -> Iter<&'r Shell> {
        Iter::from_object(self)
    }
}

impl<'r> ObjectIters<'r> for Sketch {
    fn referenced_objects(&'r self) -> Vec<&'r dyn ObjectIters> {
        let mut objects = Vec::new();

        for face in self.faces() {
            objects.push(face as &dyn ObjectIters);
        }

        objects
    }

    fn sketch_iter(&'r self) -> Iter<&'r Sketch> {
        Iter::from_object(self)
    }
}

impl<'r> ObjectIters<'r> for Solid {
    fn referenced_objects(&'r self) -> Vec<&'r dyn ObjectIters> {
        let mut objects = Vec::new();

        for face in self.shells() {
            objects.push(face as &dyn ObjectIters);
        }

        objects
    }

    fn solid_iter(&'r self) -> Iter<&'r Solid> {
        Iter::from_object(self)
    }
}

impl<'r> ObjectIters<'r> for Surface {
    fn referenced_objects(&'r self) -> Vec<&'r dyn ObjectIters> {
        Vec::new()
    }

    fn surface_iter(&'r self) -> Iter<&'r Surface> {
        Iter::from_object(self)
    }
}

impl<'r> ObjectIters<'r> for Vertex {
    fn referenced_objects(&'r self) -> Vec<&'r dyn ObjectIters> {
        vec![
            self.curve() as &dyn ObjectIters,
            self.global() as &dyn ObjectIters,
        ]
    }

    fn vertex_iter(&'r self) -> Iter<&'r Vertex> {
        Iter::from_object(self)
    }
}

// This implementation is useful for test code.
impl<'r, T, O> ObjectIters<'r> for T
where
    T: 'r,
    O: ObjectIters<'r> + 'r,
    &'r T: IntoIterator<Item = &'r O>,
{
    fn referenced_objects(&'r self) -> Vec<&'r dyn ObjectIters> {
        let mut objects = Vec::new();

        for object in self.into_iter() {
            objects.push(object as &dyn ObjectIters);
        }

        objects
    }
}

/// An iterator over objects
///
/// See [`ObjectIters`].
pub struct Iter<T>(VecDeque<T>);

impl<T> Iter<T> {
    fn empty() -> Self {
        Self(VecDeque::new())
    }

    fn from_object(object: T) -> Self {
        let mut objects = VecDeque::new();
        objects.push_back(object);
        Self(objects)
    }

    fn with(mut self, other: Self) -> Self
    where
        T: PartialEq,
    {
        for object in other {
            if !self.0.contains(&object) {
                self.0.push_back(object);
            }
        }

        self
    }
}

impl<T> Iterator for Iter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use crate::objects::{
        Curve, Cycle, Edge, Face, GlobalCurve, GlobalVertex, Shell, Sketch,
        Solid, Surface, Vertex,
    };

    use super::ObjectIters as _;

    #[test]
    fn curve() {
        let surface = Surface::xy_plane();
        let object = Curve::build(surface).u_axis();

        assert_eq!(1, object.curve_iter().count());
        assert_eq!(0, object.cycle_iter().count());
        assert_eq!(0, object.edge_iter().count());
        assert_eq!(0, object.face_iter().count());
        assert_eq!(1, object.global_curve_iter().count());
        assert_eq!(0, object.global_vertex_iter().count());
        assert_eq!(0, object.shell_iter().count());
        assert_eq!(0, object.sketch_iter().count());
        assert_eq!(0, object.solid_iter().count());
        assert_eq!(0, object.surface_iter().count());
        assert_eq!(0, object.vertex_iter().count());
    }

    #[test]
    fn cycle() {
        let object = Cycle::build(Surface::xy_plane()).polygon_from_points([
            [0., 0.],
            [1., 0.],
            [0., 1.],
        ]);

        assert_eq!(3, object.curve_iter().count());
        assert_eq!(1, object.cycle_iter().count());
        assert_eq!(3, object.edge_iter().count());
        assert_eq!(0, object.face_iter().count());
        assert_eq!(3, object.global_curve_iter().count());
        assert_eq!(3, object.global_vertex_iter().count());
        assert_eq!(0, object.shell_iter().count());
        assert_eq!(0, object.sketch_iter().count());
        assert_eq!(0, object.solid_iter().count());
        assert_eq!(0, object.surface_iter().count());
        assert_eq!(6, object.vertex_iter().count());
    }

    #[test]
    fn edge() {
        let object = Edge::build().line_segment_from_points(
            &Surface::xy_plane(),
            [[0., 0.], [1., 0.]],
        );

        assert_eq!(1, object.curve_iter().count());
        assert_eq!(0, object.cycle_iter().count());
        assert_eq!(1, object.edge_iter().count());
        assert_eq!(0, object.face_iter().count());
        assert_eq!(1, object.global_curve_iter().count());
        assert_eq!(2, object.global_vertex_iter().count());
        assert_eq!(0, object.shell_iter().count());
        assert_eq!(0, object.sketch_iter().count());
        assert_eq!(0, object.solid_iter().count());
        assert_eq!(0, object.surface_iter().count());
        assert_eq!(2, object.vertex_iter().count());
    }

    #[test]
    fn face() {
        let surface = Surface::xy_plane();
        let object = Face::build(surface).polygon_from_points([
            [0., 0.],
            [1., 0.],
            [0., 1.],
        ]);

        assert_eq!(3, object.curve_iter().count());
        assert_eq!(1, object.cycle_iter().count());
        assert_eq!(3, object.edge_iter().count());
        assert_eq!(1, object.face_iter().count());
        assert_eq!(3, object.global_curve_iter().count());
        assert_eq!(3, object.global_vertex_iter().count());
        assert_eq!(0, object.shell_iter().count());
        assert_eq!(0, object.sketch_iter().count());
        assert_eq!(0, object.solid_iter().count());
        assert_eq!(1, object.surface_iter().count());
        assert_eq!(6, object.vertex_iter().count());
    }

    #[test]
    fn global_curve() {
        let object = GlobalCurve::build().x_axis();

        assert_eq!(0, object.curve_iter().count());
        assert_eq!(0, object.cycle_iter().count());
        assert_eq!(0, object.edge_iter().count());
        assert_eq!(0, object.face_iter().count());
        assert_eq!(1, object.global_curve_iter().count());
        assert_eq!(0, object.global_vertex_iter().count());
        assert_eq!(0, object.shell_iter().count());
        assert_eq!(0, object.sketch_iter().count());
        assert_eq!(0, object.solid_iter().count());
        assert_eq!(0, object.surface_iter().count());
        assert_eq!(0, object.vertex_iter().count());
    }

    #[test]
    fn global_vertex() {
        let object = GlobalVertex::from_position([0., 0., 0.]);

        assert_eq!(0, object.curve_iter().count());
        assert_eq!(0, object.cycle_iter().count());
        assert_eq!(0, object.edge_iter().count());
        assert_eq!(0, object.face_iter().count());
        assert_eq!(0, object.global_curve_iter().count());
        assert_eq!(1, object.global_vertex_iter().count());
        assert_eq!(0, object.shell_iter().count());
        assert_eq!(0, object.sketch_iter().count());
        assert_eq!(0, object.solid_iter().count());
        assert_eq!(0, object.surface_iter().count());
        assert_eq!(0, object.vertex_iter().count());
    }

    #[test]
    fn shell() {
        let object = Shell::build().cube_from_edge_length(1.);

        assert_eq!(20, object.curve_iter().count());
        assert_eq!(6, object.cycle_iter().count());
        assert_eq!(20, object.edge_iter().count());
        assert_eq!(6, object.face_iter().count());
        assert_eq!(18, object.global_curve_iter().count());
        assert_eq!(8, object.global_vertex_iter().count());
        assert_eq!(1, object.shell_iter().count());
        assert_eq!(0, object.sketch_iter().count());
        assert_eq!(0, object.solid_iter().count());
        assert_eq!(6, object.surface_iter().count());
        assert_eq!(40, object.vertex_iter().count());
    }

    #[test]
    fn sketch() {
        let surface = Surface::xy_plane();
        let face = Face::build(surface).polygon_from_points([
            [0., 0.],
            [1., 0.],
            [0., 1.],
        ]);
        let object = Sketch::new().with_faces([face]);

        assert_eq!(3, object.curve_iter().count());
        assert_eq!(1, object.cycle_iter().count());
        assert_eq!(3, object.edge_iter().count());
        assert_eq!(1, object.face_iter().count());
        assert_eq!(3, object.global_curve_iter().count());
        assert_eq!(3, object.global_vertex_iter().count());
        assert_eq!(0, object.shell_iter().count());
        assert_eq!(1, object.sketch_iter().count());
        assert_eq!(0, object.solid_iter().count());
        assert_eq!(1, object.surface_iter().count());
        assert_eq!(6, object.vertex_iter().count());
    }

    #[test]
    fn solid() {
        let object = Solid::build().cube_from_edge_length(1.);

        assert_eq!(20, object.curve_iter().count());
        assert_eq!(6, object.cycle_iter().count());
        assert_eq!(20, object.edge_iter().count());
        assert_eq!(6, object.face_iter().count());
        assert_eq!(18, object.global_curve_iter().count());
        assert_eq!(8, object.global_vertex_iter().count());
        assert_eq!(1, object.shell_iter().count());
        assert_eq!(0, object.sketch_iter().count());
        assert_eq!(1, object.solid_iter().count());
        assert_eq!(6, object.surface_iter().count());
        assert_eq!(40, object.vertex_iter().count());
    }

    #[test]
    fn surface() {
        let object = Surface::xy_plane();

        assert_eq!(0, object.curve_iter().count());
        assert_eq!(0, object.cycle_iter().count());
        assert_eq!(0, object.edge_iter().count());
        assert_eq!(0, object.face_iter().count());
        assert_eq!(0, object.global_curve_iter().count());
        assert_eq!(0, object.global_vertex_iter().count());
        assert_eq!(0, object.shell_iter().count());
        assert_eq!(0, object.sketch_iter().count());
        assert_eq!(0, object.solid_iter().count());
        assert_eq!(1, object.surface_iter().count());
        assert_eq!(0, object.vertex_iter().count());
    }

    #[test]
    fn vertex() {
        let surface = Surface::xy_plane();
        let curve = Curve::build(surface).u_axis();
        let global_vertex = GlobalVertex::from_position([0., 0., 0.]);
        let object = Vertex::new([0.], curve, global_vertex);

        assert_eq!(1, object.curve_iter().count());
        assert_eq!(0, object.cycle_iter().count());
        assert_eq!(0, object.edge_iter().count());
        assert_eq!(0, object.face_iter().count());
        assert_eq!(1, object.global_curve_iter().count());
        assert_eq!(1, object.global_vertex_iter().count());
        assert_eq!(0, object.shell_iter().count());
        assert_eq!(0, object.sketch_iter().count());
        assert_eq!(0, object.solid_iter().count());
        assert_eq!(0, object.surface_iter().count());
        assert_eq!(1, object.vertex_iter().count());
    }
}
