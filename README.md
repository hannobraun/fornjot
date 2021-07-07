# Fornjot

## About

Experimental CAD system. The CAD system is a library, written in the Rust programming language.

The following models are currently available:

- [Cube](https://github.com/hannobraun/fornjot/blob/main/models/cube)
- [Spacer](https://github.com/hannobraun/fornjot/blob/main/models/spacer)

## Status

Fornjot is still at an experimental stage, where I figure out which approaches work and which don't. It is capable enough to create very simple models and export those to [3MF] for 3D-printing.

It is very likely that you will run into limits when trying to use Fornjot for anything but the simplest models. If you are missing any functionality, pull requests are very welcome.

## Usage

Fornjot is a regular Rust library, and CAD models are just Rust applications that use the library. To use Fornjot for your own models, create a Rust application using Cargo, include Fornjot as a dependency (see [`fj`] on crates.io), and use the models from this repository (see above) as inspiration.

To display one of the models from this repository, run:

``` bash
cargo run --bin spacer
```

Replace `spacer` with the name of the model you want to see.

To export one of the models from this repository to 3MF, run:

``` bash
cargo run --bin spacer -- --export model.3mf
```

Replace `spacer with the name of the model you want to export.

Some models have parameters that can be overridden. For example, you can override the inner radius of the spacer model like this:

``` bash
cargo run --bin spacer -- --model-params="{\"inner\": 5.0}"
```

`model-params` is a JSON object. Check out `spacer`'s source code, to see what else can be overridden.

## License

This project is open source software, licensed under the terms of the [Zero Clause BSD License] (0BSD, for short). This basically means you can do anything with the software, without any restrictions, but you can't hold the authors liable for problems.

See [LICENSE.md] for full details.

[3MF]: https://en.wikipedia.org/wiki/3D_Manufacturing_Format
[Zero Clause BSD License]: https://opensource.org/licenses/0BSD
[LICENSE.md]: https://github.com/hannobraun/fornjot/blob/main/LICENSE.md
