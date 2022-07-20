//! API for iterating over the objects of a shape, or part of a shape

use std::collections::VecDeque;

use crate::objects::{
    Curve, Cycle, Edge, Face, GlobalVertex, Sketch, Solid, Surface, Vertex,
};

/// Access iterators over all objects of a shape, or part of it
///
/// Implemented for all object types. An implementation must return itself, in
/// addition to any other objects it references.
pub trait ObjectIters {
    /// Iterate over all curves
    fn curve_iter(&self) -> Iter<Curve<3>>;

    /// Iterate over all cycles
    fn cycle_iter(&self) -> Iter<Cycle>;

    /// Iterate over all edges
    fn edge_iter(&self) -> Iter<Edge>;

    /// Iterate over all faces
    fn face_iter(&self) -> Iter<Face>;

    /// Iterate over all global vertices
    fn global_vertex_iter(&self) -> Iter<GlobalVertex>;

    /// Iterate over all sketches
    fn sketch_iter(&self) -> Iter<Sketch>;

    /// Iterate over all solids
    fn solid_iter(&self) -> Iter<Solid>;

    /// Iterate over all surfaces
    fn surface_iter(&self) -> Iter<Surface>;

    /// Iterator over all vertices
    fn vertex_iter(&self) -> Iter<Vertex>;
}

impl ObjectIters for Curve<3> {
    fn curve_iter(&self) -> Iter<Curve<3>> {
        Iter::from_object(*self)
    }

    fn cycle_iter(&self) -> Iter<Cycle> {
        Iter::empty()
    }

    fn edge_iter(&self) -> Iter<Edge> {
        Iter::empty()
    }

    fn face_iter(&self) -> Iter<Face> {
        Iter::empty()
    }

    fn global_vertex_iter(&self) -> Iter<GlobalVertex> {
        Iter::empty()
    }

    fn sketch_iter(&self) -> Iter<Sketch> {
        Iter::empty()
    }

    fn solid_iter(&self) -> Iter<Solid> {
        Iter::empty()
    }

    fn surface_iter(&self) -> Iter<Surface> {
        Iter::empty()
    }

    fn vertex_iter(&self) -> Iter<Vertex> {
        Iter::empty()
    }
}

impl ObjectIters for Cycle {
    fn curve_iter(&self) -> Iter<Curve<3>> {
        let mut iter = Iter::empty();

        for edge in self.edges() {
            iter = iter.with(edge.curve_iter());
        }

        iter
    }

    fn cycle_iter(&self) -> Iter<Cycle> {
        Iter::from_object(self.clone())
    }

    fn edge_iter(&self) -> Iter<Edge> {
        let mut iter = Iter::empty();

        for edge in self.edges() {
            iter = iter.with(edge.edge_iter());
        }

        iter
    }

    fn face_iter(&self) -> Iter<Face> {
        let mut iter = Iter::empty();

        for edge in self.edges() {
            iter = iter.with(edge.face_iter());
        }

        iter
    }

    fn global_vertex_iter(&self) -> Iter<GlobalVertex> {
        let mut iter = Iter::empty();

        for edge in self.edges() {
            iter = iter.with(edge.global_vertex_iter());
        }

        iter
    }

    fn sketch_iter(&self) -> Iter<Sketch> {
        let mut iter = Iter::empty();

        for edge in self.edges() {
            iter = iter.with(edge.sketch_iter());
        }

        iter
    }

    fn solid_iter(&self) -> Iter<Solid> {
        let mut iter = Iter::empty();

        for edge in self.edges() {
            iter = iter.with(edge.solid_iter());
        }

        iter
    }

    fn surface_iter(&self) -> Iter<Surface> {
        let mut iter = Iter::empty();

        for edge in self.edges() {
            iter = iter.with(edge.surface_iter());
        }

        iter
    }

    fn vertex_iter(&self) -> Iter<Vertex> {
        let mut iter = Iter::empty();

        for edge in self.edges() {
            iter = iter.with(edge.vertex_iter());
        }

        iter
    }
}

