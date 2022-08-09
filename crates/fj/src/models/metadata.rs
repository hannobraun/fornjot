use abi_stable::std_types::{ROption, RString, RVec};

/// Information about a particular module that can be used by the host for
/// things like introspection and search.
#[derive(Debug, Clone, PartialEq, Eq, abi_stable::StableAbi)]
#[repr(C)]
pub struct Metadata {
    /// A short, human-friendly name used to identify this module.
    pub name: RString,
    /// A semver-compliant version number.
    pub version: RString,
    /// A short, one-line description.
    pub short_description: ROption<RString>,
    /// A more elaborate description.
    pub description: ROption<RString>,
    /// A link to the homepage.
    pub homepage: ROption<RString>,
    /// A link to the source code.
    pub repository: ROption<RString>,
    /// The name of the software license(s) this software is released under.
    ///
    /// This is interpreted as a SPDX license expression (e.g.  `MIT OR
    /// Apache-2.0`). See [the SPDX site][spdx] for more information.
    ///
    /// [spdx]: https://spdx.dev/spdx-specification-21-web-version/#h.jxpfx0ykyb60
    pub license: ROption<RString>,
}

impl Metadata {
    /// Create a [`Metadata`] object with the bare minimum required fields.
    ///
    /// # Panics
    ///
    /// The `name` and `version` fields must not be empty.
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        let name = name.into();
        assert!(!name.is_empty());
        let version = version.into();
        assert!(!version.is_empty());

        Metadata {
            name: name.into(),
            version: version.into(),
            short_description: ROption::RNone,
            description: ROption::RNone,
            homepage: ROption::RNone,
            repository: ROption::RNone,
            license: ROption::RNone,
        }
    }

    /// Set the [`Metadata::short_description`] field.
    pub fn with_short_description(
        self,
        short_description: impl Into<String>,
    ) -> Self {
        let short_description = RString::from(short_description.into());
        if short_description.is_empty() {
            return self;
        }

        Metadata {
            short_description: Some(short_description).into(),
            ..self
        }
    }

    /// Set the [`Metadata::description`] field.
    pub fn with_description(self, description: impl Into<String>) -> Self {
        let description = RString::from(description.into());
        if description.is_empty() {
            return self;
        }

        Metadata {
            description: Some(description).into(),
            ..self
        }
    }

    /// Set the [`Metadata::homepage`] field.
    pub fn with_homepage(self, homepage: impl Into<String>) -> Self {
        let homepage = RString::from(homepage.into());
        if homepage.is_empty() {
            return self;
        }

        Metadata {
            homepage: Some(homepage).into(),
            ..self
        }
    }

    /// Set the [`Metadata::repository`] field.
    pub fn with_repository(self, repository: impl Into<String>) -> Self {
        let repository = RString::from(repository.into());
        if repository.is_empty() {
            return self;
        }

        Metadata {
            repository: Some(repository).into(),
            ..self
        }
    }

    /// Set the [`Metadata::license`] field.
    pub fn with_license(self, license: impl Into<String>) -> Self {
        let license = RString::from(license.into());
        if license.is_empty() {
            return self;
        }

        Metadata {
            license: Some(license).into(),
            ..self
        }
    }
}

/// Metadata about a [`crate::models::Model`].
#[derive(Debug, Clone, PartialEq, abi_stable::StableAbi)]
#[repr(C)]
pub struct ModelMetadata {
    /// A short, human-friendly name used to identify this model.
    pub name: RString,
    /// A description of what this model does.
    pub description: ROption<RString>,
    /// Arguments that the model uses when calculating its geometry.
    pub arguments: RVec<ArgumentMetadata>,
}

impl ModelMetadata {
    /// Create metadata for a model.
    ///
    /// # Panics
    ///
    /// The `name` must not be empty.
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        assert!(!name.is_empty());

        ModelMetadata {
            name: RString::from(name),
            description: ROption::RNone,
            arguments: RVec::new(),
        }
    }

    /// Set the [`ModelMetadata::description`].
    pub fn with_description(self, description: impl Into<String>) -> Self {
        let description = RString::from(description.into());
        if description.is_empty() {
            return self;
        }

        ModelMetadata {
            description: Some(description).into(),
            ..self
        }
    }

    /// Add an argument to the [`ModelMetadata::arguments`] list.
    ///
    /// As a convenience, string literals can be automatically converted into
    /// [`ArgumentMetadata`] with no description or default value.
    pub fn with_argument(mut self, arg: impl Into<ArgumentMetadata>) -> Self {
        self.arguments.push(arg.into());
        self
    }
}

/// Metadata describing a model's argument.
#[derive(Debug, Clone, PartialEq, abi_stable::StableAbi)]
#[repr(C)]
pub struct ArgumentMetadata {
    /// The name used to refer to this argument.
    pub name: RString,
    /// A short description of this argument that could be shown to the user
    /// in something like a tooltip.
    pub description: ROption<RString>,
    /// Something that could be used as a default if no value was provided.
    pub default_value: ROption<RString>,
}

impl ArgumentMetadata {
    /// Create a new [`ArgumentMetadata`].
    ///
    /// # Panics
    ///
    /// The `name` must not be empty.
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        assert!(!name.is_empty());
        ArgumentMetadata {
            name: name.into(),
            description: ROption::RNone,
            default_value: ROption::RNone,
        }
    }

    /// Set the [`ArgumentMetadata::description`].
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        let description = RString::from(description.into());
        if description.is_empty() {
            return self;
        }

        self.description = Some(description).into();
        self
    }

    /// Set the [`ArgumentMetadata::default_value`].
    pub fn with_default_value(
        mut self,
        default_value: impl Into<String>,
    ) -> Self {
        self.default_value =
            ROption::RSome(RString::from(default_value.into()));
        self
    }
}

impl From<&str> for ArgumentMetadata {
    fn from(name: &str) -> Self {
        ArgumentMetadata::new(name)
    }
}
