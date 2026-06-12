//! # Custom testing infrastructure

mod operations;

/// # Return all tests
pub fn all() -> &'static [fn()] {
    &[
        operations::sketch::circle,
        operations::sketch::empty,
        operations::sketch::triangle,
    ]
}