impl ObjectIters for Edge {
    fn curve_iter(&self) -> Iter<Curve<3>> {
        let mut iter = Iter::empty().with(self.curve().global().curve_iter());

        for vertex in self.vertices().into_iter().flatten() {
            iter = iter.with(vertex.curve_iter());
        }

        iter
    }

    fn cycle_iter(&self) -> Iter<Cycle> {
        let mut iter = Iter::empty().with(self.curve().global().cycle_iter());

        for vertex in self.vertices().into_iter().flatten() {
            iter = iter.with(vertex.cycle_iter());
        }

        iter
    }

    fn edge_iter(&self) -> Iter<Edge> {
        Iter::from_object(*self)
    }

    fn face_iter(&self) -> Iter<Face> {
        let mut iter = Iter::empty().with(self.curve().global().face_iter());

        for vertex in self.vertices().into_iter().flatten() {
            iter = iter.with(vertex.face_iter());
        }

        iter
    }

    fn global_vertex_iter(&self) -> Iter<GlobalVertex> {
        let mut iter =
            Iter::empty().with(self.curve().global().global_vertex_iter());

        for vertex in self.vertices().into_iter().flatten() {
            iter = iter.with(vertex.global_vertex_iter());
        }

        iter
    }

    fn sketch_iter(&self) -> Iter<Sketch> {
        let mut iter = Iter::empty().with(self.curve().global().sketch_iter());

        for vertex in self.vertices().into_iter().flatten() {
            iter = iter.with(vertex.sketch_iter());
        }

        iter
    }

    fn solid_iter(&self) -> Iter<Solid> {
        let mut iter = Iter::empty().with(self.curve().global().solid_iter());

        for vertex in self.vertices().into_iter().flatten() {
            iter = iter.with(vertex.solid_iter());
        }

        iter
    }

    fn surface_iter(&self) -> Iter<Surface> {
        let mut iter = Iter::empty().with(self.curve().global().surface_iter());

        for vertex in self.vertices().into_iter().flatten() {
            iter = iter.with(vertex.surface_iter());
        }

        iter
    }

    fn vertex_iter(&self) -> Iter<Vertex> {
        let mut iter = Iter::empty().with(self.curve().global().vertex_iter());

        for vertex in self.vertices().into_iter().flatten() {
            iter = iter.with(vertex.vertex_iter());
        }

        iter
    }
}

impl ObjectIters for Face {
    fn curve_iter(&self) -> Iter<Curve<3>> {
        if let Face::Face(face) = self {
            let mut iter = Iter::empty().with(face.surface().curve_iter());

            for cycle in face.all_cycles() {
                iter = iter.with(cycle.curve_iter());
            }

            return iter;
        }

        Iter::empty()
    }

    fn cycle_iter(&self) -> Iter<Cycle> {
        if let Face::Face(face) = self {
            let mut iter = Iter::empty().with(face.surface().cycle_iter());

            for cycle in face.all_cycles() {
                iter = iter.with(cycle.cycle_iter());
            }

            return iter;
        }

        Iter::empty()
    }

    fn edge_iter(&self) -> Iter<Edge> {
        if let Face::Face(face) = self {
            let mut iter = Iter::empty().with(face.surface().edge_iter());

            for cycle in face.all_cycles() {
                iter = iter.with(cycle.edge_iter());
            }

            return iter;
        }

        Iter::empty()
    }

    fn face_iter(&self) -> Iter<Face> {
        Iter::from_object(self.clone())
    }

    fn global_vertex_iter(&self) -> Iter<GlobalVertex> {
        if let Face::Face(face) = self {
            let mut iter =
                Iter::empty().with(face.surface().global_vertex_iter());

            for cycle in face.all_cycles() {
                iter = iter.with(cycle.global_vertex_iter());
            }

            return iter;
        }

        Iter::empty()
    }

