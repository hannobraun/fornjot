//! API for iterating over the objects of a shape, or part of a shape

use std::collections::VecDeque;

use crate::{
    objects::{Curve, Cycle, Edge, Face, Surface, Vertex},
    shape::Shape,
};

/// Access iterators over all objects of a shape, or part of it
///
/// Implemented for all object types. An implementation must return itself, in
/// addition to any other objects it references.
pub trait ObjectIters {
    /// Iterate over all curves
    fn curve_iter(&self) -> Iter<Curve<3>>;

    /// Iterate over all cycles
    fn cycle_iter(&self) -> Iter<Cycle<3>>;

    /// Iterate over all edges
    fn edge_iter(&self) -> Iter<Edge<3>>;

    /// Iterate over all faces
    fn face_iter(&self) -> Iter<Face>;

    /// Iterate over all surfaces
    fn surface_iter(&self) -> Iter<Surface>;

    /// Iterate over all vertices
    fn vertex_iter(&self) -> Iter<Vertex>;
}

impl ObjectIters for Curve<3> {
    fn curve_iter(&self) -> Iter<Curve<3>> {
        Iter::from_object(*self)
    }

    fn cycle_iter(&self) -> Iter<Cycle<3>> {
        Iter::empty()
    }

    fn edge_iter(&self) -> Iter<Edge<3>> {
        Iter::empty()
    }

    fn face_iter(&self) -> Iter<Face> {
        Iter::empty()
    }

    fn surface_iter(&self) -> Iter<Surface> {
        Iter::empty()
    }

    fn vertex_iter(&self) -> Iter<Vertex> {
        Iter::empty()
    }
}

impl ObjectIters for Cycle<3> {
    fn curve_iter(&self) -> Iter<Curve<3>> {
        let mut iter = Iter::empty();

        for edge in self.edges() {
            iter = iter.with(edge.curve_iter());
        }

        iter
    }

    fn cycle_iter(&self) -> Iter<Cycle<3>> {
        Iter::from_object(self.clone())
    }

