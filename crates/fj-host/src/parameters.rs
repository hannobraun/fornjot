use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

/// Parameters that are passed to a model.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Parameters(pub HashMap<String, String>);

impl Parameters {
    /// Construct an empty instance of `Parameters`
    pub fn empty() -> Self {
        Self(HashMap::new())
    }

    /// Insert a value into the [`Parameters`] dictionary, implicitly converting
    /// the arguments to strings and returning `&mut self` to enable chaining.
    pub fn insert(
        &mut self,
        key: impl Into<String>,
        value: impl ToString,
    ) -> &mut Self {
        self.0.insert(key.into(), value.to_string());
        self
    }
}

impl Deref for Parameters {
    type Target = HashMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Parameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