    fn sketch_iter(&self) -> Iter<Sketch> {
        if let Face::Face(face) = self {
            let mut iter = Iter::empty().with(face.surface().sketch_iter());

            for cycle in face.all_cycles() {
                iter = iter.with(cycle.sketch_iter());
            }

            return iter;
        }

        Iter::empty()
    }

    fn solid_iter(&self) -> Iter<Solid> {
        if let Face::Face(face) = self {
            let mut iter = Iter::empty().with(face.surface().solid_iter());

            for cycle in face.all_cycles() {
                iter = iter.with(cycle.solid_iter());
            }

            return iter;
        }

        Iter::empty()
    }

    fn surface_iter(&self) -> Iter<Surface> {
        if let Face::Face(face) = self {
            let mut iter = Iter::empty().with(face.surface().surface_iter());

            for cycle in face.all_cycles() {
                iter = iter.with(cycle.surface_iter());
            }

            return iter;
        }

        Iter::empty()
    }

    fn vertex_iter(&self) -> Iter<Vertex> {
        if let Face::Face(face) = self {
            let mut iter = Iter::empty().with(face.surface().vertex_iter());

            for cycle in face.all_cycles() {
                iter = iter.with(cycle.vertex_iter());
            }

            return iter;
        }

        Iter::empty()
    }
}

impl ObjectIters for GlobalVertex {
    fn curve_iter(&self) -> Iter<Curve<3>> {
        Iter::empty()
    }

    fn cycle_iter(&self) -> Iter<Cycle> {
        Iter::empty()
    }

    fn edge_iter(&self) -> Iter<Edge> {
        Iter::empty()
    }

    fn face_iter(&self) -> Iter<Face> {
        Iter::empty()
    }

    fn global_vertex_iter(&self) -> Iter<GlobalVertex> {
        Iter::from_object(*self)
    }

    fn sketch_iter(&self) -> Iter<Sketch> {
        Iter::empty()
    }

    fn solid_iter(&self) -> Iter<Solid> {
        Iter::empty()
    }

    fn surface_iter(&self) -> Iter<Surface> {
        Iter::empty()
    }

    fn vertex_iter(&self) -> Iter<Vertex> {
        Iter::empty()
    }
}

impl ObjectIters for Sketch {
    fn curve_iter(&self) -> Iter<Curve<3>> {
        let mut iter = Iter::empty();

        for edge in self.faces() {
            iter = iter.with(edge.curve_iter());
        }

        iter
    }

    fn cycle_iter(&self) -> Iter<Cycle> {
        let mut iter = Iter::empty();

        for edge in self.faces() {
            iter = iter.with(edge.cycle_iter());
        }

        iter
    }

    fn edge_iter(&self) -> Iter<Edge> {
        let mut iter = Iter::empty();

        for edge in self.faces() {
            iter = iter.with(edge.edge_iter());
        }

        iter
    }

    fn face_iter(&self) -> Iter<Face> {
        let mut iter = Iter::empty();

        for edge in self.faces() {
            iter = iter.with(edge.face_iter());
        }

        iter
    }

    fn global_vertex_iter(&self) -> Iter<GlobalVertex> {
        let mut iter = Iter::empty();

        for edge in self.faces() {
            iter = iter.with(edge.global_vertex_iter());
        }

        iter
    }

    fn sketch_iter(&self) -> Iter<Sketch> {
        Iter::from_object(self.clone())
    }

    fn solid_iter(&self) -> Iter<Solid> {
        let mut iter = Iter::empty();

        for edge in self.faces() {
            iter = iter.with(edge.solid_iter());
        }

        iter
    }

    fn surface_iter(&self) -> Iter<Surface> {
        let mut iter = Iter::empty();

        for edge in self.faces() {
            iter = iter.with(edge.surface_iter());
        }

        iter
    }

    fn vertex_iter(&self) -> Iter<Vertex> {
        let mut iter = Iter::empty();

        for edge in self.faces() {
            iter = iter.with(edge.vertex_iter());
        }

        iter
    }
}

