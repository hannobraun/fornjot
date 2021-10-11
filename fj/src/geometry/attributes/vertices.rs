use crate::{
    geometry::{operations, shapes},
    math::Vector,
};

use super::Path;

/// The vertices that make up a shape
///
/// `D` defines the dimension of the vertices' positions.
pub trait Vertices<const D: usize> {
    /// Return the vertices of the shape
    fn vertices(&self) -> Vec<operations::Translate<shapes::Vertex, D>>;
}

impl<const D: usize> Vertices<D> for shapes::Vertex {
    fn vertices(&self) -> Vec<operations::Translate<shapes::Vertex, D>> {
        vec![operations::Translate {
            shape: *self,
            offset: Vector::zeros(),
        }]
    }
}

impl<T, P, const D: usize> Vertices<D> for operations::Sweep<T, P>
where
    T: Vertices<D>,
    P: Path<D>,
{
    fn vertices(&self) -> Vec<operations::Translate<shapes::Vertex, D>> {
        let mut vertices = self.shape.vertices().clone();

        for mut vertex in self.shape.vertices() {
            vertex.offset += self.path.path();
            vertices.push(vertex);
        }

        vertices
    }
}

impl<T, const D: usize> Vertices<D> for operations::Translate<T, D>
where
    T: Vertices<D>,
{
    fn vertices(&self) -> Vec<operations::Translate<shapes::Vertex, D>> {
        let mut vertices = self.shape.vertices();

        for vertex in &mut vertices {
            vertex.offset += self.offset;
        }

        vertices
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::vector;

    use crate::{
        geometry::{operations::Translate, shapes::Vertex},
        prelude::*,
    };

    use super::Vertices;

    #[test]
    fn for_vertex() {
        assert_eq!(
            <Vertex as Vertices<0>>::vertices(&Vertex),
            [Vertex.translate(vector![])],
        );
        assert_eq!(
            <Vertex as Vertices<1>>::vertices(&Vertex),
            [Vertex.translate(vector![0.])],
        );
        assert_eq!(
            <Vertex as Vertices<2>>::vertices(&Vertex),
            [Vertex.translate(vector![0., 0.])],
        );
        assert_eq!(
            <Vertex as Vertices<3>>::vertices(&Vertex),
            [Vertex.translate(vector![0., 0., 0.])],
        );
    }

    #[test]
    fn test_vertices_for_sweep() {
        let vertices = MockVertices([
            Vertex.translate(vector![1., 2., 3.]),
            Vertex.translate(vector![2., 3., 4.]),
        ]);

        let sweep = vertices.sweep(vector![3., 2., 1.]);

        assert_eq!(
            sweep.vertices(),
            [
                Vertex.translate(vector![1., 2., 3.]),
                Vertex.translate(vector![2., 3., 4.]),
                Vertex.translate(vector![4., 4., 4.]),
                Vertex.translate(vector![5., 5., 5.]),
            ],
        );
    }

    #[test]
    fn test_vertices_for_translate() {
        let vertex_1d = Vertex.translate(vector![2.]);
        let vertex_2d = Vertex.translate(vector![2., 3.]);
        let vertex_3d = Vertex.translate(vector![2., 3., 4.]);

        assert_eq!(vertex_1d.vertices(), [vertex_1d]);
        assert_eq!(vertex_2d.vertices(), [vertex_2d]);
        assert_eq!(vertex_3d.vertices(), [vertex_3d]);
    }

    struct MockVertices<const N: usize>([Translate<Vertex, 3>; N]);

    impl<const N: usize> Vertices<3> for MockVertices<N> {
        fn vertices(
            &self,
        ) -> Vec<
            crate::geometry::operations::Translate<
                crate::geometry::shapes::Vertex,
                3,
            >,
        > {
            self.0.into()
        }
    }
}
