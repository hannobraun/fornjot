//! # Fornjot CAD Kernel
//!
//! This library is part of the [Fornjot] ecosystem. Fornjot is an open-source,
//! code-first CAD application; and collection of libraries that make up the CAD
//! application, but can be used independently.
//!
//! This library is an internal component of Fornjot. It is not relevant to end
//! users that just want to create CAD models.
//!
//! The CAD kernel is the core of Fornjot: the geometry, the topology, and the
//! algorithms that handle them. It is separate from the CAD application, and
//! could be used in other applications.
//!
//!
//! ## Design Principle
//!
//! The CAD kernel follows the design principle of **robustness through
//! explicitness**. This means that geometrical relationships must be expressed
//! explicitly, or they are not accepted.
//!
//! It is important to understand that this principle defines the destination of
//! CAD kernel development. It does not reflect the current reality.
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

#![warn(missing_docs)]

pub mod algorithms;
pub mod builder;
pub mod iter;
pub mod objects;