impl ObjectIters for Solid {
    fn curve_iter(&self) -> Iter<Curve<3>> {
        let mut iter = Iter::empty();

        for edge in self.faces() {
            iter = iter.with(edge.curve_iter());
        }

        iter
    }

    fn cycle_iter(&self) -> Iter<Cycle> {
        let mut iter = Iter::empty();

        for edge in self.faces() {
            iter = iter.with(edge.cycle_iter());
        }

        iter
    }

    fn edge_iter(&self) -> Iter<Edge> {
        let mut iter = Iter::empty();

        for edge in self.faces() {
            iter = iter.with(edge.edge_iter());
        }

        iter
    }

    fn face_iter(&self) -> Iter<Face> {
        let mut iter = Iter::empty();

        for edge in self.faces() {
            iter = iter.with(edge.face_iter());
        }

        iter
    }

    fn global_vertex_iter(&self) -> Iter<GlobalVertex> {
        let mut iter = Iter::empty();

        for edge in self.faces() {
            iter = iter.with(edge.global_vertex_iter());
        }

        iter
    }

    fn sketch_iter(&self) -> Iter<Sketch> {
        let mut iter = Iter::empty();

        for edge in self.faces() {
            iter = iter.with(edge.sketch_iter());
        }

        iter
    }

    fn solid_iter(&self) -> Iter<Solid> {
        Iter::from_object(self.clone())
    }

    fn surface_iter(&self) -> Iter<Surface> {
        let mut iter = Iter::empty();

        for edge in self.faces() {
            iter = iter.with(edge.surface_iter());
        }

        iter
    }

    fn vertex_iter(&self) -> Iter<Vertex> {
        let mut iter = Iter::empty();

        for edge in self.faces() {
            iter = iter.with(edge.vertex_iter());
        }

        iter
    }
}

impl ObjectIters for Surface {
    fn curve_iter(&self) -> Iter<Curve<3>> {
        Iter::empty()
    }

    fn cycle_iter(&self) -> Iter<Cycle> {
        Iter::empty()
    }

    fn edge_iter(&self) -> Iter<Edge> {
        Iter::empty()
    }

    fn face_iter(&self) -> Iter<Face> {
        Iter::empty()
    }

    fn global_vertex_iter(&self) -> Iter<GlobalVertex> {
        Iter::empty()
    }

    fn sketch_iter(&self) -> Iter<Sketch> {
        Iter::empty()
    }

    fn solid_iter(&self) -> Iter<Solid> {
        Iter::empty()
    }

    fn surface_iter(&self) -> Iter<Surface> {
        Iter::from_object(*self)
    }

    fn vertex_iter(&self) -> Iter<Vertex> {
        Iter::empty()
    }
}

impl ObjectIters for Vertex {
    fn curve_iter(&self) -> Iter<Curve<3>> {
        self.global().curve_iter()
    }

    fn cycle_iter(&self) -> Iter<Cycle> {
        self.global().cycle_iter()
    }

    fn edge_iter(&self) -> Iter<Edge> {
        self.global().edge_iter()
    }

    fn face_iter(&self) -> Iter<Face> {
        self.global().face_iter()
    }

    fn global_vertex_iter(&self) -> Iter<GlobalVertex> {
        self.global().global_vertex_iter()
    }

    fn sketch_iter(&self) -> Iter<Sketch> {
        self.global().sketch_iter()
    }

    fn solid_iter(&self) -> Iter<Solid> {
        self.global().solid_iter()
    }

    fn surface_iter(&self) -> Iter<Surface> {
        self.global().surface_iter()
    }

    fn vertex_iter(&self) -> Iter<Vertex> {
        Iter::from_object(*self)
    }
}

