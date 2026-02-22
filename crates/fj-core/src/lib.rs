//! # Fornjot Core
//!
//! [Fornjot] is an early-stage b-rep CAD kernel written in Rust. The kernel is
//! split into multiple libraries that can be used semi-independently, and this
//! is one of those.
//!
//! This library defines geometric and topological primitives, and the
//! algorithms that operate on them.
//!
//!
//! ## Design Principle
//!
//! The CAD kernel follows the design principle of **robustness through
//! explicitness**. This means that geometrical relationships must be expressed
//! explicitly, or they are not accepted.
//!
//! This principle is not fully implemented yet. There are quite a few
//! validation checks that enforce part of it, but many are still missing.
//!
//! ### Motivation
//!
//! A problem that CAD kernels need to handle is the inherent fuzziness of
//! geometric relationships. Is a point on a curve, or just close to it? Does a
//! curve lie in a surface, or does it not? This is exacerbated by the limited
//! precision of numerical representations in computers, and especially the
//! inconvenient precision characteristics of floating-point numbers.
//!
//! These problems can be addressed by always comparing numbers using an epsilon
//! value. Numbers that are very close together (their difference is smaller
//! than epsilon) are considered equal.
//!
//! This approach has several problems:
//! - If the epsilon value is chosen too high, then very small models can become
//!   buggy, as distinct geometry is merged together.
//! - If the epsilon value is chosen too low, then very large models can become
//!   buggy, as geometry that is supposed to be identical is recognized as
//!   distinct.
//! - These epsilon comparisons need to be used everywhere where numbers are
//!   handled. It can be easy to forget this. Using custom wrapper types is
//!   possible, but either inflexible (because the epsilon value is hardcoded)
//!   or inconvenient (because the epsilon value needs to be provided).
//!
//! Choosing an epsilon value that is suitable for *most* use cases is possible,
//! at the cost of non-standard use cases breaking in unexpected and non-obvious
//! ways. Fornjot has chosen a different approach.
//!
//! ### Explicitness
//!
//! By requiring geometric relationships to be *explicit*, we don't have to use
//! error-prone heuristics to determine those relationships. That means, for
//! example, two vertices that happen to be identical, or very close to each
//! other, are not accepted.
//!
//! If vertex instances that refer to the same point are used in different
//! places (for example, in two neighboring edges that share a vertex), then
//! those vertex instances must be known by the system to refer to the same
//! vertex. If a vertex lies on an edge or in a surface, then it must be defined
//! in terms of its position on that edge or surface.
//!
//! This can have consequences for how users define models. For example, if the
//! user moves two shapes close to each other, so they touch but don't
//! intersect, this should lead to an error message, explaining to the user why
//! what they did is a problem, and teaching them how to define their model in
//! a different way, so the system knows the semantic relationships between
//! geometrical objects.
//!
//! ### Validation
//!
//! These rules of explicitness must be validated, so the user can know if there
//! is a problem in the model, and fix it. This is preferable to failing in
//! unexpected ways later on.
//!
//! For the comparisons required for validation, an epsilon value must be used.
//! This epsilon value can be derived from the size of the model, and should be
//! chosen as high as possible, so any potential problems can be immediately
//! reported as errors.
//!
//! If the user does something non-standard, they can override the epsilon value
//! on a per-shape basis. Forcing the user to deal with these issues up-front
//! should lead to less work overall.
//!
//! [Fornjot]: https://www.fornjot.app/

// This Clippy lint warns about keys in maps and sets whose types have interior
// mutability. This applies to `Handle`, which is why this lint triggers in
// multiple places in this crate.
//
// These are false positives, however. As per the documentation of the lint[1],
// These types are specifically okay to use, if their `Hash` and `Ord`
// implementations do not rely on any fields that are not mutable. And for
// `Handle`, those implementations only rely on the `Handle`s ID, which is
// constant.
//
// I don't think I have ever personally encountered a bug that this lint would
// have warned against. So I think it makes sense to, as a matter of
// convenience, just disable the lint for the whole crate, instead of doing so
// for every single false positive.
//
// It might be an even better option to configure Clippy to ignore `Handle`
// specifically for this lint, which should be possible according to the lint
// documentation.[1] For some reason this didn't work when I tried it, and I
// couldn't figure out why. In the end, I decided it wasn't worth spending more
// time on this, so this is the situation I ended up with.
//
// [1] https://rust-lang.github.io/rust-clippy/master/index.html#/mutable_key_type
#![allow(clippy::mutable_key_type)]

pub mod algorithms;
pub mod geometry;
pub mod interop;
pub mod layers;
pub mod math;
pub mod new;
pub mod operations;
pub mod presentation;
pub mod queries;
pub mod storage;
pub mod topology;
pub mod validate;
pub mod validation;

mod core;

pub use self::core::Core;
