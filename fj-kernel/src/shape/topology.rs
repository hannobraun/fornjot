use std::marker::PhantomData;

use crate::topology::{Cycle, Edge, Face, Vertex};

use super::{stores::Stores, Iter};

/// The vertices of a shape
pub struct Topology<'r> {
    pub(super) stores: Stores,
    pub(super) _lifetime: PhantomData<&'r ()>,
}

impl Topology<'_> {
    /// Access iterator over all vertices
    ///
    /// The caller must not make any assumptions about the order of vertices.
    pub fn vertices(&self) -> Iter<Vertex> {
        self.stores.vertices.iter()
    }

    /// Access iterator over all edges
    ///
    /// The caller must not make any assumptions about the order of edges.
    pub fn edges(&self) -> Iter<Edge> {
        self.stores.edges.iter()
    }

    /// Access an iterator over all cycles
    ///
    /// The caller must not make any assumptions about the order of cycles.
    pub fn cycles(&self) -> Iter<Cycle> {
        self.stores.cycles.iter()
    }

    /// Access an iterator over all faces
    ///
    /// The caller must not make any assumptions about the order of faces.
    pub fn faces(&self) -> Iter<Face> {
        self.stores.faces.iter()
    }
}

#[cfg(test)]
mod tests {
    use std::ops::{Deref, DerefMut};

    use fj_math::{Point, Scalar};

    use crate::{
        geometry::{Curve, Surface},
        shape::{Handle, Shape, ValidationError},
        topology::{Cycle, Edge, Face, Vertex},
    };

    const MIN_DISTANCE: f64 = 5e-7;

    #[test]
    fn add_vertex() -> anyhow::Result<()> {
        let mut shape = Shape::new().with_min_distance(MIN_DISTANCE);
        let mut other = Shape::new();

        let point = shape.insert(Point::from([0., 0., 0.]))?;
        shape.insert(Vertex { point })?;

        // Should fail, as `point` is not part of the shape.
        let point = other.insert(Point::from([1., 0., 0.]))?;
        let result = shape.insert(Vertex { point });
        assert!(matches!(result, Err(ValidationError::Structural(_))));

        // `point` is too close to the original point. `assert!` is commented,
        // because that only causes a warning to be logged right now.
        let point = shape.insert(Point::from([5e-8, 0., 0.]))?;
        let result = shape.insert(Vertex { point });
        assert!(matches!(result, Err(ValidationError::Uniqueness)));

        // `point` is farther than `MIN_DISTANCE` away from original point.
        // Should work.
        let point = shape.insert(Point::from([5e-6, 0., 0.]))?;
        shape.insert(Vertex { point })?;

        Ok(())
    }

    #[test]
    fn add_edge() -> anyhow::Result<()> {
        let mut shape = TestShape::new();
        let mut other = TestShape::new();

        let curve = other.add_curve();
        let a = Vertex::build(&mut other).from_point([1., 0., 0.])?;
        let b = Vertex::build(&mut other).from_point([2., 0., 0.])?;

        // Shouldn't work. Nothing has been added to `shape`.
        let err = shape
            .insert(Edge {
                curve: curve.clone(),
                vertices: Some([a.clone(), b.clone()]),
            })
            .unwrap_err();
        assert!(err.missing_curve(&curve));
        assert!(err.missing_vertex(&a));
        assert!(err.missing_vertex(&b));

        let curve = shape.add_curve();
        let a = Vertex::build(&mut shape).from_point([1., 0., 0.])?;
        let b = Vertex::build(&mut shape).from_point([2., 0., 0.])?;

        // Everything has been added to `shape` now. Should work!
        shape.insert(Edge {
            curve,
            vertices: Some([a, b]),
        })?;

        Ok(())
    }

    #[test]
    fn add_cycle() -> anyhow::Result<()> {
        let mut shape = TestShape::new();
        let mut other = TestShape::new();

        // Trying to refer to edge that is not from the same shape. Should fail.
        let edge = other.add_edge()?;
        let err = shape
            .insert(Cycle {
                edges: vec![edge.clone()],
            })
            .unwrap_err();
        assert!(err.missing_edge(&edge));

        // Referring to edge that *is* from the same shape. Should work.
        let edge = shape.add_edge()?;
        shape.insert(Cycle { edges: vec![edge] })?;

        Ok(())
    }

    #[test]
    fn add_face() -> anyhow::Result<()> {
        let mut shape = TestShape::new();
        let mut other = TestShape::new();

        let surface = other.add_surface();
        let cycle = other.add_cycle()?;

        // Nothing has been added to `shape`. Should fail.
        let err = shape
            .insert(Face::Face {
                surface: surface.clone(),
                exteriors: vec![cycle.clone()],
                interiors: Vec::new(),
                color: [255, 0, 0, 255],
            })
            .unwrap_err();
        assert!(err.missing_surface(&surface));
        assert!(err.missing_cycle(&cycle));

        let surface = shape.add_surface();
        let cycle = shape.add_cycle()?;

        // Everything has been added to `shape` now. Should work!
        shape.insert(Face::Face {
            surface,
            exteriors: vec![cycle],
            interiors: Vec::new(),
            color: [255, 0, 0, 255],
        })?;

        Ok(())
    }

    struct TestShape {
        inner: Shape,
        next_point: Point<3>,
    }

    impl TestShape {
        fn new() -> Self {
            Self {
                inner: Shape::new(),
                next_point: Point::from([0., 0., 0.]),
            }
        }

        fn add_curve(&mut self) -> Handle<Curve> {
            self.insert(Curve::x_axis()).unwrap()
        }

        fn add_surface(&mut self) -> Handle<Surface> {
            self.insert(Surface::x_y_plane()).unwrap()
        }

        fn add_edge(&mut self) -> anyhow::Result<Handle<Edge>> {
            let vertices = [(); 2].map(|()| {
                let point = self.next_point;
                self.next_point.x += Scalar::ONE;

                let point = self.insert(point).unwrap();
                self.insert(Vertex { point }).unwrap()
            });
            let edge = Edge::build(&mut self.inner)
                .line_segment_from_vertices(vertices)?;

            Ok(edge)
        }

        fn add_cycle(&mut self) -> anyhow::Result<Handle<Cycle>> {
            let edge = self.add_edge()?;
            let cycle = self.insert(Cycle { edges: vec![edge] })?;
            Ok(cycle)
        }
    }

    impl Deref for TestShape {
        type Target = Shape;

        fn deref(&self) -> &Self::Target {
            &self.inner
        }
    }

    impl DerefMut for TestShape {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.inner
        }
    }
}
