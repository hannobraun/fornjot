# Fornjot Crates

## About

Fornjot is both an application and an ecosystem of components that make up this application, but can be used independently. All if those components are located in this directory.

If you're just looking to use Fornjot as a CAD program, don't worry about this directory. Check out [the website](https://www.fornjot.app/) or the [top-level `README.md`](../README.md).


## Overview

This is a list of the crates in this directory, along with a short description for each, starting with the most basic ones and leading up to the high-level ones most relevant to end users:

- [`fj-math`]: Math primitives used by the rest of the Fornjot ecosystem.
- [`fj-interop`]: Basic types that allow other crates to interoperate, without depending on each other.
- [`fj-kernel`]: CAD kernel of Fornjot. Defines geometric and topological primitives, and algorithms that operate on those primitives.
- [`fj-operations`]: CAD operations, built on top of `fj-kernel`. Link between the kernel, and the API that users use to define models.
- [`fj-export`]: Exports Fornjot models to external data formats.
- [`fj-host`]: Loads Fornjot models and watches them for changes.
- [`fj-viewer`]: Displays Fornjot models.
- [`fj-window`]: Embeds `fj-viewer` in a Winit-based window.
- [`fj-app`]: The Fornjot CAD application.
- [`fj-proc`]: Procedural macros to improve the usability of the `fj` crate.
- [`fj`]: End-user API for defining Fornjot models.

[`fj`]: https://crates.io/crates/fj
[`fj-app`]: https://crates.io/crates/fj-app
[`fj-export`]: https://crates.io/crates/fj-export
[`fj-host`]: https://crates.io/crates/fj-host
[`fj-interop`]: https://crates.io/crates/fj-interop
[`fj-kernel`]: https://crates.io/crates/fj-kernel
[`fj-math`]: https://crates.io/crates/fj-math
[`fj-operations`]: https://crates.io/crates/fj-operations
[`fj-proc`]: https://crates.io/crates/fj-proc
[`fj-viewer`]: https://crates.io/crates/fj-viewer
[`fj-window`]: https://crates.io/crates/fj-window
