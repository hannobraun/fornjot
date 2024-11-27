//! # The operations log and supporting infrastructure
//!
//! See [`OpsLog`].

use std::fmt;

use tuples::CombinRight;

use super::{Operation, Triangle, Vertex};

/// # The operations that make up geometry, ordered in a specific sequence
///
/// This is the entry point to the API that is used to create geometry. You
/// create an instance of [`OpsLog`], call its methods to create operations, and
/// can later look at the final result, or any intermediate step.
///
/// A core idea here is that operations are the primitive that make up any
/// geometry. This is different from mainline Fornjot code, in which geometry
/// (and topology) is made up of *objects* instead.
///
/// In this new model, objects and operations are the same thing. The most basic
/// operations just create a single object. More complex operations could
/// combine multiple operations (or objects) into higher-level operations (or
/// objects). But they all present [through the same interface](Operation).
///
/// Right now, the only operations available are [`Vertex`] and [`Triangle`].
/// [`Vertex`] is the lower-level of the two, while [`Triangle`] builds on top
/// of it to create a higher-level operation/object.
///
/// It might be worth noting that [`OpsLog`] is itself an operation (i.e. it
/// implements [`Operation`]). It is simply the operation that unifies all the
/// operations that were added to it, and creates the uniform intermediate
/// representation of the complete shape.
///
/// ## Adding operations
///
/// Operations are added using the [`OpsLog::vertex`] and [`OpsLog::triangle`]
/// methods. Later prototypes might move these to extension traits or something,
/// to support arbitrary operations, but here those are hardcoded and the only
/// ones supported.
///
/// Those methods employ some trickery in the form of the [`OperationResult`]
/// return value, which allows the caller both to chain calls to add multiple
/// objects in a row, but then also access every single operation created in a
/// convenient way.
///
/// Check out the code of [`crate::model::model`] to see how that looks. I'm
/// pretty happy with how it turned out, but we'll have to see how well it
/// scales, once arbitrary (user-defined) operations have to be supported.
#[derive(Default)]
pub struct OpsLog {
    /// # The operations
    ///
    /// This doesn't store the operations directly. The goal here is to provide
    /// access to every intermediate state that the geometry construction went
    /// through, and storing operations directly won't achieve that.
    ///
    /// For example, if you were to look at the second-to-last operation in
    /// isolation, you wouldn't see the whole shape before the last operation,
    /// but only the single triangle or something that made up this
    /// second-to-last operation.
    ///
    /// So what this does instead, is wrap operations into
    /// [`OperationInSequence`], which knows about all previous operations and
    /// can provide the intermediate state.
    pub operations: Vec<OperationInSequence>,

    /// # Which operation is currently selected
    ///
    /// This is a UI concept and this field probably shouldn't live here. For
    /// now, it still does though.
    pub selected: usize,
}

impl OpsLog {
    /// # Add a vertex
    ///
    /// See documentation of [`OpsLog`] for context.
    pub fn vertex(
        &mut self,
        vertex: impl Into<Vertex>,
    ) -> OperationResult<(Vertex,)> {
        let vertex = vertex.into();

        self.operations.push(OperationInSequence {
            operation: ClonedOperation::from_op(&vertex),
            previous: self
                .operations
                .last()
                .map(|op| ClonedOperation::from_op(op)),
        });

        OperationResult {
            operations: self,
            results: (vertex,),
        }
    }

    /// # Add a triangle
    ///
    /// See documentation of [`OpsLog`] for context.
    pub fn triangle(
        &mut self,
        triangle: impl Into<Triangle>,
    ) -> OperationResult<(Triangle,)> {
        let triangle = triangle.into();

        self.operations.push(OperationInSequence {
            operation: ClonedOperation::from_op(&triangle),
            previous: self
                .operations
                .last()
                .map(|op| ClonedOperation::from_op(op)),
        });

        OperationResult {
            operations: self,
            results: (triangle,),
        }
    }

    /// # Used by the UI; not interesting
    pub fn select_last(&mut self) {
        self.selected = self.operations.len().saturating_sub(1);
    }

    /// # Used by the UI; not interesting
    pub fn select_next(&mut self) {
        if self.selected < self.operations.len() {
            self.selected += 1;
        }
    }