    fn edge_iter(&self) -> Iter<Edge<3>> {
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

impl ObjectIters for Edge<3> {
    fn curve_iter(&self) -> Iter<Curve<3>> {
        let mut iter = Iter::empty().with(self.curve().curve_iter());

        for vertex in self.vertices().into_iter().flatten() {
            iter = iter.with(vertex.curve_iter());
        }

        iter
    }

    fn cycle_iter(&self) -> Iter<Cycle<3>> {
        let mut iter = Iter::empty().with(self.curve().cycle_iter());

        for vertex in self.vertices().into_iter().flatten() {
            iter = iter.with(vertex.cycle_iter());
        }

        iter
    }

    fn edge_iter(&self) -> Iter<Edge<3>> {
        Iter::from_object(self.clone())
    }

    fn face_iter(&self) -> Iter<Face> {
        let mut iter = Iter::empty().with(self.curve().face_iter());

        for vertex in self.vertices().into_iter().flatten() {
            iter = iter.with(vertex.face_iter());
        }

        iter
    }

    fn surface_iter(&self) -> Iter<Surface> {
        let mut iter = Iter::empty().with(self.curve().surface_iter());

        for vertex in self.vertices().into_iter().flatten() {
            iter = iter.with(vertex.surface_iter());
        }

        iter
    }

    fn vertex_iter(&self) -> Iter<Vertex> {
        let mut iter = Iter::empty().with(self.curve().vertex_iter());

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

    fn cycle_iter(&self) -> Iter<Cycle<3>> {
        if let Face::Face(face) = self {
            let mut iter = Iter::empty().with(face.surface().cycle_iter());

            for cycle in face.all_cycles() {
                iter = iter.with(cycle.cycle_iter());
            }

            return iter;
        }

        Iter::empty()
    }

    fn edge_iter(&self) -> Iter<Edge<3>> {
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

impl ObjectIters for Surface {
    fn curve_iter(&self) -> Iter<Curve<3>> {
        Iter::empty()
    }

    fn cycle_iter(&self) -> Iter<Cycle<3>> {
        Iter::empty()
    }

    fn edge_iter(&self) -> Iter<Edge<3>> {
        Iter::empty()
    }

    fn face_iter(&self) -> Iter<Face> {
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
        Iter::empty()
    }

    fn cycle_iter(&self) -> Iter<Cycle<3>> {
        Iter::empty()
    }

    fn edge_iter(&self) -> Iter<Edge<3>> {
        Iter::empty()
    }

    fn face_iter(&self) -> Iter<Face> {
        Iter::empty()
    }

    fn surface_iter(&self) -> Iter<Surface> {
        Iter::empty()
    }

    fn vertex_iter(&self) -> Iter<Vertex> {
        Iter::from_object(*self)
    }
}

// This implementation exists to ease the transition away from `Shape`. It will
// likely be removed at some point, together with `Shape`.
impl ObjectIters for Shape {
    fn curve_iter(&self) -> Iter<Curve<3>> {
        Iter::from_iter(self.curves().map(|handle| handle.get()))
    }

    fn cycle_iter(&self) -> Iter<Cycle<3>> {
        Iter::from_iter(self.cycles().map(|handle| handle.get()))
    }

    fn edge_iter(&self) -> Iter<Edge<3>> {
        Iter::from_iter(self.edges().map(|handle| handle.get()))
    }

    fn face_iter(&self) -> Iter<Face> {
        Iter::from_iter(self.faces().map(|handle| handle.get()))
    }

    fn surface_iter(&self) -> Iter<Surface> {
        Iter::from_iter(self.surfaces().map(|handle| handle.get()))
    }

    fn vertex_iter(&self) -> Iter<Vertex> {
        Iter::from_iter(self.vertices().map(|handle| handle.get()))
    }
}

// This implementation exists to paper over the lack of any "top-level" objects
// that are an entry point into a shape (basically, the lack of `Sketch` and
// `Solid`).
impl<T> ObjectIters for T
where
    for<'r> &'r T: IntoIterator<Item = &'r Face>,
{
    fn curve_iter(&self) -> Iter<Curve<3>> {
        let mut iter = Iter::empty();

        for face in self.into_iter() {
            iter = iter.with(face.curve_iter());
        }

        iter
    }

    fn cycle_iter(&self) -> Iter<Cycle<3>> {
        let mut iter = Iter::empty();

        for face in self.into_iter() {
            iter = iter.with(face.cycle_iter());
        }

        iter
    }

    fn edge_iter(&self) -> Iter<Edge<3>> {
        let mut iter = Iter::empty();

        for face in self.into_iter() {
            iter = iter.with(face.edge_iter());
        }

        iter
    }

    fn face_iter(&self) -> Iter<Face> {
        let mut iter = Iter::empty();

        for face in self.into_iter() {
            iter = iter.with(face.face_iter());
        }

        iter
    }

    fn surface_iter(&self) -> Iter<Surface> {
        let mut iter = Iter::empty();

        for face in self.into_iter() {
            iter = iter.with(face.surface_iter());
        }

        iter
    }

    fn vertex_iter(&self) -> Iter<Vertex> {
        let mut iter = Iter::empty();

        for face in self.into_iter() {
            iter = iter.with(face.vertex_iter());
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

    fn from_iter(iter: impl IntoIterator<Item = T>) -> Self {
        let mut objects = VecDeque::new();
        objects.extend(iter);
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
    use crate::{
        objects::{Curve, Cycle, Edge, Face, Surface, Vertex},
        shape::Shape,
    };

    use super::ObjectIters as _;

    #[test]
    fn curve() {
        let curve = Curve::x_axis();

        assert_eq!(1, curve.curve_iter().count());
        assert_eq!(0, curve.cycle_iter().count());
        assert_eq!(0, curve.edge_iter().count());
        assert_eq!(0, curve.face_iter().count());
        assert_eq!(0, curve.surface_iter().count());
        assert_eq!(0, curve.vertex_iter().count());
    }

    #[test]
    fn cycle() {
        let mut shape = Shape::new();
        let cycle = Cycle::builder(Surface::xy_plane(), &mut shape)
            .build_polygon([[0., 0.], [1., 0.], [0., 1.]])
            .canonical()
            .get();

        assert_eq!(3, cycle.curve_iter().count());
        assert_eq!(1, cycle.cycle_iter().count());
        assert_eq!(3, cycle.edge_iter().count());
        assert_eq!(0, cycle.face_iter().count());
        assert_eq!(0, cycle.surface_iter().count());
        assert_eq!(3, cycle.vertex_iter().count());
    }

    #[test]
    fn edge() {
        let mut shape = Shape::new();
        let edge = Edge::builder(&mut shape)
            .build_line_segment_from_points([[0., 0., 0.], [1., 0., 0.]])
            .get();

        assert_eq!(1, edge.curve_iter().count());
        assert_eq!(0, edge.cycle_iter().count());
        assert_eq!(1, edge.edge_iter().count());
        assert_eq!(0, edge.face_iter().count());
        assert_eq!(0, edge.surface_iter().count());
        assert_eq!(2, edge.vertex_iter().count());
    }

    #[test]
    fn face() {
        let mut shape = Shape::new();
        let face = Face::builder(Surface::xy_plane(), &mut shape)
            .with_exterior_polygon([[0., 0.], [1., 0.], [0., 1.]])
            .build()
            .get();

        assert_eq!(3, face.curve_iter().count());
        assert_eq!(1, face.cycle_iter().count());
        assert_eq!(3, face.edge_iter().count());
        assert_eq!(1, face.face_iter().count());
        assert_eq!(1, face.surface_iter().count());
        assert_eq!(3, face.vertex_iter().count());
    }

    #[test]
    fn surface() {
        let surface = Surface::xy_plane();

        assert_eq!(0, surface.curve_iter().count());
        assert_eq!(0, surface.cycle_iter().count());
        assert_eq!(0, surface.edge_iter().count());
        assert_eq!(0, surface.face_iter().count());
        assert_eq!(1, surface.surface_iter().count());
        assert_eq!(0, surface.vertex_iter().count());
    }

    #[test]
    fn vertex() {
        let mut shape = Shape::new();
        let vertex = Vertex::builder(&mut shape)
            .build_from_point([0., 0., 0.])
            .get();

        assert_eq!(0, vertex.curve_iter().count());
        assert_eq!(0, vertex.cycle_iter().count());
        assert_eq!(0, vertex.edge_iter().count());
        assert_eq!(0, vertex.face_iter().count());
        assert_eq!(0, vertex.surface_iter().count());
        assert_eq!(1, vertex.vertex_iter().count());
    }
}
