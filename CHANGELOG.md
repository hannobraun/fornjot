# Fornjot - Changelog

## 0.3.1 (2021-11-22)

- Reload current model, whenever its source code is modified.

## v0.3.0 (2021-11-21)

- Function representation (F-rep) has been phased out in favor of a more traditional approach inspired by boundary (B-rep). This has resulted in much higher-quality triangulation of the geometry (spacer previously took around 1 second, now there is no perceivable delay).
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
