use crate::storage::Handle;

/// Recursively replace a (partial) object referenced by another partial object
pub trait Replace<T> {
    /// Recursively replace the referenced object
    fn replace(&mut self, object: Handle<T>) -> &mut Self;
}

impl<T, R, const N: usize> Replace<T> for [R; N]
where
    R: Replace<T>,
{
    fn replace(&mut self, object: Handle<T>) -> &mut Self {
        for item in self.iter_mut() {
            item.replace(object.clone());
        }

        self
    }
}
