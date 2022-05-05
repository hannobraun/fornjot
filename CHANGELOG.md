# Fornjot - Changelog

## v0.6.0 (2022-05-05)

The following changelog is a summary of user-visible changes, meaning changes visible to end users (who define CAD models using `fj` and `fj-app`), or changes visible to users of the API.

For a full summary of all pull requests, feel free to check out all Weekly Dev Logs that cover the time period since the previous release:

- [2022-W04](https://www.fornjot.app/blog/weekly-dev-log/2022-w04/)
- [2022-W05](https://www.fornjot.app/blog/weekly-dev-log/2022-w05/)
- [2022-W06](https://www.fornjot.app/blog/weekly-dev-log/2022-w06/)
- [2022-W07](https://www.fornjot.app/blog/weekly-dev-log/2022-w07/)
- [2022-W08](https://www.fornjot.app/blog/weekly-dev-log/2022-w08/)
- [2022-W09](https://www.fornjot.app/blog/weekly-dev-log/2022-w09/)
- [2022-W10](https://www.fornjot.app/blog/weekly-dev-log/2022-w10/)
- [2022-W11](https://www.fornjot.app/blog/weekly-dev-log/2022-w11/)
- [2022-W12](https://www.fornjot.app/blog/weekly-dev-log/2022-w12/)
- [2022-W13](https://www.fornjot.app/blog/weekly-dev-log/2022-w13/)
- [2022-W14](https://www.fornjot.app/blog/weekly-dev-log/2022-w14/)
- [2022-W15](https://www.fornjot.app/blog/weekly-dev-log/2022-w15/)
- [2022-W16/W17](https://www.fornjot.app/blog/weekly-dev-log/2022-w16-17/)

### [`fj`](https://crates.io/crates/fj)

The API that Fornjot models are written against.

- Improve documentation ([#106], [#411])
- Remove `fj::Difference` ([#265])
- Add support for coloring models ([#343])
- Rename `fj::Union` to `fj::Group` ([#366])
- Add convenient syntax for `fj::Difference2d` ([#372])
- Clean up API ([#412])
- Support sweeping in arbitrary directions ([#505])

### [`fj-app`](https://crates.io/crates/fj-app)

The main Fornjot application.

- Fix model loading error, if name contains '-' ([#107])
- Fix circle approximation being able to freeze application ([#111])
- Prevent potential floating-point accuracy issues in triangulation ([#133])
- Add missing space to error message ([#144])
- Enable console output ([#148], [#297])
- Fix various triangulation bugs ([#158], [#448], [#453])
- Display size of model bounding box ([#217])
- Ensure that vertices are unique ([#278])
- Fix sweeping of non-symmetrical sketches ([#284])
- Fix bugs that affect shading faces and exporting 3MF files ([#289], [#484])
- Fix crash on some graphics hardware ([#323])
- Fix warning about glyph cache size ([#337])
- Add support for specifying tolerance as command-line argument ([#352], [#359])
- Rename application to `fj-app` ([#356])
- Add configuration file ([#362])
- Enable `fj-app` to run outside of Fornjot repository ([#364])
- Fix tolerance value not being updated on model reload ([#379])
- Fix race condition when loading model initially ([#380])
- Fix warning about buffer having a pending mapping ([#397])
- Fix crash with AMD GPUs ([#437])
- Make rotation work, even when not clicking on model ([#503])

### [`fj-export`](https://crates.io/crates/fj-export)

Library for exporting Fornjot models to external file formats.

Initial release.

### [`fj-host`](https://crates.io/crates/fj-host)

Library for hosting Fornjot models.

Initial release.

### [`fj-interop`](https://crates.io/crates/fj-interop)

Library that defines types to allow interoperation between other Fornjot components.

Initial release.

### [`fj-kernel`](https://crates.io/crates/fj-kernel)

Fornjot's CAD kernel library.

Initial release.

### [`fj-math`](https://crates.io/crates/fj-math)

Library that provides math primitives for the Fornjot ecosystem.

Initial release.

### [`fj-operations`](https://crates.io/crates/fj-operations)

Library that defines CAD operations, as a link between `fj` and `fj-kernel`.

Initial release.

### [`fj-viewer`](https://crates.io/crates/fj-viewer)

Library that provides a model viewer.

Initial release.

[#106]: https://github.com/hannobraun/Fornjot/pull/106
[#107]: https://github.com/hannobraun/Fornjot/pull/107
[#111]: https://github.com/hannobraun/Fornjot/pull/111
[#133]: https://github.com/hannobraun/Fornjot/pull/133
[#144]: https://github.com/hannobraun/Fornjot/pull/144
[#148]: https://github.com/hannobraun/Fornjot/pull/148
[#158]: https://github.com/hannobraun/Fornjot/pull/158
[#217]: https://github.com/hannobraun/Fornjot/pull/217
[#265]: https://github.com/hannobraun/Fornjot/pull/265
[#278]: https://github.com/hannobraun/Fornjot/pull/278
[#284]: https://github.com/hannobraun/Fornjot/pull/284
[#289]: https://github.com/hannobraun/Fornjot/pull/289
[#297]: https://github.com/hannobraun/Fornjot/pull/297
[#323]: https://github.com/hannobraun/Fornjot/pull/323
[#337]: https://github.com/hannobraun/Fornjot/pull/337
[#343]: https://github.com/hannobraun/Fornjot/pull/343
[#352]: https://github.com/hannobraun/Fornjot/pull/352
[#356]: https://github.com/hannobraun/Fornjot/pull/356
[#359]: https://github.com/hannobraun/Fornjot/pull/359
[#362]: https://github.com/hannobraun/Fornjot/pull/362
[#364]: https://github.com/hannobraun/Fornjot/pull/364
[#366]: https://github.com/hannobraun/Fornjot/pull/366
[#372]: https://github.com/hannobraun/Fornjot/pull/372
[#379]: https://github.com/hannobraun/Fornjot/pull/379
[#380]: https://github.com/hannobraun/Fornjot/pull/380
[#397]: https://github.com/hannobraun/Fornjot/pull/397
[#411]: https://github.com/hannobraun/Fornjot/pull/411
[#412]: https://github.com/hannobraun/Fornjot/pull/412
[#437]: https://github.com/hannobraun/Fornjot/pull/437
[#448]: https://github.com/hannobraun/Fornjot/pull/448
[#453]: https://github.com/hannobraun/Fornjot/pull/453
[#484]: https://github.com/hannobraun/Fornjot/pull/484
[#503]: https://github.com/hannobraun/Fornjot/pull/503
[#505]: https://github.com/hannobraun/Fornjot/pull/505


## v0.5.0 (2022-01-26)

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