    /// # Used by the UI; not interesting
    pub fn select_previous(&mut self) {
        self.selected = self.selected.saturating_sub(1);
    }

    /// # Used by the UI; not interesting
    pub fn selected(&self) -> Option<&dyn Operation> {
        self.operations.get(self.selected).map(|op| op as &_)
    }
}

impl fmt::Display for OpsLog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(op) = self.operations.last() {
            op.fmt(f)
        } else {
            write!(f, "empty operations log")
        }
    }
}

impl Operation for OpsLog {
    fn vertices(&self, vertices: &mut Vec<Vertex>) {
        if let Some(op) = self.operations.last() {
            op.vertices(vertices);
        }
    }

    fn triangles(&self, triangles: &mut Vec<Triangle>) {
        if let Some(op) = self.operations.last() {
            op.triangles(triangles);
        }
    }
}

/// # Representation of an intermediate state in constructing a shape
///
/// This is a wrapper around an operation, but it also knows about the previous
/// operation in the sequence (which itself is expected to know about its
/// previous operation).
///
/// [`OperationInSequence`] is used by [`OpsLog`] to provide the ability to look
/// at every intermediate state.
///
/// ## Efficiency
///
/// This is implemented in a rather inefficient way, by cloning the uniform
/// representation of the operations it references, via [`ClonedOperation`].
///
/// This is fine within the context of this experiment, and I'm not too worried
/// about it long-term either. Operations can live in centralized stores, which
/// can return handles to refer to them. Similar to what current mainline
/// Fornjot does with its topological objects.
///
/// I'm not sure if that would be the best way to do it, but at least it
/// wouldn't create multiple clones of every operation.
pub struct OperationInSequence {
    pub operation: ClonedOperation,
    pub previous: Option<ClonedOperation>,
}

impl Operation for OperationInSequence {
    fn vertices(&self, vertices: &mut Vec<Vertex>) {
        if let Some(op) = &self.previous {
            op.vertices(vertices);
        }
        self.operation.vertices(vertices);
    }

    fn triangles(&self, triangles: &mut Vec<Triangle>) {
        if let Some(op) = &self.previous {
            op.triangles(triangles);
        }
        self.operation.triangles(triangles);
    }
}

impl fmt::Display for OperationInSequence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.operation.fmt(f)
    }
}

/// # Allow chaining calls to create operations, but also access all results
///
/// This is an implementation details of [`OpsLog`]. See documentation on adding
/// operations there.
pub struct OperationResult<'r, T> {
    operations: &'r mut OpsLog,
    results: T,
}

impl<'r, T> OperationResult<'r, T> {
    pub fn vertex(
        self,
        vertex: impl Into<Vertex>,
    ) -> OperationResult<'r, T::Out>
    where
        T: CombinRight<Vertex>,
    {
        let OperationResult {
            results: (vertex,), ..
        } = self.operations.vertex(vertex);

        OperationResult {
            operations: self.operations,
            results: self.results.push_right(vertex),
        }
    }

    pub fn triangle(
        self,
        triangle: impl Into<Triangle>,
    ) -> OperationResult<'r, T::Out>
    where
        T: CombinRight<Triangle>,
    {
        let OperationResult {
            results: (triangle,),
            ..
        } = self.operations.triangle(triangle);

        OperationResult {
            operations: self.operations,
            results: self.results.push_right(triangle),
        }
    }

    pub fn results(self) -> T {
        self.results
    }
}

/// # Implementation details of [`OperationInSequence`]
pub struct ClonedOperation {
    pub description: String,
    pub vertices: Vec<Vertex>,
    pub triangles: Vec<Triangle>,
}

impl ClonedOperation {
    pub fn from_op(op: &dyn Operation) -> Self {
        let mut vertices = Vec::new();
        let mut triangles = Vec::new();

        op.vertices(&mut vertices);
        op.triangles(&mut triangles);

        Self {
            description: op.to_string(),
            vertices,
            triangles,
        }
    }
}

impl fmt::Display for ClonedOperation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl Operation for ClonedOperation {
    fn vertices(&self, vertices: &mut Vec<Vertex>) {
        vertices.extend(&self.vertices);
    }

    fn triangles(&self, triangles: &mut Vec<Triangle>) {
        triangles.extend(&self.triangles);
    }
}
