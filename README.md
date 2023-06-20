# Fornjot

[**Blog**](https://www.fornjot.app/blog/) | [**Community**](https://www.fornjot.app/community/) | [**Contribution Guide**](CONTRIBUTING.md)

## About

Fornjot is an early-stage [b-rep](https://en.wikipedia.org/wiki/Boundary_representation) CAD kernel written in the Rust programming language.

For an introduction of what the project aims to achieve, [please check out the website](https://www.fornjot.app/).


## Sponsors

Fornjot is supported by [**@MitchellHansen**](https://github.com/MitchellHansen), [**@webtrax-oz**](https://github.com/webtrax-oz), [**@reivilibre**](https://github.com/reivilibre), [**@lthiery**](https://github.com/lthiery), [**@ahdinosaur**](https://github.com/ahdinosaur), [**@martindederer**](https://github.com/martindederer), [**@bollian**](https://github.com/bollian), [**@nullstyle**](https://github.com/nullstyle), [**@tachiniererin**](https://github.com/tachiniererin), [**@HalfVoxel**](https://github.com/HalfVoxel), [**@sucaba**](https://github.com/sucaba), [**@Kethku**](https://github.com/Kethku), [**@jessebraham**](https://github.com/jessebraham), [**@seanjensengrey**](https://github.com/seanjensengrey), [**@MattOslin**](https://github.com/MattOslin), [**@benwis**](https://github.com/benwis), [**@happysalada**](https://github.com/happysalada), [**@jminer**](https://github.com/jminer), [**@jeevcat**](https://github.com/jeevcat), [**@voxpelli**](https://github.com/voxpelli), [**@U007D**](https://github.com/U007D), [**@guillaumechauvat**](https://github.com/guillaumechauvat), [**@mayfieldiv**](https://github.com/mayfieldiv), and [my other awesome sponsors](https://github.com/sponsors/hannobraun). Thank you!

**Please consider [supporting me too](https://github.com/sponsors/hannobraun), to help make Fornjot sustainable long-term.**


## Table of Contents

- [**Status**](#status)
- [**Overview**](#overview)
- [**Usage**](#usage)
- [**Community**](#community)
- [**Get Involved**](#get-involved)
- [**License**](#license)


## Status

Fornjot is usable for some toy examples, but currently lacks the features for something more advanced. Work to change that is underway.


## Overview

This is a list of the crates that make up Fornjot, along with a short description for each, starting with the most basic ones and leading up to the high-level ones:

- [`fj`]: All-in-one API that re-exports all of the following crates.
- [`fj-math`]: Math primitives used by the rest of Fornjot.
- [`fj-interop`]: Basic types that allow other crates to interoperate, without depending on each other.
- [`fj-core`]: Core primitives and code operating on those primitives.
- [`fj-export`]: Exports Fornjot models to external data formats.
- [`fj-viewer`]: Displays Fornjot models.
- [`fj-window`]: Simple windowing abstraction for use with `fj-viewer`.

[`fj`]: https://crates.io/crates/fj
[`fj-core`]: https://crates.io/crates/fj-core
[`fj-export`]: https://crates.io/crates/fj-export
[`fj-interop`]: https://crates.io/crates/fj-interop
[`fj-math`]: https://crates.io/crates/fj-math
[`fj-viewer`]: https://crates.io/crates/fj-viewer
[`fj-window`]: https://crates.io/crates/fj-window


## Usage

Fornjot is a set of Rust libraries (see list above). The definitive documentation on how to use those is their reference documentation. The `crates.io` pages of each library (see list above) link to those.

If you want to use Fornjot to create a specific model in Rust, the best starting point are the [example models](models/) in this repository:

- To display a model, run `cargo run -p cuboid` (replace `cuboid` with name of model you want to display).
- To export a model, run `cargo run -p cuboid -- --export model.3mf` (replace `cuboid` with name of model you want to export; optionally replace `3mf` with another supported file format).
- To see full set of CLI options, run `cargo run -p cuboid -- --help` (all models have the same CLI interface, so this shouldn't differ much between them).


## Community

If you are interested in Fornjot, please consider joining the community. We'd love to have you!

Please check out [the community page on the website](https://www.fornjot.app/community/) for information on where to find us!


## Get Involved

If you are interested in helping out, just fork one of the GitHub repositories and submit a pull request:

- [Main Fornjot repository](https://github.com/hannobraun/Fornjot)
- [Website repository](https://github.com/hannobraun/www.fornjot.app)

If you don't know what to work on, check out the [`good first issues`](https://github.com/hannobraun/Fornjot/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22). If you need some more guidance, check out the [contribution guide](CONTRIBUTING.md), [or just ask](https://www.fornjot.app/community/)!


## License

This project is open source, licensed under the terms of the [Zero Clause BSD License] (0BSD, for short). This basically means you can do anything with it, without any restrictions, but you can't hold the authors liable for problems.

See [LICENSE.md] for full details.

[`fj`]: https://crates.io/crates/fj
[Zero Clause BSD License]: https://opensource.org/licenses/0BSD
[LICENSE.md]: LICENSE.md
