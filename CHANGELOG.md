# Fornjot - Changelog

## v0.14.0 (2022-09-05)

### End-user improvements

Improvements to Fornjot and its documentation that are visible to end-users.

- Fix crash on some graphics hardware ([#1035])

### Ecosystem improvements

Improvements to the Fornjot ecosystem that are relevant to developers who are building on top of Fornjot components.

#### `fj-kernel`

- Clean up approximation code ([#1011], [#1012], [#1013], [#1028])
- Clean up and expand `algorithms::reverse` ([#1017])
- Fix edge cases in object equality comparisons ([#1018], [#1022])
- Improve and expand object validation ([#1023], [#1024], [#1030], [#1031])
- Make small improvements in kernel ([#1025])
- Clean up sweep algorithm ([#1026], [#1033])

#### `fj-math`

- Add some validation code to `PolyChain` ([#1027])
- Derive `Default` for all math types ([#1029])
- Add `Line::from_points_with_line_coords` ([#1032])

### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update release procedure ([#1008], [#1009])
- Update dependencies ([#1010])
- Expand release automation ([#1016])

[#1008]: https://github.com/hannobraun/Fornjot/pull/1008
[#1009]: https://github.com/hannobraun/Fornjot/pull/1009
[#1010]: https://github.com/hannobraun/Fornjot/pull/1010
[#1011]: https://github.com/hannobraun/Fornjot/pull/1011
[#1012]: https://github.com/hannobraun/Fornjot/pull/1012
[#1013]: https://github.com/hannobraun/Fornjot/pull/1013
[#1016]: https://github.com/hannobraun/Fornjot/pull/1016
[#1017]: https://github.com/hannobraun/Fornjot/pull/1017
[#1018]: https://github.com/hannobraun/Fornjot/pull/1018
[#1022]: https://github.com/hannobraun/Fornjot/pull/1022
[#1023]: https://github.com/hannobraun/Fornjot/pull/1023
[#1024]: https://github.com/hannobraun/Fornjot/pull/1024
[#1025]: https://github.com/hannobraun/Fornjot/pull/1025
[#1026]: https://github.com/hannobraun/Fornjot/pull/1026
[#1027]: https://github.com/hannobraun/Fornjot/pull/1027
[#1028]: https://github.com/hannobraun/Fornjot/pull/1028
[#1029]: https://github.com/hannobraun/Fornjot/pull/1029
[#1030]: https://github.com/hannobraun/Fornjot/pull/1030
[#1031]: https://github.com/hannobraun/Fornjot/pull/1031
[#1032]: https://github.com/hannobraun/Fornjot/pull/1032
[#1033]: https://github.com/hannobraun/Fornjot/pull/1033
[#1035]: https://github.com/hannobraun/Fornjot/pull/1035


## v0.13.0 (2022-08-29)

### End-user improvements

Improvements to Fornjot and its documentation that are visible to end-users.

- Update usage documentation in README ([#994])

### Ecosystem improvements

Improvements to the Fornjot ecosystem that are relevant to developers who are building on top of Fornjot components.

#### `fj-host`

- Improve comments and documentation ([#988])

#### `fj-kernel`

- Expand intersection tests ([#977], [#978])
- Extract `Shell` from `Solid` ([#983])
- Clean up sweep API ([#984], [#989], [#991])
- Add builder API for `Sketch` ([#992])
- Add `GlobalEdge` ([#998], [#999])
- Make some minor cleanups ([#1000], [#1001], [#1005])
- Clean up `approx` module ([#1003], [#1006])

#### `fj-math`

- Make minor API additions ([#1004])

#### `fj-viewer`/`fj-window`

- Upgrade dependencies related to wgpu/winit ([#975], [#979])

### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#964], [#965], [#966], [#968], [#969], [#970], [#974])
- Update release procedure ([#972])
- Upgrade to Rust 1.63.0 ([#973])
- Expand release automation ([#981], [#982])
- Add usage documentation to `CONTRIBUTING.md` ([#995])

[#961]: https://github.com/hannobraun/Fornjot/pull/961
[#962]: https://github.com/hannobraun/Fornjot/pull/962
[#964]: https://github.com/hannobraun/Fornjot/pull/964
[#965]: https://github.com/hannobraun/Fornjot/pull/965
[#966]: https://github.com/hannobraun/Fornjot/pull/966
[#968]: https://github.com/hannobraun/Fornjot/pull/968
[#969]: https://github.com/hannobraun/Fornjot/pull/969
[#970]: https://github.com/hannobraun/Fornjot/pull/970
[#972]: https://github.com/hannobraun/Fornjot/pull/972
[#973]: https://github.com/hannobraun/Fornjot/pull/973
[#974]: https://github.com/hannobraun/Fornjot/pull/974
[#975]: https://github.com/hannobraun/Fornjot/pull/975
[#977]: https://github.com/hannobraun/Fornjot/pull/977
[#978]: https://github.com/hannobraun/Fornjot/pull/978
[#979]: https://github.com/hannobraun/Fornjot/pull/979
[#981]: https://github.com/hannobraun/Fornjot/pull/981
[#982]: https://github.com/hannobraun/Fornjot/pull/982
[#983]: https://github.com/hannobraun/Fornjot/pull/983
[#984]: https://github.com/hannobraun/Fornjot/pull/984
[#988]: https://github.com/hannobraun/Fornjot/pull/988
[#989]: https://github.com/hannobraun/Fornjot/pull/989
[#991]: https://github.com/hannobraun/Fornjot/pull/991
[#992]: https://github.com/hannobraun/Fornjot/pull/992
[#994]: https://github.com/hannobraun/Fornjot/pull/994
[#995]: https://github.com/hannobraun/Fornjot/pull/995
[#998]: https://github.com/hannobraun/Fornjot/pull/998
[#999]: https://github.com/hannobraun/Fornjot/pull/999
[#1000]: https://github.com/hannobraun/Fornjot/pull/1000
[#1001]: https://github.com/hannobraun/Fornjot/pull/1001
[#1003]: https://github.com/hannobraun/Fornjot/pull/1003
[#1004]: https://github.com/hannobraun/Fornjot/pull/1004
[#1005]: https://github.com/hannobraun/Fornjot/pull/1005
[#1006]: https://github.com/hannobraun/Fornjot/pull/1006


## v0.12.0 (2022-08-22)

### End-user improvements

Improvements to Fornjot and its documentation that are visible to end-users.

- Display the last few status updates ([#919], [#945], [#952]; thank you, [@devanlooches]!)
- Add table of contents to README ([#942])
- Display model compile times in status updates ([#960]; thank you, [@connor-lennox]!)

### Ecosystem improvements

Improvements to the Fornjot ecosystem that are relevant to developers who are building on top of Fornjot components.

#### `fj-kernel`

- Add `Surface` to `Cycle` ([#939])
- Clean up and expand intersection testing code ([#940], [#941], [#946], [#947], [#948], [#949], [#950], [#951])

### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Switch to model-driven host API ([#885], [#934]; thank you, [@Michael-F-Bryan]!)
- Update list of sponsors in README ([#921], [#961])
- Update dependencies ([#922], [#923], [#924], [#925], [#926], [#928], [#929], [#930], [#931], [#933], [#953], [#954], [#956], [#957], [#958], [#959])
- Update release procedure ([#932])

[#885]: https://github.com/hannobraun/Fornjot/pull/885
[#919]: https://github.com/hannobraun/Fornjot/pull/919
[#921]: https://github.com/hannobraun/Fornjot/pull/921
[#922]: https://github.com/hannobraun/Fornjot/pull/922
[#923]: https://github.com/hannobraun/Fornjot/pull/923
[#924]: https://github.com/hannobraun/Fornjot/pull/924
[#925]: https://github.com/hannobraun/Fornjot/pull/925
[#926]: https://github.com/hannobraun/Fornjot/pull/926
[#928]: https://github.com/hannobraun/Fornjot/pull/928
[#929]: https://github.com/hannobraun/Fornjot/pull/929
[#930]: https://github.com/hannobraun/Fornjot/pull/930
[#931]: https://github.com/hannobraun/Fornjot/pull/931
[#932]: https://github.com/hannobraun/Fornjot/pull/932
[#933]: https://github.com/hannobraun/Fornjot/pull/933
[#934]: https://github.com/hannobraun/Fornjot/pull/934
[#939]: https://github.com/hannobraun/Fornjot/pull/939
[#940]: https://github.com/hannobraun/Fornjot/pull/940
[#941]: https://github.com/hannobraun/Fornjot/pull/941
[#942]: https://github.com/hannobraun/Fornjot/pull/942
[#945]: https://github.com/hannobraun/Fornjot/pull/945
[#946]: https://github.com/hannobraun/Fornjot/pull/946
[#947]: https://github.com/hannobraun/Fornjot/pull/947
[#948]: https://github.com/hannobraun/Fornjot/pull/948
[#949]: https://github.com/hannobraun/Fornjot/pull/949
[#950]: https://github.com/hannobraun/Fornjot/pull/950
[#951]: https://github.com/hannobraun/Fornjot/pull/951
[#952]: https://github.com/hannobraun/Fornjot/pull/952
[#953]: https://github.com/hannobraun/Fornjot/pull/953
[#954]: https://github.com/hannobraun/Fornjot/pull/954
[#956]: https://github.com/hannobraun/Fornjot/pull/956
[#957]: https://github.com/hannobraun/Fornjot/pull/957
[#958]: https://github.com/hannobraun/Fornjot/pull/958
[#959]: https://github.com/hannobraun/Fornjot/pull/959
[#960]: https://github.com/hannobraun/Fornjot/pull/960
[#961]: https://github.com/hannobraun/Fornjot/pull/961

[@connor-lennox]: https://github.com/connor-lennox
[@devanlooches]: https://github.com/devanlooches
[@Michael-F-Bryan]: https://github.com/Michael-F-Bryan

## v0.11.0 (2022-08-08)

### End-user improvements

Improvements to Fornjot and its documentation that are visible to end-users.

- Prevent crashes due to unavailable features in graphics backend ([#902], [#909], [#914]; special thanks go to first-time contributor [@hekno25]!)
- Add UI element that display current model status ([#911]; special thanks go to first-time contributor [@devanlooches]!)

### Ecosystem improvements

Improvements to the Fornjot ecosystem that are relevant to developers who are building on top of Fornjot components.

#### `fj-kernel`

- Clean up handling of curves ([#900], [#901], [#904])
- Clean up intersection code ([#905], [#906])
- Implement face/face intersection ([#915])
- Make ray casting code public, clean it up ([#918])

#### `fj-math`

- Validate `Line` and `Circle` on construction ([#910], [#913])
- Extend and clean up `AbsDiffEq` implementations ([#912])

### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#892], [#894], [#895], [#896], [#899])
- Update release procedure ([#898])
- Speed up release automation ([#903])
- Update description of Cargo packages ([#916])
- Update list of sponsors in README ([#921])

[#892]: https://github.com/hannobraun/Fornjot/pull/892
[#894]: https://github.com/hannobraun/Fornjot/pull/894
[#895]: https://github.com/hannobraun/Fornjot/pull/895
[#896]: https://github.com/hannobraun/Fornjot/pull/896
[#898]: https://github.com/hannobraun/Fornjot/pull/898
[#899]: https://github.com/hannobraun/Fornjot/pull/899
[#900]: https://github.com/hannobraun/Fornjot/pull/900
[#901]: https://github.com/hannobraun/Fornjot/pull/901
[#902]: https://github.com/hannobraun/Fornjot/pull/902
[#903]: https://github.com/hannobraun/Fornjot/pull/903
[#904]: https://github.com/hannobraun/Fornjot/pull/904
[#905]: https://github.com/hannobraun/Fornjot/pull/905
[#906]: https://github.com/hannobraun/Fornjot/pull/906
[#909]: https://github.com/hannobraun/Fornjot/pull/909
[#910]: https://github.com/hannobraun/Fornjot/pull/910
[#911]: https://github.com/hannobraun/Fornjot/pull/911
[#912]: https://github.com/hannobraun/Fornjot/pull/912
[#913]: https://github.com/hannobraun/Fornjot/pull/913
[#914]: https://github.com/hannobraun/Fornjot/pull/914
[#915]: https://github.com/hannobraun/Fornjot/pull/915
[#916]: https://github.com/hannobraun/Fornjot/pull/916
[#918]: https://github.com/hannobraun/Fornjot/pull/918
[#921]: https://github.com/hannobraun/Fornjot/pull/921

[@devanlooches]: https://github.com/devanlooches
[@hekno25]: https://github.com/hekno25


## v0.10.0 (2022-08-01)

### End-user improvements

Improvements to Fornjot and its documentation that are visible to end-users.

- Add `--version` argument ([#868]; thank you, [@Michael-F-Bryan]!)
- Improve README ([#877], [#882])


### Ecosystem improvements

Improvements to the Fornjot ecosystem that are relevant to developers who are building on top of Fornjot components.

#### `fj-interop`

- Re-use `fj_math::Triangle` in `fj_interop::mesh` ([#886])

#### `fj-kernel`

- Improve wording in doc comment ([#880])
- Clean up API of object types ([#881], [#891])
- Implement curve/edge intersection ([#884], [#888], [#889])
- Clean up surface/surface intersection ([#890])

#### `fj-math`

- Make `Triangle::from_points` fallible; add `Line::is_coincident_with` ([#887])


### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#870], [#871], [#872], [#873], [#874], [#876])
- Update release procedure ([#875], [#879])

[#868]: https://github.com/hannobraun/Fornjot/pull/868
[#870]: https://github.com/hannobraun/Fornjot/pull/870
[#871]: https://github.com/hannobraun/Fornjot/pull/871
[#872]: https://github.com/hannobraun/Fornjot/pull/872
[#873]: https://github.com/hannobraun/Fornjot/pull/873
[#874]: https://github.com/hannobraun/Fornjot/pull/874
[#875]: https://github.com/hannobraun/Fornjot/pull/875
[#876]: https://github.com/hannobraun/Fornjot/pull/876
[#877]: https://github.com/hannobraun/Fornjot/pull/877
[#879]: https://github.com/hannobraun/Fornjot/pull/879
[#880]: https://github.com/hannobraun/Fornjot/pull/880
[#881]: https://github.com/hannobraun/Fornjot/pull/881
[#882]: https://github.com/hannobraun/Fornjot/pull/882
[#884]: https://github.com/hannobraun/Fornjot/pull/884
[#886]: https://github.com/hannobraun/Fornjot/pull/886
[#887]: https://github.com/hannobraun/Fornjot/pull/887
[#888]: https://github.com/hannobraun/Fornjot/pull/888
[#889]: https://github.com/hannobraun/Fornjot/pull/889
[#890]: https://github.com/hannobraun/Fornjot/pull/890
[#891]: https://github.com/hannobraun/Fornjot/pull/891

[@Michael-F-Bryan]: https://github.com/Michael-F-Bryan


## v0.9.0 (2022-07-25)

### End-user improvements

Improvements to Fornjot and its documentation that are visible to end-users.

- Determine model's `target/` directory from Cargo metadata ([#828], [#841], [#853]; special thanks go to first-time contributor [@Michael-F-Bryan]!)
- Derive `PartialEq` for types in `fj` crate ([#832]; thank you, [@Michael-F-Bryan]!)
- Type-check model functions ([#867]; thank you, [@Michael-F-Bryan]!)

### Ecosystem improvements

Improvements to the Fornjot ecosystem that are relevant to developers who are building on top of Fornjot components.

#### `fj-interop`

- Convert `Color` into a struct ([#862])

#### `fj-kernel`

- Clean up and expand APIs of `Edge`, `Face`, and `Cycle` ([#854], [#855], [#863], [#865])
- Return references to objects, where appropriate ([#858])
- Make names of `Local` methods more explicit ([#860])
- Revamp builder API ([#864], [#866])

### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Fix release automation issues ([#814], [#843]; thank you, [@hendrikmaus]!)
- Update dependencies ([#836], [#840])
- Update release procedure ([#838], [#839], [#857])
- Add unit test for triangulation bug ([#842]; special thanks go to first-time contributor [@willhansen]!)
- Upgrade to  Rust 1.62.1 ([#852])
- Clean up `fj-kernel`'s `iter` module ([#859])
- Expand implementation note ([#861])

[#814]: https://github.com/hannobraun/Fornjot/pull/814
[#828]: https://github.com/hannobraun/Fornjot/pull/828
[#832]: https://github.com/hannobraun/Fornjot/pull/832
[#836]: https://github.com/hannobraun/Fornjot/pull/836
[#838]: https://github.com/hannobraun/Fornjot/pull/838
[#839]: https://github.com/hannobraun/Fornjot/pull/839
[#840]: https://github.com/hannobraun/Fornjot/pull/840
[#841]: https://github.com/hannobraun/Fornjot/pull/841
[#842]: https://github.com/hannobraun/Fornjot/pull/842
[#843]: https://github.com/hannobraun/Fornjot/pull/843
[#852]: https://github.com/hannobraun/Fornjot/pull/852
[#853]: https://github.com/hannobraun/Fornjot/pull/853
[#854]: https://github.com/hannobraun/Fornjot/pull/854
[#855]: https://github.com/hannobraun/Fornjot/pull/855
[#857]: https://github.com/hannobraun/Fornjot/pull/857
[#858]: https://github.com/hannobraun/Fornjot/pull/858
[#859]: https://github.com/hannobraun/Fornjot/pull/859
[#860]: https://github.com/hannobraun/Fornjot/pull/860
[#861]: https://github.com/hannobraun/Fornjot/pull/861
[#862]: https://github.com/hannobraun/Fornjot/pull/862
[#863]: https://github.com/hannobraun/Fornjot/pull/863
[#864]: https://github.com/hannobraun/Fornjot/pull/864
[#865]: https://github.com/hannobraun/Fornjot/pull/865
[#866]: https://github.com/hannobraun/Fornjot/pull/866
[#867]: https://github.com/hannobraun/Fornjot/pull/867

[@hendrikmaus]: https://github.com/hendrikmaus
[@Michael-F-Bryan]: https://github.com/Michael-F-Bryan
[@willhansen]: https://github.com/willhansen


## v0.8.0 (2022-07-18)

### End-user improvements

Improvements to Fornjot and its documentation that are visible to end-users.

- Make moving the model work, even if mouse is not hovering over it ([#806])
- Make group and transform operations work on all shapes ([#825])

### Ecosystem improvements

Improvements to the Fornjot ecosystem that are relevant to developers who are building on top of Fornjot components.

#### `fj-interop`

- Add `ProcessedShape` from `fj-operations` ([#809]; thank you [@jeevcat]!)

#### `fj-kernel`

- Implement curve/face intersection algorithm ([#802], [#812], [#813], [#817], [#826])
- Return local curves from surface/surface intersection ([#811])
- Derive `Copy` for `VerticesOfEdge` ([#818])
- Add `Sketch`/`Solid` to distinguish between 2D/3D shapes ([#819], [#823], [#827])
- Provide more complete and convenient transform API ([#822])

#### `fj-math`

- Fix edge case in `Vector::scalar_projection_onto` ([#810])

#### `fj-operations`

- Rename `ToShape` to `Shape`; clean it up ([#820])
- Make use of `Sketch` and `Solid` ([#824])

#### `fj-viewer`

- Make events more high-level ([#803]; thank you [@jeevcat]!)

### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#799], [#800], [#801])
- Update list of sponsors ([#833])

[#799]: https://github.com/hannobraun/Fornjot/pull/799
[#800]: https://github.com/hannobraun/Fornjot/pull/800
[#801]: https://github.com/hannobraun/Fornjot/pull/801
[#802]: https://github.com/hannobraun/Fornjot/pull/802
[#803]: https://github.com/hannobraun/Fornjot/pull/803
[#806]: https://github.com/hannobraun/Fornjot/pull/806
[#809]: https://github.com/hannobraun/Fornjot/pull/809
[#810]: https://github.com/hannobraun/Fornjot/pull/810
[#811]: https://github.com/hannobraun/Fornjot/pull/811
[#812]: https://github.com/hannobraun/Fornjot/pull/812
[#813]: https://github.com/hannobraun/Fornjot/pull/813
[#817]: https://github.com/hannobraun/Fornjot/pull/817
[#818]: https://github.com/hannobraun/Fornjot/pull/818
[#819]: https://github.com/hannobraun/Fornjot/pull/819
[#820]: https://github.com/hannobraun/Fornjot/pull/820
[#822]: https://github.com/hannobraun/Fornjot/pull/822
[#823]: https://github.com/hannobraun/Fornjot/pull/823
[#824]: https://github.com/hannobraun/Fornjot/pull/824
[#825]: https://github.com/hannobraun/Fornjot/pull/825
[#826]: https://github.com/hannobraun/Fornjot/pull/826
[#827]: https://github.com/hannobraun/Fornjot/pull/827
[#833]: https://github.com/hannobraun/Fornjot/pull/833

[@jeevcat]: https://github.com/jeevcat


## v0.7.0 (2022-07-07)

The following changelog is a summary of user-visible changes made since the previous release. User-visible changes are changes visible to end users (who define CAD models using `fj` and `fj-app`), and changes to the API of Fornjot ecosystem crates.

For a full summary of all pull requests, feel free to check out the Weekly Dev Logs that cover the time period since the previous release:

- [2022-W18](https://www.fornjot.app/blog/weekly-dev-log/2022-w18/)
- [2022-W19](https://www.fornjot.app/blog/weekly-dev-log/2022-w19/)
- [2022-W20](https://www.fornjot.app/blog/weekly-dev-log/2022-w20/)
- [2022-W21](https://www.fornjot.app/blog/weekly-dev-log/2022-w21/)
- [2022-W22](https://www.fornjot.app/blog/weekly-dev-log/2022-w22/)
- [2022-W23](https://www.fornjot.app/blog/weekly-dev-log/2022-w23/)
- [2022-W24](https://www.fornjot.app/blog/weekly-dev-log/2022-w24/)
- [2022-W25](https://www.fornjot.app/blog/weekly-dev-log/2022-w25/)
- [2022-W26](https://www.fornjot.app/blog/weekly-dev-log/2022-w26/)

### [`fj`](https://crates.io/crates/fj)

The API that Fornjot models are written against.

- Support serialization using Serde ([#610], [#682], [#685], [#688])
- Add `Angle` ([#619], [#621], [#641])
- Add `#[fj::model]` macro ([#643], [#652], [#655], [#659])
- Fix memory leak in `Sketch` ([#646])

### [`fj-app`](https://crates.io/crates/fj-app)

The Fornjot application.

- Fix usability of `--parameters` ([#692])

In addition to the changes listed here, many of the changes to other crates, listed below, have a direct impact on the user experience of `fj-app`.

### [`fj-export`](https://crates.io/crates/fj-export)

Library for exporting Fornjot models to external file formats.

- Support export to STL ([#594], [#599], [#604])

### [`fj-host`](https://crates.io/crates/fj-host)

Library for loading and running Fornjot models in a host application.

No changes in this release.

### [`fj-interop`](https://crates.io/crates/fj-interop)

Library that defines types to enable interoperation between Fornjot components.

No changes in this release.

### [`fj-kernel`](https://crates.io/crates/fj-kernel)

Fornjot's CAD kernel library.

- Expand and update constructors of `Curve`/`Surface` ([#542], [#611], [#690], [#721])
- Implement some intersection tests ([#543], [#560], [#562])
- Replace `Line` with `fj_math::Line` ([#558])
- Update `Surface` API for point/vector conversion ([#561])
- Update conversion API of geometry types ([#564])
- Store local representation of vertices ([#574], [#625], [#627], [#751], [#752])
- Generate approximations in native coordinates ([#575], [#762])
- Replace `Circle` with `fj_math::Circle` ([#578])
- Store local representation of curves ([#579], [#591], [#750])
- Make `Face` easier to use, less redundant ([#593], [#597])
- Fix face orientation ([#628])
- Require surface coordinates when building faces or cycles ([#665])
- Add custom data type to represent edge vertices ([#667])
- Remove `Edge::new` ([#693])
- Move all objects to new `objects` module ([#694])
- Implement new validation infrastructure ([#705], [#706], [#707], [#709], [#710], [#718])
- Remove `Shape` API ([#715], [#716], [#730], [#732], [#733], [#735], [#736], [#737], [#738], [#743], [#747])
- Add `Local` to manage local forms; use it to replace `geometry::Point` ([#761])

### [`fj-math`](https://crates.io/crates/fj-math)

Library that provides math primitives for the Fornjot ecosystem.

- Rename `Transform::project_to_slice` to `project_to_array` ([#545])
- Add support for `Point`/`Vector` subtraction ([#547])
- Add `Vector::scalar_projection_onto` ([#553])
- Add `Line` ([#557], [#563])
- Improve `Aabb` API ([#559])
- Add `Circle` ([#577])
- Add `Triangle::normal` ([#600])

### [`fj-operations`](https://crates.io/crates/fj-operations)

Library that defines CAD operations, serving as a link between `fj` and `fj-kernel`.

- Make 2D difference operation more flexible ([#598])
- Fix bounding volume of swept shapes ([#623])
- Improve error handling ([#629], [#632])
- Reduce reliance on `Shape` ([#734])

### [`fj-proc`](https://crates.io/crates/fj-proc)

Procedural macros for the `fj` crate.

Initial release.

### [`fj-viewer`](https://crates.io/crates/fj-viewer)

Library that provides a model viewer.

- Fix field of view ([#614])
- Improve error handling ([#633], [#635])
- Extract `fj-window` ([#640])
- Fix camera rotation ([#644], [#669])
- Fix performance issue related to mouse movement ([#758])
- Simplify zoom, fix it for larger models ([#764], [#781])

### [`fj-window`](https://crates.io/crates/fj-window)

Library to embed `fj-viewer` in a winit-based window.

Initial release.

[#542]: https://github.com/hannobraun/Fornjot/pull/542
[#543]: https://github.com/hannobraun/Fornjot/pull/543
[#545]: https://github.com/hannobraun/Fornjot/pull/545
[#547]: https://github.com/hannobraun/Fornjot/pull/547
[#553]: https://github.com/hannobraun/Fornjot/pull/553
[#557]: https://github.com/hannobraun/Fornjot/pull/557
[#558]: https://github.com/hannobraun/Fornjot/pull/558
[#559]: https://github.com/hannobraun/Fornjot/pull/559
[#560]: https://github.com/hannobraun/Fornjot/pull/560
[#561]: https://github.com/hannobraun/Fornjot/pull/561
[#562]: https://github.com/hannobraun/Fornjot/pull/562
[#563]: https://github.com/hannobraun/Fornjot/pull/563
[#564]: https://github.com/hannobraun/Fornjot/pull/564
[#574]: https://github.com/hannobraun/Fornjot/pull/574
[#575]: https://github.com/hannobraun/Fornjot/pull/575
[#577]: https://github.com/hannobraun/Fornjot/pull/577
[#578]: https://github.com/hannobraun/Fornjot/pull/578
[#579]: https://github.com/hannobraun/Fornjot/pull/579
[#591]: https://github.com/hannobraun/Fornjot/pull/591
[#593]: https://github.com/hannobraun/Fornjot/pull/593
[#594]: https://github.com/hannobraun/Fornjot/pull/594
[#597]: https://github.com/hannobraun/Fornjot/pull/597
[#598]: https://github.com/hannobraun/Fornjot/pull/598
[#599]: https://github.com/hannobraun/Fornjot/pull/599
[#600]: https://github.com/hannobraun/Fornjot/pull/600
[#604]: https://github.com/hannobraun/Fornjot/pull/604
[#610]: https://github.com/hannobraun/Fornjot/pull/610
[#611]: https://github.com/hannobraun/Fornjot/pull/611
[#614]: https://github.com/hannobraun/Fornjot/pull/614
[#619]: https://github.com/hannobraun/Fornjot/pull/619
[#621]: https://github.com/hannobraun/Fornjot/pull/621
[#623]: https://github.com/hannobraun/Fornjot/pull/623
[#625]: https://github.com/hannobraun/Fornjot/pull/625
[#627]: https://github.com/hannobraun/Fornjot/pull/627
[#628]: https://github.com/hannobraun/Fornjot/pull/628
[#629]: https://github.com/hannobraun/Fornjot/pull/629
[#632]: https://github.com/hannobraun/Fornjot/pull/632
[#633]: https://github.com/hannobraun/Fornjot/pull/633
[#635]: https://github.com/hannobraun/Fornjot/pull/635
[#640]: https://github.com/hannobraun/Fornjot/pull/640
[#641]: https://github.com/hannobraun/Fornjot/pull/641
[#643]: https://github.com/hannobraun/Fornjot/pull/643
[#644]: https://github.com/hannobraun/Fornjot/pull/644
[#646]: https://github.com/hannobraun/Fornjot/pull/646
[#652]: https://github.com/hannobraun/Fornjot/pull/652
[#655]: https://github.com/hannobraun/Fornjot/pull/655
[#659]: https://github.com/hannobraun/Fornjot/pull/659
[#665]: https://github.com/hannobraun/Fornjot/pull/665
[#667]: https://github.com/hannobraun/Fornjot/pull/667
[#669]: https://github.com/hannobraun/Fornjot/pull/669
[#682]: https://github.com/hannobraun/Fornjot/pull/682
[#685]: https://github.com/hannobraun/Fornjot/pull/685
[#688]: https://github.com/hannobraun/Fornjot/pull/688
[#690]: https://github.com/hannobraun/Fornjot/pull/690
[#692]: https://github.com/hannobraun/Fornjot/pull/692
[#693]: https://github.com/hannobraun/Fornjot/pull/693
[#694]: https://github.com/hannobraun/Fornjot/pull/694
[#705]: https://github.com/hannobraun/Fornjot/pull/705
[#706]: https://github.com/hannobraun/Fornjot/pull/706
[#707]: https://github.com/hannobraun/Fornjot/pull/707
[#709]: https://github.com/hannobraun/Fornjot/pull/709
[#710]: https://github.com/hannobraun/Fornjot/pull/710
[#715]: https://github.com/hannobraun/Fornjot/pull/715
[#716]: https://github.com/hannobraun/Fornjot/pull/716
[#718]: https://github.com/hannobraun/Fornjot/pull/718
[#721]: https://github.com/hannobraun/Fornjot/pull/721
[#730]: https://github.com/hannobraun/Fornjot/pull/730
[#732]: https://github.com/hannobraun/Fornjot/pull/732
[#733]: https://github.com/hannobraun/Fornjot/pull/733
[#734]: https://github.com/hannobraun/Fornjot/pull/734
[#735]: https://github.com/hannobraun/Fornjot/pull/735
[#736]: https://github.com/hannobraun/Fornjot/pull/736
[#737]: https://github.com/hannobraun/Fornjot/pull/737
[#738]: https://github.com/hannobraun/Fornjot/pull/738
[#743]: https://github.com/hannobraun/Fornjot/pull/743
[#747]: https://github.com/hannobraun/Fornjot/pull/747
[#750]: https://github.com/hannobraun/Fornjot/pull/750
[#751]: https://github.com/hannobraun/Fornjot/pull/751
[#752]: https://github.com/hannobraun/Fornjot/pull/752
[#758]: https://github.com/hannobraun/Fornjot/pull/758
[#761]: https://github.com/hannobraun/Fornjot/pull/761
[#762]: https://github.com/hannobraun/Fornjot/pull/762
[#764]: https://github.com/hannobraun/Fornjot/pull/764
[#781]: https://github.com/hannobraun/Fornjot/pull/781


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
