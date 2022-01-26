# Fornjot - Changelog

## Unreleased

### `fj` Library

- Replace `fj::Rectangle` with the more powerful `fj::Sketch`.
- Add `fj::Union` to express unions. This is subject to limitations (see API Reference).
- Add `fj::Transform` to support transforming shapes.
- Add traits to provide simplified syntax for various operations. These traits can be accessed through a `use fj::prelude::*;`.
- Rename `fj::Difference` to `fj::Difference2d` to make room for a 3D difference operation.
- Add `fj::Difference` to express difference operation in 3D. This is not supported by the host application yet.
- Improve documentation ([#86])


### Host Application

- Fix shapes that are very near or very far not being shown on camera.
- Add support for Windows and macOS ([#22], [#23], [#28]; special thanks to Fornjot's first contributor, [@Bandsberg](https://github.com/Bandsberg)!).
- Add support for concave 2D sketches.
- Add debug info visualization mechanism to help debug internal algorithms. So far, it just outputs lines to visualize the triangulation algorithm.
- Fix bug in 2D difference operation, that would create an internal pseudo-face within the model, if the 2D difference was swept into a 3D model.
- Add blacklist to avoid multiple rebuilds on model changes ([#39]; special thanks to first-time contributor, [@mxdamien](https://github.com/mxdamien))
- Fix triangulation bugs that would cause errors in some models ([#61], [#74], [#81])


- Add star model to repository ([#50])
- Lots of internal clean-ups, to enable more features in the future.

[#22]: https://github.com/hannobraun/fornjot/pull/22
[#23]: https://github.com/hannobraun/fornjot/pull/23
[#28]: https://github.com/hannobraun/fornjot/pull/28
[#39]: https://github.com/hannobraun/fornjot/pull/39
[#50]: https://github.com/hannobraun/fornjot/pull/50
[#61]: https://github.com/hannobraun/fornjot/pull/61
[#74]: https://github.com/hannobraun/fornjot/pull/74
[#81]: https://github.com/hannobraun/fornjot/pull/81
[#86]: https://github.com/hannobraun/fornjot/pull/86


## v0.4.0 (2021-12-07)

### Host Application

- Tweak zooming behavior:
  Zoom speed is dependent on the frequency of input signals (either the movement of the mouse wheel, or of the fingers on the track pad). Speed zooming in is limited depending on the distance to the model.
- Improve rotation behavior:
  Always rotate the model around the point on the model that the mouse cursor points at, not the origin of the model coordinate system. This allows for much more precise control when inspecting details of the model.
- Improve movement behavior:
  When moving the model, keep the same point on the model under the cursor for the whole movement. This doesn't work great yet (see [#18](https://github.com/hannobraun/fornjot/issues/18)).
- Rename `--arguments` argument of host application to `--parameters`.

### `fj` Library

- Replace `fj::Square` with `fj::Rectangle`.


## v0.3.1 (2021-11-22)

- Reload current model, whenever its source code is modified.


## v0.3.0 (2021-11-21)

- Function representation (F-rep) has been phased out in favor of a more traditional approach inspired by boundary representation (B-rep). This has resulted in much higher-quality triangulation of the geometry in significantly less time (spacer previously took around 1 second, now there is no perceivable delay).
- Most of the system is no longer a library; it now consists of a host application, and a very light library used to define geometry. Models are compiled as dynamic libraries and loaded at runtime. This has resulted in much shorter compile times when changing a model (previously many seconds, now way below 0.5s for the spacer model).
- Due to a rewrite of all CAD-specific code, the way models are defined is completely different.


## v0.2.0 (2021-07-07)

- Add support for exporting models to the 3MF format. This makes it possible to 3D-print Fornjot models.
- Also return surface normal (in addition to distance from surface) when sampling geometry.
- Greatly improve accuracy of the triangle mesh that is generated from models. More room for improvement remains, as sharp edges aren't reproduced faithfully.


## v0.1.1 (2021-05-19)

- Link `README.md` in `Cargo.toml`


## v0.1.0 (2021-05-19)

Initial release.
