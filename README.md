# Fornjot

## About

Experimental CAD system, written in the Rust programming language.

![Screenshot of the spacer model](https://github.com/hannobraun/fornjot/blob/main/models/spacer/spacer.png)

The following models are currently available:

- [Cube](/models/cube)
- [Spacer](/models/spacer)


## Status

Fornjot is still **highly experimental**. Development is currently focused on incrementally expanding features, to support specific new CAD models. This grounds the development effort in practical use cases and ensures constant forward motion. Which is good, because CAD is a very difficult topic, and there's always the danger of getting lost in the weeds, trying to implement some complicated algorithm from a barely legible paper.

However, that also means that other models, even if they're very similar to existing ones, are often not supported, because the algorithms used make simplifying assumptions. If you're interested in Fornjot and are considering to use it, you should fully expect to run into limitation pretty much immediately. Unless you are willing to contribute to its development, it would be better to wait for a year or ten, to let it mature.


## Features

### Code-CAD in Rust

Models are defined as Rust code. They are compiled separately, and loaded into a host application as a plug-in, to ensure fast compile times.

``` rust
use std::collections::HashMap;

#[no_mangle]
pub extern "C" fn model(args: &HashMap<String, String>) -> fj::Shape {
    let outer = args
        .get("outer")
        .unwrap_or(&"1.0".to_owned())
        .parse()
        .unwrap();
    let inner = args
        .get("inner")
        .unwrap_or(&"0.5".to_owned())
        .parse()
        .unwrap();
    let height = args
        .get("height")
        .unwrap_or(&"1.0".to_owned())
        .parse()
        .unwrap();

    let outer_edge = fj::Circle { radius: outer };
    let inner_edge = fj::Circle { radius: inner };

    let footprint = fj::Difference {
        a: outer_edge.into(),
        b: inner_edge.into(),
    };

    let spacer = fj::Sweep {
        shape: footprint.into(),
        length: height,
    };

    spacer.into()
}
```

This is the code for the [spacer model](/models/spacer). As you can see, there's still some work to do, to make the process of defining models more convenient.

### Export to 3MF

Exporting models to the [3D Manufacturing Format](https://en.wikipedia.org/wiki/3D_Manufacturing_Format) (3MF), which is used in 3D printing, is supported.


## Usage

### Defining models

Models depend on the [`fj`](/fj) library, which they use to define the geometry. Furthermore, they need to be built as a dynamic library. Just use the examples in the [`models/`](/models) directory as a template.

### Viewing models

To compile and view a model, run it from the host application.

``` sh
# Compile/view the spacer model
cargo run -- -m spacer
```

This invocation expects that the model exists in the `models/spacer` directory, with a package name of `spacer`.

Rotate the model by pressing the left mouse button while moving the mouse. Move the model by pressing the right mouse button while moving the mouse. Zoom with the mouse wheel.

Toggle model rendering by pressing `1`. Toggle mesh rendering by pressing `2`.

So far, the host application is not published on [crates.io](https://crates.io/), and the whole process is not really optimized for being used outside of this repository. Contributions to improve that situations are very welcome.

### Exporting models

To export a model to a 3MF file, run:

``` sh
cargo run -- -m spacer --export spacer.3mf
```

### Model parameters

Some models have parameters that can be overridden. For example, to override the inner and outer radii of the spacer model:

``` sh
cargo run -- -m spacer --arguments outer=8.0 inner=5.0
```


## License

This project is open source, licensed under the terms of the [Zero Clause BSD License] (0BSD, for short). This basically means you can do anything with it, without any restrictions, but you can't hold the authors liable for problems.

See [LICENSE.md] for full details.

[`fj`]: https://crates.io/crates/fj
[Zero Clause BSD License]: https://opensource.org/licenses/0BSD
[LICENSE.md]: https://github.com/hannobraun/fornjot/blob/main/LICENSE.md