// This implementation exists to paper over the lack of any "top-level" objects
// that are an entry point into a shape (basically, the lack of `Sketch` and
// `Solid`).
//
// It is also very useful in test code.
impl<T, O> ObjectIters for T
where
    for<'r> &'r T: IntoIterator<Item = &'r O>,
    O: ObjectIters,
{
    fn curve_iter(&self) -> Iter<Curve<3>> {
        let mut iter = Iter::empty();

        for object in self.into_iter() {
            iter = iter.with(object.curve_iter());
        }

        iter
    }

    fn cycle_iter(&self) -> Iter<Cycle> {
        let mut iter = Iter::empty();

        for object in self.into_iter() {
            iter = iter.with(object.cycle_iter());
        }

        iter
    }

    fn edge_iter(&self) -> Iter<Edge> {
        let mut iter = Iter::empty();

        for object in self.into_iter() {
            iter = iter.with(object.edge_iter());
        }

        iter
    }

    fn face_iter(&self) -> Iter<Face> {
        let mut iter = Iter::empty();

        for object in self.into_iter() {
            iter = iter.with(object.face_iter());
        }

        iter
    }

    fn global_vertex_iter(&self) -> Iter<GlobalVertex> {
        let mut iter = Iter::empty();

        for object in self.into_iter() {
            iter = iter.with(object.global_vertex_iter());
        }

        iter
    }

    fn sketch_iter(&self) -> Iter<Sketch> {
        let mut iter = Iter::empty();

        for object in self.into_iter() {
            iter = iter.with(object.sketch_iter());
        }

        iter
    }

    fn solid_iter(&self) -> Iter<Solid> {
        let mut iter = Iter::empty();

        for object in self.into_iter() {
            iter = iter.with(object.solid_iter());
        }

        iter
    }

    fn surface_iter(&self) -> Iter<Surface> {
        let mut iter = Iter::empty();

        for object in self.into_iter() {
            iter = iter.with(object.surface_iter());
        }

        iter
    }

    fn vertex_iter(&self) -> Iter<Vertex> {
        let mut iter = Iter::empty();

        for object in self.into_iter() {
            iter = iter.with(object.vertex_iter());
        }

        iter
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
        Curve, Cycle, Edge, Face, GlobalVertex, Sketch, Solid, Surface, Vertex,
    };

    use super::ObjectIters as _;

    #[test]
    fn curve() {
        let object = Curve::x_axis();

        assert_eq!(1, object.curve_iter().count());
        assert_eq!(0, object.cycle_iter().count());
        assert_eq!(0, object.edge_iter().count());
        assert_eq!(0, object.face_iter().count());
        assert_eq!(0, object.global_vertex_iter().count());
        assert_eq!(0, object.sketch_iter().count());
        assert_eq!(0, object.solid_iter().count());
        assert_eq!(0, object.surface_iter().count());
        assert_eq!(0, object.vertex_iter().count());
    }

    #[test]
    fn cycle() {
        let object = Cycle::polygon_from_points(
            &Surface::xy_plane(),
            [[0., 0.], [1., 0.], [0., 1.]],
        );

        assert_eq!(3, object.curve_iter().count());
        assert_eq!(1, object.cycle_iter().count());
        assert_eq!(3, object.edge_iter().count());
        assert_eq!(0, object.face_iter().count());
        assert_eq!(3, object.global_vertex_iter().count());
        assert_eq!(0, object.sketch_iter().count());
        assert_eq!(0, object.solid_iter().count());
        assert_eq!(0, object.surface_iter().count());
        assert_eq!(6, object.vertex_iter().count());
    }

    #[test]
    fn edge() {
        let object = Edge::line_segment_from_points(
            &Surface::xy_plane(),
            [[0., 0.], [1., 0.]],
        );

        assert_eq!(1, object.curve_iter().count());
        assert_eq!(0, object.cycle_iter().count());
        assert_eq!(1, object.edge_iter().count());
        assert_eq!(0, object.face_iter().count());
        assert_eq!(2, object.global_vertex_iter().count());
        assert_eq!(0, object.sketch_iter().count());
        assert_eq!(0, object.solid_iter().count());
        assert_eq!(0, object.surface_iter().count());
        assert_eq!(2, object.vertex_iter().count());
    }

    #[test]
    fn face() {
        let object = Face::builder(Surface::xy_plane())
            .with_exterior_polygon([[0., 0.], [1., 0.], [0., 1.]])
            .build();

        assert_eq!(3, object.curve_iter().count());
        assert_eq!(1, object.cycle_iter().count());
        assert_eq!(3, object.edge_iter().count());
        assert_eq!(1, object.face_iter().count());
        assert_eq!(3, object.global_vertex_iter().count());
        assert_eq!(0, object.sketch_iter().count());
        assert_eq!(0, object.solid_iter().count());
        assert_eq!(1, object.surface_iter().count());
        assert_eq!(6, object.vertex_iter().count());
    }

    #[test]
    fn global_vertex() {
        let object = GlobalVertex::from_position([0., 0., 0.]);

        assert_eq!(0, object.curve_iter().count());
        assert_eq!(0, object.cycle_iter().count());
        assert_eq!(0, object.edge_iter().count());
        assert_eq!(0, object.face_iter().count());
        assert_eq!(1, object.global_vertex_iter().count());
        assert_eq!(0, object.sketch_iter().count());
        assert_eq!(0, object.solid_iter().count());
        assert_eq!(0, object.surface_iter().count());
        assert_eq!(0, object.vertex_iter().count());
    }

    #[test]
    fn sketch() {
        let face = Face::builder(Surface::xy_plane())
            .with_exterior_polygon([[0., 0.], [1., 0.], [0., 1.]])
            .build();
        let object = Sketch::from_faces([face]);

        assert_eq!(3, object.curve_iter().count());
        assert_eq!(1, object.cycle_iter().count());
        assert_eq!(3, object.edge_iter().count());
        assert_eq!(1, object.face_iter().count());
        assert_eq!(3, object.global_vertex_iter().count());
        assert_eq!(1, object.sketch_iter().count());
        assert_eq!(0, object.solid_iter().count());
        assert_eq!(1, object.surface_iter().count());
        assert_eq!(6, object.vertex_iter().count());
    }

    #[test]
    fn solid() {
        let object = Solid::cube_from_edge_length(1.);

        assert_eq!(18, object.curve_iter().count());
        assert_eq!(6, object.cycle_iter().count());
        assert_eq!(20, object.edge_iter().count());
        assert_eq!(6, object.face_iter().count());
        assert_eq!(8, object.global_vertex_iter().count());
        assert_eq!(0, object.sketch_iter().count());
        assert_eq!(1, object.solid_iter().count());
        assert_eq!(6, object.surface_iter().count());
        assert_eq!(16, object.vertex_iter().count());
    }

    #[test]
    fn surface() {
        let object = Surface::xy_plane();

        assert_eq!(0, object.curve_iter().count());
        assert_eq!(0, object.cycle_iter().count());
        assert_eq!(0, object.edge_iter().count());
        assert_eq!(0, object.face_iter().count());
        assert_eq!(0, object.global_vertex_iter().count());
        assert_eq!(0, object.sketch_iter().count());
        assert_eq!(0, object.solid_iter().count());
        assert_eq!(1, object.surface_iter().count());
        assert_eq!(0, object.vertex_iter().count());
    }

    #[test]
    fn vertex() {
        let global_vertex = GlobalVertex::from_position([0., 0., 0.]);
        let object = Vertex::new([0.], global_vertex);

        assert_eq!(0, object.curve_iter().count());
        assert_eq!(0, object.cycle_iter().count());
        assert_eq!(0, object.edge_iter().count());
        assert_eq!(0, object.face_iter().count());
        assert_eq!(1, object.global_vertex_iter().count());
        assert_eq!(0, object.sketch_iter().count());
        assert_eq!(0, object.solid_iter().count());
        assert_eq!(0, object.surface_iter().count());
        assert_eq!(1, object.vertex_iter().count());
    }
}
