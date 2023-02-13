# Fornjot - Changelog

## v0.36.0 (2023-02-13)

### End-user improvements

Improvements to Fornjot and its documentation that are visible to end users.

- Don't abort application, if model code panics ([#1534]; thank you, [@mxdamien]!)

### Ecosystem improvements

Improvements to Fornjot components that are relevant to developers building on top of those. These have an indirect effect on end users, through fixed bugs and improved robustness.

#### `fj-kernel`

- Make various fixes and small updates in builder API ([#1572])

### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#1560], [#1561], [#1562], [#1563], [#1564], [#1566], [#1567])
- Upgrade to Rust 1.67.1 ([#1574])

[#1534]: https://github.com/hannobraun/Fornjot/pull/1534
[#1560]: https://github.com/hannobraun/Fornjot/pull/1560
[#1561]: https://github.com/hannobraun/Fornjot/pull/1561
[#1562]: https://github.com/hannobraun/Fornjot/pull/1562
[#1563]: https://github.com/hannobraun/Fornjot/pull/1563
[#1564]: https://github.com/hannobraun/Fornjot/pull/1564
[#1566]: https://github.com/hannobraun/Fornjot/pull/1566
[#1567]: https://github.com/hannobraun/Fornjot/pull/1567
[#1572]: https://github.com/hannobraun/Fornjot/pull/1572
[#1574]: https://github.com/hannobraun/Fornjot/pull/1574

[@mxdamien]: https://github.com/mxdamien


## v0.35.0 (2023-02-06)

### End-user improvements

Improvements to Fornjot and its documentation that are visible to end users.

- Fixed status messages taking up too much space ([#1551]; thank you, [@tmayoff]!)
- Display version mismatch warning in GUI ([#1554]; thank you, [@tmayoff]!)

### Ecosystem improvements

Improvements to Fornjot components that are relevant to developers building on top of those. These have an indirect effect on end users, through fixed bugs and improved robustness.

#### `fj-kernel`

- Improve validation error message ([#1553])
- Lift limitation when inferring surface as plane ([#1556])
- Reuse cached curve approximation, if range is reversed ([#1557])

#### `fj-math`

- Improve projections into plane ([#1555])

### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#1546], [#1547], [#1548], [#1550])
- Update list of sponsors ([#1552])
- Updated nix flake inputs ([#1558]; thank you, [@Philipp-M]!)

[#1546]: https://github.com/hannobraun/Fornjot/pull/1546
[#1547]: https://github.com/hannobraun/Fornjot/pull/1547
[#1548]: https://github.com/hannobraun/Fornjot/pull/1548
[#1550]: https://github.com/hannobraun/Fornjot/pull/1550
[#1551]: https://github.com/hannobraun/Fornjot/pull/1551
[#1552]: https://github.com/hannobraun/Fornjot/pull/1552
[#1553]: https://github.com/hannobraun/Fornjot/pull/1553
[#1554]: https://github.com/hannobraun/Fornjot/pull/1554
[#1555]: https://github.com/hannobraun/Fornjot/pull/1555
[#1556]: https://github.com/hannobraun/Fornjot/pull/1556
[#1557]: https://github.com/hannobraun/Fornjot/pull/1557
[#1558]: https://github.com/hannobraun/Fornjot/pull/1558

[@tmayoff]: https://github.com/tmayoff
[@Philipp-M]: https://github.com/Philipp-M


## v0.34.0 (2023-01-30)

### End-user improvements

Improvements to Fornjot and its documentation that are visible to end users.

*None this week. Busy working on the kernel!*

### Ecosystem improvements

Improvements to Fornjot components that are relevant to developers building on top of those. These have an indirect effect on end users, through fixed bugs and improved robustness.

#### `fj-kernel`

- Continue simplifying object graph around `HalfEdge` ([#1535], [#1536])
- Add more debug information to approximation ([#1537])
- Improve validation error messages ([#1540])
- Respect existing boundary when updating `HalfEdge` as line segment ([#1541])

#### `fj-window`

- Box event loop error variants ([#1539])

### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#1528], [#1529], [#1530], [#1531], [#1533])
- Minimize scope of `unsafe` block ([#1538])
- Update release automation ([#1542])
- Upgrade to Rust 1.67.0 ([#1543])

[#1528]: https://github.com/hannobraun/Fornjot/pull/1528
[#1529]: https://github.com/hannobraun/Fornjot/pull/1529
[#1530]: https://github.com/hannobraun/Fornjot/pull/1530
[#1531]: https://github.com/hannobraun/Fornjot/pull/1531
[#1533]: https://github.com/hannobraun/Fornjot/pull/1533
[#1535]: https://github.com/hannobraun/Fornjot/pull/1535
[#1536]: https://github.com/hannobraun/Fornjot/pull/1536
[#1537]: https://github.com/hannobraun/Fornjot/pull/1537
[#1538]: https://github.com/hannobraun/Fornjot/pull/1538
[#1539]: https://github.com/hannobraun/Fornjot/pull/1539
[#1540]: https://github.com/hannobraun/Fornjot/pull/1540
[#1541]: https://github.com/hannobraun/Fornjot/pull/1541
[#1542]: https://github.com/hannobraun/Fornjot/pull/1542
[#1543]: https://github.com/hannobraun/Fornjot/pull/1543


## v0.33.0 (2023-01-23)

### End-user improvements

Improvements to Fornjot and its documentation that are visible to end users.

None this week, busy working on the kernel!

### Ecosystem improvements

Improvements to Fornjot components that are relevant to developers building on top of those. These have an indirect effect on end users, through fixed bugs and improved robustness.

#### `fj-kernel`

- Improve validation error message ([#1518])
- Add `FaceBuilder::infer_curves` ([#1520])
- Simplify object graph around `HalfEdge` ([#1521], [#1522], [#1524], [#1526], [#1527])

#### `fj-math`

- Fix `Plane::project_vector` ([#1523])

### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#1512], [#1517])

[#1512]: https://github.com/hannobraun/Fornjot/pull/1512
[#1517]: https://github.com/hannobraun/Fornjot/pull/1517
[#1518]: https://github.com/hannobraun/Fornjot/pull/1518
[#1520]: https://github.com/hannobraun/Fornjot/pull/1520
[#1521]: https://github.com/hannobraun/Fornjot/pull/1521
[#1522]: https://github.com/hannobraun/Fornjot/pull/1522
[#1523]: https://github.com/hannobraun/Fornjot/pull/1523
[#1524]: https://github.com/hannobraun/Fornjot/pull/1524
[#1526]: https://github.com/hannobraun/Fornjot/pull/1526
[#1527]: https://github.com/hannobraun/Fornjot/pull/1527


## v0.32.0 (2023-01-16)

### End-user improvements

Improvements to Fornjot and its documentation that are visible to end users.

- Fix app crashing, if it is minimized too long ([#1504])

### Ecosystem improvements

Improvements to Fornjot components that are relevant to developers building on top of those. These have an indirect effect on end users, through fixed bugs and improved robustness.

#### `fj-kernel`

- Improve builder API ([#1495], [#1501], [#1502], [#1509], [#1510])
- Don't stop on first validation error ([#1505])

### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#1490], [#1491], [#1492], [#1494])
- Fix rust-analyzer configuration in VS Code ([#1497])
- Upgrade to Rust 1.66.1 ([#1500])
- Box large error variants ([#1506]; thank you, [@zthompson47]!)

[#1490]: https://github.com/hannobraun/Fornjot/pull/1490
[#1491]: https://github.com/hannobraun/Fornjot/pull/1491
[#1492]: https://github.com/hannobraun/Fornjot/pull/1492
[#1494]: https://github.com/hannobraun/Fornjot/pull/1494
[#1495]: https://github.com/hannobraun/Fornjot/pull/1495
[#1497]: https://github.com/hannobraun/Fornjot/pull/1497
[#1500]: https://github.com/hannobraun/Fornjot/pull/1500
[#1501]: https://github.com/hannobraun/Fornjot/pull/1501
[#1502]: https://github.com/hannobraun/Fornjot/pull/1502
[#1504]: https://github.com/hannobraun/Fornjot/pull/1504
[#1505]: https://github.com/hannobraun/Fornjot/pull/1505
[#1506]: https://github.com/hannobraun/Fornjot/pull/1506
[#1509]: https://github.com/hannobraun/Fornjot/pull/1509
[#1510]: https://github.com/hannobraun/Fornjot/pull/1510

[@zthompson47]: https://github.com/zthompson47


## v0.31.0 (2023-01-09)

### End-user improvements

Improvements to Fornjot and its documentation that are visible to end users.

- Fix model updates freezing GUI; improve loading messages ([#1476]; thank you, [@zthompson47]!)
- Don't wrap `Angle` by default ([#1478]; thank you, [@antonok-edm]!)
- Support arcs in sketches ([#1484]; thank you, [@antonok-edm]!)

### Ecosystem improvements

Improvements to Fornjot components that are relevant to developers building on top of those. These have an indirect effect on end users, through fixed bugs and improved robustness.

#### `fj-kernel`

- Expand and clean up builder API ([#1479], [#1483], [#1485], [#1489])
- Remove `fj_kernel::iter` ([#1480])
- Remove `Vertex::global_form` ([#1481])
- Improve some validation error messages and validation test output ([#1486])

### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#1472], [#1473], [#1474], [#1475], [#1487])
- Expand and clean up release automation ([#1482])

[#1472]: https://github.com/hannobraun/Fornjot/pull/1472
[#1473]: https://github.com/hannobraun/Fornjot/pull/1473
[#1474]: https://github.com/hannobraun/Fornjot/pull/1474
[#1475]: https://github.com/hannobraun/Fornjot/pull/1475
[#1476]: https://github.com/hannobraun/Fornjot/pull/1476
[#1478]: https://github.com/hannobraun/Fornjot/pull/1478
[#1479]: https://github.com/hannobraun/Fornjot/pull/1479
[#1480]: https://github.com/hannobraun/Fornjot/pull/1480
[#1481]: https://github.com/hannobraun/Fornjot/pull/1481
[#1482]: https://github.com/hannobraun/Fornjot/pull/1482
[#1483]: https://github.com/hannobraun/Fornjot/pull/1483
[#1484]: https://github.com/hannobraun/Fornjot/pull/1484
[#1485]: https://github.com/hannobraun/Fornjot/pull/1485
[#1486]: https://github.com/hannobraun/Fornjot/pull/1486
[#1487]: https://github.com/hannobraun/Fornjot/pull/1487
[#1489]: https://github.com/hannobraun/Fornjot/pull/1489

[@zthompson47]: https://github.com/zthompson47
[@antonok-edm]: https://github.com/antonok-edm


## v0.30.0 (2023-01-02)

### End-user improvements

Improvements to Fornjot and its documentation that are visible to end users.

None this time!

### Ecosystem improvements

Improvements to Fornjot components that are relevant to developers building on top of those. These have an indirect effect on end users, through fixed bugs and improved robustness.

#### `fj-kernel`

- Simplify `CycleBuilder` and `FaceBuilder` ([#1467])

### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#1459], [#1460], [#1461], [#1462], [#1463], [#1465], [#1470])
- Upgrade to Rust 1.66.0 ([#1466])
- Replace `robust-predicates` ([#1468])
- Cross-compile to Android and iOS ([#1469])

[#1459]: https://github.com/hannobraun/Fornjot/pull/1459
[#1460]: https://github.com/hannobraun/Fornjot/pull/1460
[#1461]: https://github.com/hannobraun/Fornjot/pull/1461
[#1462]: https://github.com/hannobraun/Fornjot/pull/1462
[#1463]: https://github.com/hannobraun/Fornjot/pull/1463
[#1465]: https://github.com/hannobraun/Fornjot/pull/1465
[#1466]: https://github.com/hannobraun/Fornjot/pull/1466
[#1467]: https://github.com/hannobraun/Fornjot/pull/1467
[#1468]: https://github.com/hannobraun/Fornjot/pull/1468
[#1469]: https://github.com/hannobraun/Fornjot/pull/1469
[#1470]: https://github.com/hannobraun/Fornjot/pull/1470


## v0.29.0 (2022-12-19)

### End-user improvements

Improvements to Fornjot and its documentation that are visible to end users.

- Fix crash when minimizing window on Windows ([#1447]; thank you, [@kazatsuyu]!)

### Ecosystem improvements

Improvements to Fornjot components that are relevant to developers building on top of those. These have an indirect effect on end users, through fixed bugs and improved robustness.

#### `fj-kernel`

- Continue cleanup of object construction code ([#1445], [#1446], [#1448], [#1449], [#1450], [#1451], [#1452], [#1453], [#1456], [#1457])
- Fix doc comment ([#1458])

#### `fj-math`

- Return line coordinates from `Line::from_points` ([#1455])

### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#1437], [#1438], [#1440], [#1443], [#1444])

[#1437]: https://github.com/hannobraun/Fornjot/pull/1437
[#1438]: https://github.com/hannobraun/Fornjot/pull/1438
[#1440]: https://github.com/hannobraun/Fornjot/pull/1440
[#1443]: https://github.com/hannobraun/Fornjot/pull/1443
[#1444]: https://github.com/hannobraun/Fornjot/pull/1444
[#1445]: https://github.com/hannobraun/Fornjot/pull/1445
[#1446]: https://github.com/hannobraun/Fornjot/pull/1446
[#1447]: https://github.com/hannobraun/Fornjot/pull/1447
[#1448]: https://github.com/hannobraun/Fornjot/pull/1448
[#1449]: https://github.com/hannobraun/Fornjot/pull/1449
[#1450]: https://github.com/hannobraun/Fornjot/pull/1450
[#1451]: https://github.com/hannobraun/Fornjot/pull/1451
[#1452]: https://github.com/hannobraun/Fornjot/pull/1452
[#1453]: https://github.com/hannobraun/Fornjot/pull/1453
[#1455]: https://github.com/hannobraun/Fornjot/pull/1455
[#1456]: https://github.com/hannobraun/Fornjot/pull/1456
[#1457]: https://github.com/hannobraun/Fornjot/pull/1457
[#1458]: https://github.com/hannobraun/Fornjot/pull/1458

[@kazatsuyu]: https://github.com/kazatsuyu


## v0.28.0 (2022-12-12)

### End-user improvements

Improvements to Fornjot and its documentation that are visible to end users.

- Fail, if RUST_LOG is invalid ([#1435]; thank you, [@zthompson47]!)

### Ecosystem improvements

Improvements to Fornjot components that are relevant to developers building on top of those. These have an indirect effect on end users, through fixed bugs and improved robustness.

#### `fj-kernel`

- Improve behavior around validation failures ([#1417], [#1418], [#1436])
- Continue cleanup of object construction code ([#1419], [#1423], [#1428], [#1429], [#1430], [#1432], [#1433])

### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#1412], [#1414], [#1416])
- Fix some Clippy lints ([#1421]; thank you, [@danieleades]!)
- Improve usage and documentation of `justfile` ([#1422], [#1425])

[#1412]: https://github.com/hannobraun/Fornjot/pull/1412
[#1414]: https://github.com/hannobraun/Fornjot/pull/1414
[#1416]: https://github.com/hannobraun/Fornjot/pull/1416
[#1417]: https://github.com/hannobraun/Fornjot/pull/1417
[#1418]: https://github.com/hannobraun/Fornjot/pull/1418
[#1419]: https://github.com/hannobraun/Fornjot/pull/1419
[#1421]: https://github.com/hannobraun/Fornjot/pull/1421
[#1422]: https://github.com/hannobraun/Fornjot/pull/1422
[#1423]: https://github.com/hannobraun/Fornjot/pull/1423
[#1425]: https://github.com/hannobraun/Fornjot/pull/1425
[#1428]: https://github.com/hannobraun/Fornjot/pull/1428
[#1429]: https://github.com/hannobraun/Fornjot/pull/1429
[#1430]: https://github.com/hannobraun/Fornjot/pull/1430
[#1432]: https://github.com/hannobraun/Fornjot/pull/1432
[#1433]: https://github.com/hannobraun/Fornjot/pull/1433
[#1435]: https://github.com/hannobraun/Fornjot/pull/1435
[#1436]: https://github.com/hannobraun/Fornjot/pull/1436

[@danieleades]: https://github.com/danieleades
[@zthompson47]: https://github.com/zthompson47


## v0.27.0 (2022-12-05)

### End-user improvements

Improvements to Fornjot and its documentation that are visible to end users.

- Display more errors in the GUI and display more information about them ([#1405])

### Ecosystem improvements

Improvements to Fornjot components that are relevant to developers building on top of those. These have an indirect effect on end users, through fixed bugs and improved robustness.

#### `fj-kernel`

- Move validation to separate service ([#1403], [#1404])
- Continue cleanup of object construction code ([#1406], [#1407], [#1408], [#1409])
- Rename `GlobalVertex::from_position` to `new` ([#1410])
- Touch up documentation of objects ([#1411])

### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#1397], [#1398], [#1399], [#1400], [#1402])

[#1397]: https://github.com/hannobraun/Fornjot/pull/1397
[#1398]: https://github.com/hannobraun/Fornjot/pull/1398
[#1399]: https://github.com/hannobraun/Fornjot/pull/1399
[#1400]: https://github.com/hannobraun/Fornjot/pull/1400
[#1402]: https://github.com/hannobraun/Fornjot/pull/1402
[#1403]: https://github.com/hannobraun/Fornjot/pull/1403
[#1404]: https://github.com/hannobraun/Fornjot/pull/1404
[#1405]: https://github.com/hannobraun/Fornjot/pull/1405
[#1406]: https://github.com/hannobraun/Fornjot/pull/1406
[#1407]: https://github.com/hannobraun/Fornjot/pull/1407
[#1408]: https://github.com/hannobraun/Fornjot/pull/1408
[#1409]: https://github.com/hannobraun/Fornjot/pull/1409
[#1410]: https://github.com/hannobraun/Fornjot/pull/1410
[#1411]: https://github.com/hannobraun/Fornjot/pull/1411


## v0.26.0 (2022-11-28)

### End-user improvements

Improvements to Fornjot and its documentation that are visible to end users.

- Remove model generation feature to fix `cargo install` error ([#1373])
- Enable model version check on Windows ([#1374])
- Change messages to say "evaluating" instead of "compiling" ([#1396])

### Ecosystem improvements

Improvements to Fornjot components that are relevant to developers building on top of those. These have an indirect effect on end users, through fixed bugs and improved robustness.

#### `fj-kernel`

- Build service abstraction around `Objects` ([#1377], [#1384], [#1390], [#1392], [#1393])
- Fix `Store` iteration bug ([#1383])
- Simplify old builder structs ([#1388])
- Add `Object` enum ([#1391])

#### `fj-operations`

- Take `&mut Objects` in `Shape::compute_brep` ([#1389])
- Simplify return value of `Shape::compute_brep` ([#1394])

### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#1378], [#1379], [#1380], [#1381], [#1382], [#1386])
- Make some small cleanups ([#1395])

[#1373]: https://github.com/hannobraun/Fornjot/pull/1373
[#1374]: https://github.com/hannobraun/Fornjot/pull/1374
[#1377]: https://github.com/hannobraun/Fornjot/pull/1377
[#1378]: https://github.com/hannobraun/Fornjot/pull/1378
[#1379]: https://github.com/hannobraun/Fornjot/pull/1379
[#1380]: https://github.com/hannobraun/Fornjot/pull/1380
[#1381]: https://github.com/hannobraun/Fornjot/pull/1381
[#1382]: https://github.com/hannobraun/Fornjot/pull/1382
[#1383]: https://github.com/hannobraun/Fornjot/pull/1383
[#1384]: https://github.com/hannobraun/Fornjot/pull/1384
[#1386]: https://github.com/hannobraun/Fornjot/pull/1386
[#1388]: https://github.com/hannobraun/Fornjot/pull/1388
[#1389]: https://github.com/hannobraun/Fornjot/pull/1389
[#1390]: https://github.com/hannobraun/Fornjot/pull/1390
[#1391]: https://github.com/hannobraun/Fornjot/pull/1391
[#1392]: https://github.com/hannobraun/Fornjot/pull/1392
[#1393]: https://github.com/hannobraun/Fornjot/pull/1393
[#1394]: https://github.com/hannobraun/Fornjot/pull/1394
[#1395]: https://github.com/hannobraun/Fornjot/pull/1395
[#1396]: https://github.com/hannobraun/Fornjot/pull/1396


## v0.25.0 (2022-11-21)

### End-user improvements

Improvements to Fornjot and its documentation that are visible to end users.

- Fix potential soundness hole in version comparison ([#1358])
- Fix error when `cargo install`ing `fj-app` from `crates.io` ([#1364], [#1365])
- Soften shading ([#1366])
- Improve output of `--version` ([#1367])
- Fix triangulation of sharp, concave faces ([#1369])

### Ecosystem improvements

Improvements to Fornjot components that are relevant to developers building on top of those. These have an indirect effect on end users, through fixed bugs and improved robustness.

#### `fj-kernel`

- Add infrastructure for abstracting over access to referenced objects ([#1359])
- Continue cleanup of partial object API ([#1360], [#1361], [#1362])

### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#1345], [#1347], [#1348], [#1355])
- Update release procedure ([#1363])
- Make some smaller code cleanups ([#1368], [#1370], [#1371])

[#1345]: https://github.com/hannobraun/Fornjot/pull/1345
[#1347]: https://github.com/hannobraun/Fornjot/pull/1347
[#1348]: https://github.com/hannobraun/Fornjot/pull/1348
[#1355]: https://github.com/hannobraun/Fornjot/pull/1355
[#1358]: https://github.com/hannobraun/Fornjot/pull/1358
[#1359]: https://github.com/hannobraun/Fornjot/pull/1359
[#1360]: https://github.com/hannobraun/Fornjot/pull/1360
[#1361]: https://github.com/hannobraun/Fornjot/pull/1361
[#1362]: https://github.com/hannobraun/Fornjot/pull/1362
[#1363]: https://github.com/hannobraun/Fornjot/pull/1363
[#1364]: https://github.com/hannobraun/Fornjot/pull/1364
[#1365]: https://github.com/hannobraun/Fornjot/pull/1365
[#1366]: https://github.com/hannobraun/Fornjot/pull/1366
[#1367]: https://github.com/hannobraun/Fornjot/pull/1367
[#1368]: https://github.com/hannobraun/Fornjot/pull/1368
[#1369]: https://github.com/hannobraun/Fornjot/pull/1369
[#1370]: https://github.com/hannobraun/Fornjot/pull/1370
[#1371]: https://github.com/hannobraun/Fornjot/pull/1371


## v0.24.0 (2022-11-14)

### End-user improvements

Improvements to Fornjot and its documentation that are visible to end users.

- Don't require `--model` to pass a model ([#1323]; thank you, [@kopackiw]!)
- Add command to create a new model ([#1344]; thank you, [@MartinKavik]!)

### Ecosystem improvements

Improvements to Fornjot components that are relevant to developers building on top of those. These have an indirect effect on end users, through fixed bugs and improved robustness.

#### `fj-kernel`

- Complete transition to new validation infrastructure ([#1326], [#1328], [#1330])
- Continue cleaning up partial object API ([#1331], [#1334], [#1337], [#1338], [#1339], [#1340], [#1343])

#### `fj-operations`

- Remove use of old validation infrastructure ([#1329])

### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#1321])
- Make some minor code cleanups ([#1322], [#1332], [#1336])
- Fix some release automation issues ([#1324], [#1325], [#1333], [#1335]; thank you, [@kopackiw]!)
- Add Windows support to `export-validator` ([#1342]; thank you, [@MartinKavik]!)

[#1321]: https://github.com/hannobraun/Fornjot/pull/1321
[#1322]: https://github.com/hannobraun/Fornjot/pull/1322
[#1323]: https://github.com/hannobraun/Fornjot/pull/1323
[#1324]: https://github.com/hannobraun/Fornjot/pull/1324
[#1325]: https://github.com/hannobraun/Fornjot/pull/1325
[#1326]: https://github.com/hannobraun/Fornjot/pull/1326
[#1328]: https://github.com/hannobraun/Fornjot/pull/1328
[#1329]: https://github.com/hannobraun/Fornjot/pull/1329
[#1330]: https://github.com/hannobraun/Fornjot/pull/1330
[#1331]: https://github.com/hannobraun/Fornjot/pull/1331
[#1332]: https://github.com/hannobraun/Fornjot/pull/1332
[#1333]: https://github.com/hannobraun/Fornjot/pull/1333
[#1334]: https://github.com/hannobraun/Fornjot/pull/1334
[#1335]: https://github.com/hannobraun/Fornjot/pull/1335
[#1336]: https://github.com/hannobraun/Fornjot/pull/1336
[#1337]: https://github.com/hannobraun/Fornjot/pull/1337
[#1338]: https://github.com/hannobraun/Fornjot/pull/1338
[#1339]: https://github.com/hannobraun/Fornjot/pull/1339
[#1340]: https://github.com/hannobraun/Fornjot/pull/1340
[#1342]: https://github.com/hannobraun/Fornjot/pull/1342
[#1343]: https://github.com/hannobraun/Fornjot/pull/1343
[#1344]: https://github.com/hannobraun/Fornjot/pull/1344

[@kopackiw]: https://github.com/kopackiw
[@MartinKavik]: https://github.com/MartinKavik


## v0.23.0 (2022-11-07)

### End-user improvements

Improvements to Fornjot and its documentation that are visible to end users.

- Fix panic when quitting application ([#1296])
- Warn on full version mismatch of host and model ([#1300]; thank you, [@zthompson47]!)
- Improve status messages around model loading ([#1302])
- Fix panic on Windows when loading model version ([#1304], [#1308])

### Ecosystem improvements

Improvements to Fornjot components that are relevant to developers building on top of those. These have an indirect effect on end users, through fixed bugs and improved robustness.

#### `fj-kernel`

- Clean up partial object API ([#1294], [#1305], [#1309], [#1310], [#1312])
- Move most validation code to new validation infrastructure ([#1295], [#1299])
- Simplify `Cycle` and `Face` ([#1297])
- Improve `Debug` implementation of `Handle` ([#1298])
- Simplify `GlobalPath` transforms ([#1313])

#### `fj-viewer`

- Simplify interaction with `Gui` ([#1301])

### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#1290], [#1291], [#1292], [#1293], [#1314], [#1315], [#1316], [#1318], [#1319])
- Upgrade to Rust 1.65.0 ([#1306])
- Make some clean-ups in internal `fj` code ([#1311])

[#1290]: https://github.com/hannobraun/Fornjot/pull/1290
[#1291]: https://github.com/hannobraun/Fornjot/pull/1291
[#1292]: https://github.com/hannobraun/Fornjot/pull/1292
[#1293]: https://github.com/hannobraun/Fornjot/pull/1293
[#1294]: https://github.com/hannobraun/Fornjot/pull/1294
[#1295]: https://github.com/hannobraun/Fornjot/pull/1295
[#1296]: https://github.com/hannobraun/Fornjot/pull/1296
[#1297]: https://github.com/hannobraun/Fornjot/pull/1297
[#1298]: https://github.com/hannobraun/Fornjot/pull/1298
[#1299]: https://github.com/hannobraun/Fornjot/pull/1299
[#1300]: https://github.com/hannobraun/Fornjot/pull/1300
[#1301]: https://github.com/hannobraun/Fornjot/pull/1301
[#1302]: https://github.com/hannobraun/Fornjot/pull/1302
[#1304]: https://github.com/hannobraun/Fornjot/pull/1304
[#1305]: https://github.com/hannobraun/Fornjot/pull/1305
[#1306]: https://github.com/hannobraun/Fornjot/pull/1306
[#1308]: https://github.com/hannobraun/Fornjot/pull/1308
[#1309]: https://github.com/hannobraun/Fornjot/pull/1309
[#1310]: https://github.com/hannobraun/Fornjot/pull/1310
[#1311]: https://github.com/hannobraun/Fornjot/pull/1311
[#1312]: https://github.com/hannobraun/Fornjot/pull/1312
[#1313]: https://github.com/hannobraun/Fornjot/pull/1313
[#1314]: https://github.com/hannobraun/Fornjot/pull/1314
[#1315]: https://github.com/hannobraun/Fornjot/pull/1315
[#1316]: https://github.com/hannobraun/Fornjot/pull/1316
[#1318]: https://github.com/hannobraun/Fornjot/pull/1318
[#1319]: https://github.com/hannobraun/Fornjot/pull/1319

[@zthompson47]: https://github.com/zthompson47


## v0.22.0 (2022-10-31)

### End-user improvements

Improvements to Fornjot and its documentation that are visible to end users.

- Fix UI being blurry on some systems ([#1266]; thank you, [@erenoku]!)
- Improve error message when failing to load model ([#1268])
- Enable anti-aliasing ([#1274])
- Fix text of status messages looking jagged ([#1275])
- Fix some crashes, turn them into actionable errors ([#1276])
- Add UI to load model from within app, if no model is passed ([#1286], [#1288]; thank you, [@erenoku]!)

### Ecosystem improvements

Improvements to Fornjot components that are relevant to developers building on top of those. These have an indirect effect on end users, through fixed bugs and improved robustness.

#### `fj-host`

- Clean up API ([#1269])

#### `fj-kernel`

- Add new validation infrastructure ([#1279], [#1282], [#1283], [#1284], [#1285])
- Simplify handling of `MaybePartial` ([#1287])

#### `fj-math`

- Replace `Point::distance` with `distance_to` ([#1281])

### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update release procedure ([#1258])
- Clean up CI build ([#1259], [#1260])
- Update dependencies ([#1261], [#1262], [#1263], [#1264], [#1265], [#1267])
- Lower limits requested of the graphics backend ([#1273])
- Make sure `fj`'s `build.rs` doesn't run too often ([#1277])

[#1258]: https://github.com/hannobraun/Fornjot/pull/1258
[#1259]: https://github.com/hannobraun/Fornjot/pull/1259
[#1260]: https://github.com/hannobraun/Fornjot/pull/1260
[#1261]: https://github.com/hannobraun/Fornjot/pull/1261
[#1262]: https://github.com/hannobraun/Fornjot/pull/1262
[#1263]: https://github.com/hannobraun/Fornjot/pull/1263
[#1264]: https://github.com/hannobraun/Fornjot/pull/1264
[#1265]: https://github.com/hannobraun/Fornjot/pull/1265
[#1266]: https://github.com/hannobraun/Fornjot/pull/1266
[#1267]: https://github.com/hannobraun/Fornjot/pull/1267
[#1268]: https://github.com/hannobraun/Fornjot/pull/1268
[#1269]: https://github.com/hannobraun/Fornjot/pull/1269
[#1273]: https://github.com/hannobraun/Fornjot/pull/1273
[#1274]: https://github.com/hannobraun/Fornjot/pull/1274
[#1275]: https://github.com/hannobraun/Fornjot/pull/1275
[#1276]: https://github.com/hannobraun/Fornjot/pull/1276
[#1277]: https://github.com/hannobraun/Fornjot/pull/1277
[#1279]: https://github.com/hannobraun/Fornjot/pull/1279
[#1281]: https://github.com/hannobraun/Fornjot/pull/1281
[#1282]: https://github.com/hannobraun/Fornjot/pull/1282
[#1283]: https://github.com/hannobraun/Fornjot/pull/1283
[#1284]: https://github.com/hannobraun/Fornjot/pull/1284
[#1285]: https://github.com/hannobraun/Fornjot/pull/1285
[#1286]: https://github.com/hannobraun/Fornjot/pull/1286
[#1287]: https://github.com/hannobraun/Fornjot/pull/1287
[#1288]: https://github.com/hannobraun/Fornjot/pull/1288

[@erenoku]: https://github.com/erenoku


## v0.21.0 (2022-10-24)

### End-user improvements

Improvements to Fornjot and its documentation that are visible to end users.

- Improve error message, if model can't be loaded ([#1235])
- Make sure versions are compatible before loading model ([#1237])
- Always require model when starting `fj-app` ([#1242])
- Fix startup delay while model is compiling ([#1244])
- Print timestamp with each status update ([#1256]; thank you, [@erenoku]!)

### Ecosystem improvements

Improvements to Fornjot components that are relevant to developers building on top of those. These have an indirect effect on end users, through fixed bugs and improved robustness.

#### `fj-host`

- Rename `Watcher::receive`; improve its error handling ([#1234])

#### `fj-kernel`

- Fix last known object duplication issues ([#1233], [#1238])
- Integrate all remaining objects into centralized object storage ([#1246], [#1247], [#1248], [#1252], [#1255])
- Simplify use of `MaybePartial` ([#1253])
- Consolidate builder API for `Face` in `FaceBuilder` ([#1254])

#### `fj-viewer`

- Clean up API ([#1232])

#### `fj-window`

- Simplify `fj_window::run` arguments ([#1243], [#1245])

### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Compile Fornjot to WebAssembly ([#1221])
- Update release procedure ([#1225])
- Run export validation on macOS ([#1226])
- Update dependencies ([#1227], [#1228], [#1229])
- Clean up some code ([#1241], [#1251])
- Update `README.md` ([#1250])

[#1221]: https://github.com/hannobraun/Fornjot/pull/1221
[#1225]: https://github.com/hannobraun/Fornjot/pull/1225
[#1226]: https://github.com/hannobraun/Fornjot/pull/1226
[#1227]: https://github.com/hannobraun/Fornjot/pull/1227
[#1228]: https://github.com/hannobraun/Fornjot/pull/1228
[#1229]: https://github.com/hannobraun/Fornjot/pull/1229
[#1232]: https://github.com/hannobraun/Fornjot/pull/1232
[#1233]: https://github.com/hannobraun/Fornjot/pull/1233
[#1234]: https://github.com/hannobraun/Fornjot/pull/1234
[#1235]: https://github.com/hannobraun/Fornjot/pull/1235
[#1237]: https://github.com/hannobraun/Fornjot/pull/1237
[#1238]: https://github.com/hannobraun/Fornjot/pull/1238
[#1241]: https://github.com/hannobraun/Fornjot/pull/1241
[#1242]: https://github.com/hannobraun/Fornjot/pull/1242
[#1243]: https://github.com/hannobraun/Fornjot/pull/1243
[#1244]: https://github.com/hannobraun/Fornjot/pull/1244
[#1245]: https://github.com/hannobraun/Fornjot/pull/1245
[#1246]: https://github.com/hannobraun/Fornjot/pull/1246
[#1247]: https://github.com/hannobraun/Fornjot/pull/1247
[#1248]: https://github.com/hannobraun/Fornjot/pull/1248
[#1250]: https://github.com/hannobraun/Fornjot/pull/1250
[#1251]: https://github.com/hannobraun/Fornjot/pull/1251
[#1252]: https://github.com/hannobraun/Fornjot/pull/1252
[#1253]: https://github.com/hannobraun/Fornjot/pull/1253
[#1254]: https://github.com/hannobraun/Fornjot/pull/1254
[#1255]: https://github.com/hannobraun/Fornjot/pull/1255
[#1256]: https://github.com/hannobraun/Fornjot/pull/1256

[@erenoku]: https://github.com/erenoku


## v0.20.0 (2022-10-17)

### End-user improvements

Improvements to Fornjot and its documentation that are visible to end users.

- Improve error message, if model can't be found ([#1154]; thank you, [@ArshErgon]!)
- Remove old UI ([#1202])
- Invert default zoom direction; add config to override that ([#1204])
- Document convenient syntax for `fj` operations ([#1205])
- Remove the need to specify `crate-type` in `Cargo.toml` ([#1209])
- Fix some `wgpu`/`egui-winit` errors and warnings ([#1216])

### Ecosystem improvements

Improvements to Fornjot components that are relevant to developers building on top of those. These have an indirect effect on end users, through fixed bugs and improved robustness.

#### `fj-kernel`

- Rename `Stores` to `Objects`, move it to `objects` ([#1198])
- Provide access to default planes through `Objects` ([#1200])
- Fix more object duplication issues ([#1206], [#1207], [#1215], [#1218], [#1220], [#1222])
- Expand partial object API([#1212], [#1213])
- Integrate `SurfaceVertex` into centralized object storage ([#1214])
- Add methods to access single `HalfEdge` vertices ([#1219])

#### `fj-math`

- Fix `Triangle::winding` ([#1217])

#### `fj-operations`

- Remove redundant argument from `Shape::compute_brep` ([#1201])

#### `fj-viewer`

- Remove dependency on winit ([#1210])

### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update release procedure ([#1195], [#1208])
- Update dependencies ([#1196], [#1197])
- Add Nix build and dev-shell support via Nix flakes ([#1199], [#1203]; thank you, [@Philipp-M]!)
- Clean up egui-related code ([#1211])

[#1154]: https://github.com/hannobraun/Fornjot/pull/1154
[#1195]: https://github.com/hannobraun/Fornjot/pull/1195
[#1196]: https://github.com/hannobraun/Fornjot/pull/1196
[#1197]: https://github.com/hannobraun/Fornjot/pull/1197
[#1198]: https://github.com/hannobraun/Fornjot/pull/1198
[#1199]: https://github.com/hannobraun/Fornjot/pull/1199
[#1200]: https://github.com/hannobraun/Fornjot/pull/1200
[#1201]: https://github.com/hannobraun/Fornjot/pull/1201
[#1202]: https://github.com/hannobraun/Fornjot/pull/1202
[#1203]: https://github.com/hannobraun/Fornjot/pull/1203
[#1204]: https://github.com/hannobraun/Fornjot/pull/1204
[#1205]: https://github.com/hannobraun/Fornjot/pull/1205
[#1206]: https://github.com/hannobraun/Fornjot/pull/1206
[#1207]: https://github.com/hannobraun/Fornjot/pull/1207
[#1208]: https://github.com/hannobraun/Fornjot/pull/1208
[#1209]: https://github.com/hannobraun/Fornjot/pull/1209
[#1210]: https://github.com/hannobraun/Fornjot/pull/1210
[#1211]: https://github.com/hannobraun/Fornjot/pull/1211
[#1212]: https://github.com/hannobraun/Fornjot/pull/1212
[#1213]: https://github.com/hannobraun/Fornjot/pull/1213
[#1214]: https://github.com/hannobraun/Fornjot/pull/1214
[#1215]: https://github.com/hannobraun/Fornjot/pull/1215
[#1216]: https://github.com/hannobraun/Fornjot/pull/1216
[#1217]: https://github.com/hannobraun/Fornjot/pull/1217
[#1218]: https://github.com/hannobraun/Fornjot/pull/1218
[#1219]: https://github.com/hannobraun/Fornjot/pull/1219
[#1220]: https://github.com/hannobraun/Fornjot/pull/1220
[#1222]: https://github.com/hannobraun/Fornjot/pull/1222

[@ArshErgon]: https://github.com/ArshErgon
[@Philipp-M]: https://github.com/Philipp-M


## v0.19.0 (2022-10-10)

### End-user improvements

Improvements to Fornjot and its documentation that are visible to end users.

*None this week. Still busy improving the kernel!*

### Ecosystem improvements

Improvements to Fornjot components that are relevant to developers building on top of those. These have an indirect effect on end users, through fixed bugs and improved robustness.

#### `fj-kernel`

- Make minor cleanups in sweep code ([#1167])
- Fix various instances of duplicate objects being created ([#1168], [#1170], [#1172], [#1174])
- Expand and improve partial object API ([#1169], [#1171])
- Improve `Debug` implementation of `ObjectId` ([#1173])
- Simplify `HalfEdge` and `Vertex` ([#1175], [#1178])
- Expand scope of centralized object storage ([#1176], [#1179], [#1180])
- Clean up handling of vertices in normalized order ([#1181])

### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update release procedure ([#1165], [#1177])
- Update dependencies ([#1166], [#1182], [#1183], [#1184], [#1185], [#1186], [#1187], [#1188], [#1189], [#1192])

[#1165]: https://github.com/hannobraun/Fornjot/pull/1165
[#1166]: https://github.com/hannobraun/Fornjot/pull/1166
[#1167]: https://github.com/hannobraun/Fornjot/pull/1167
[#1168]: https://github.com/hannobraun/Fornjot/pull/1168
[#1169]: https://github.com/hannobraun/Fornjot/pull/1169
[#1170]: https://github.com/hannobraun/Fornjot/pull/1170
[#1171]: https://github.com/hannobraun/Fornjot/pull/1171
[#1172]: https://github.com/hannobraun/Fornjot/pull/1172
[#1173]: https://github.com/hannobraun/Fornjot/pull/1173
[#1174]: https://github.com/hannobraun/Fornjot/pull/1174
[#1175]: https://github.com/hannobraun/Fornjot/pull/1175
[#1176]: https://github.com/hannobraun/Fornjot/pull/1176
[#1177]: https://github.com/hannobraun/Fornjot/pull/1177
[#1178]: https://github.com/hannobraun/Fornjot/pull/1178
[#1179]: https://github.com/hannobraun/Fornjot/pull/1179
[#1180]: https://github.com/hannobraun/Fornjot/pull/1180
[#1181]: https://github.com/hannobraun/Fornjot/pull/1181
[#1182]: https://github.com/hannobraun/Fornjot/pull/1182
[#1183]: https://github.com/hannobraun/Fornjot/pull/1183
[#1184]: https://github.com/hannobraun/Fornjot/pull/1184
[#1185]: https://github.com/hannobraun/Fornjot/pull/1185
[#1186]: https://github.com/hannobraun/Fornjot/pull/1186
[#1187]: https://github.com/hannobraun/Fornjot/pull/1187
[#1188]: https://github.com/hannobraun/Fornjot/pull/1188
[#1189]: https://github.com/hannobraun/Fornjot/pull/1189
[#1192]: https://github.com/hannobraun/Fornjot/pull/1192


## v0.18.0 (2022-10-04)

### End-user improvements

Improvements to Fornjot and its documentation that are visible to end-users.

*None this week. Busy improving the kernel!*

### Ecosystem improvements

Improvements to the Fornjot ecosystem that are relevant to developers who are building on top of Fornjot components.

#### `fj-kernel`

- Improve some panic messages ([#1139], [#1149])
- Improve partial object API ([#1140], [#1144], [#1148], [#1150])
- Fix some code that creates duplicate global curves ([#1145], [#1151], [#1152])
- Remove redundant geometry from `GlobalCurve` ([#1146], [#1153])
- Make `GlobalEdge` undirected ([#1155])
- Validate winding of interior cycles of `Face` ([#1158])
- Add `HorizontalRayToTheRight::direction` ([#1159])
- Integrate `Surface` into centralized object storage ([#1163])

#### `fj-math`

- Add `Plane` ([#1157], [#1160])
- Expand and clean up API of `Vector` ([#1161])

### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update release procedure ([#1138])
- Expand release automation ([#1141])
- Update dependencies ([#1142])
- Update screenshot of test model ([#1156])

[#1138]: https://github.com/hannobraun/Fornjot/pull/1138
[#1139]: https://github.com/hannobraun/Fornjot/pull/1139
[#1140]: https://github.com/hannobraun/Fornjot/pull/1140
[#1141]: https://github.com/hannobraun/Fornjot/pull/1141
[#1142]: https://github.com/hannobraun/Fornjot/pull/1142
[#1144]: https://github.com/hannobraun/Fornjot/pull/1144
[#1145]: https://github.com/hannobraun/Fornjot/pull/1145
[#1146]: https://github.com/hannobraun/Fornjot/pull/1146
[#1148]: https://github.com/hannobraun/Fornjot/pull/1148
[#1149]: https://github.com/hannobraun/Fornjot/pull/1149
[#1150]: https://github.com/hannobraun/Fornjot/pull/1150
[#1151]: https://github.com/hannobraun/Fornjot/pull/1151
[#1152]: https://github.com/hannobraun/Fornjot/pull/1152
[#1153]: https://github.com/hannobraun/Fornjot/pull/1153
[#1155]: https://github.com/hannobraun/Fornjot/pull/1155
[#1156]: https://github.com/hannobraun/Fornjot/pull/1156
[#1157]: https://github.com/hannobraun/Fornjot/pull/1157
[#1158]: https://github.com/hannobraun/Fornjot/pull/1158
[#1159]: https://github.com/hannobraun/Fornjot/pull/1159
[#1160]: https://github.com/hannobraun/Fornjot/pull/1160
[#1161]: https://github.com/hannobraun/Fornjot/pull/1161
[#1163]: https://github.com/hannobraun/Fornjot/pull/1163


## v0.17.0 (2022-09-26)

### End-user improvements

Improvements to Fornjot and its documentation that are visible to end-users.

*None this week. Busy improving the kernel!*

### Ecosystem improvements

Improvements to the Fornjot ecosystem that are relevant to developers who are building on top of Fornjot components.

#### `fj-kernel`

- Implement centralized object storage ([#1108], [#1116], [#1121])
- Prepare for removing geometry from `GlobalCurve` ([#1111], [#1114])
- Start converting builder API into partial object API ([#1113], [#1117], [#1118], [#1119], [#1120], [#1123], [#1124], [#1126], [#1128], [#1130], [#1131], [#1133], [#1134], [#1135])
- Simplify `Triangulate` trait ([#1122])
- Clean up `Face` constructor ([#1125])
- Remove `HalfEdge::from_curve_and_vertices` ([#1127])

### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#1101], [#1103], [#1104], [#1105], [#1106], [#1107], [#1109])
- Remove unused dependencies ([#1110])
- Expand release automation ([#1115])
- Upgrade to Rust 1.64.0 ([#1132])
- Update list of sponsors in README ([#1136])

[#1101]: https://github.com/hannobraun/Fornjot/pull/1101
[#1103]: https://github.com/hannobraun/Fornjot/pull/1103
[#1104]: https://github.com/hannobraun/Fornjot/pull/1104
[#1105]: https://github.com/hannobraun/Fornjot/pull/1105
[#1106]: https://github.com/hannobraun/Fornjot/pull/1106
[#1107]: https://github.com/hannobraun/Fornjot/pull/1107
[#1108]: https://github.com/hannobraun/Fornjot/pull/1108
[#1109]: https://github.com/hannobraun/Fornjot/pull/1109
[#1110]: https://github.com/hannobraun/Fornjot/pull/1110
[#1111]: https://github.com/hannobraun/Fornjot/pull/1111
[#1113]: https://github.com/hannobraun/Fornjot/pull/1113
[#1114]: https://github.com/hannobraun/Fornjot/pull/1114
[#1115]: https://github.com/hannobraun/Fornjot/pull/1115
[#1116]: https://github.com/hannobraun/Fornjot/pull/1116
[#1117]: https://github.com/hannobraun/Fornjot/pull/1117
[#1118]: https://github.com/hannobraun/Fornjot/pull/1118
[#1119]: https://github.com/hannobraun/Fornjot/pull/1119
[#1120]: https://github.com/hannobraun/Fornjot/pull/1120
[#1121]: https://github.com/hannobraun/Fornjot/pull/1121
[#1122]: https://github.com/hannobraun/Fornjot/pull/1122
[#1123]: https://github.com/hannobraun/Fornjot/pull/1123
[#1124]: https://github.com/hannobraun/Fornjot/pull/1124
[#1125]: https://github.com/hannobraun/Fornjot/pull/1125
[#1126]: https://github.com/hannobraun/Fornjot/pull/1126
[#1127]: https://github.com/hannobraun/Fornjot/pull/1127
[#1128]: https://github.com/hannobraun/Fornjot/pull/1128
[#1130]: https://github.com/hannobraun/Fornjot/pull/1130
[#1131]: https://github.com/hannobraun/Fornjot/pull/1131
[#1132]: https://github.com/hannobraun/Fornjot/pull/1132
[#1133]: https://github.com/hannobraun/Fornjot/pull/1133
[#1134]: https://github.com/hannobraun/Fornjot/pull/1134
[#1135]: https://github.com/hannobraun/Fornjot/pull/1135
[#1136]: https://github.com/hannobraun/Fornjot/pull/1136


## v0.16.0 (2022-09-19)

### End-user improvements

Improvements to Fornjot and its documentation that are visible to end-users.

- Fix GUI not being loaded, if model is not available ([#1095]; thank you, [@payload]!)

### Ecosystem improvements

Improvements to the Fornjot ecosystem that are relevant to developers who are building on top of Fornjot components.

#### `fj-math`

- Fix `Vector::unit_v` ([#1085])
- Expand `Circle` API ([#1086], [#1088])
- Expand `Scalar` API ([#1087], [#1093])

#### `fj-kernel`

- Improve validation of `HalfEdge` and `Vertex` ([#1075])
- Expand builder API ([#1076], [#1083])
- Expand sweep test suite ([#1077])
- Perform various cleanups ([#1080], [#1084])
- Replace `CurveKind` with `SurfacePath`/`GlobalPath` ([#1081])
- Make path approximation deterministic ([#1089], [#1090], [#1094])
- Future-proof curve approximation code ([#1082], [#1091], [#1092], [#1096])
- Un-derive `Copy` from various object types ([#1097])

### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update dependencies ([#1071], [#1072], [#1074])
- Update release procedure ([#1073])
- Expand release automation ([#1078])
- Update list of sponsors in README ([#1098])

[#1071]: https://github.com/hannobraun/Fornjot/pull/1071
[#1072]: https://github.com/hannobraun/Fornjot/pull/1072
[#1073]: https://github.com/hannobraun/Fornjot/pull/1073
[#1074]: https://github.com/hannobraun/Fornjot/pull/1074
[#1075]: https://github.com/hannobraun/Fornjot/pull/1075
[#1076]: https://github.com/hannobraun/Fornjot/pull/1076
[#1077]: https://github.com/hannobraun/Fornjot/pull/1077
[#1078]: https://github.com/hannobraun/Fornjot/pull/1078
[#1080]: https://github.com/hannobraun/Fornjot/pull/1080
[#1081]: https://github.com/hannobraun/Fornjot/pull/1081
[#1082]: https://github.com/hannobraun/Fornjot/pull/1082
[#1083]: https://github.com/hannobraun/Fornjot/pull/1083
[#1084]: https://github.com/hannobraun/Fornjot/pull/1084
[#1085]: https://github.com/hannobraun/Fornjot/pull/1085
[#1086]: https://github.com/hannobraun/Fornjot/pull/1086
[#1087]: https://github.com/hannobraun/Fornjot/pull/1087
[#1088]: https://github.com/hannobraun/Fornjot/pull/1088
[#1089]: https://github.com/hannobraun/Fornjot/pull/1089
[#1090]: https://github.com/hannobraun/Fornjot/pull/1090
[#1091]: https://github.com/hannobraun/Fornjot/pull/1091
[#1092]: https://github.com/hannobraun/Fornjot/pull/1092
[#1093]: https://github.com/hannobraun/Fornjot/pull/1093
[#1094]: https://github.com/hannobraun/Fornjot/pull/1094
[#1095]: https://github.com/hannobraun/Fornjot/pull/1095
[#1096]: https://github.com/hannobraun/Fornjot/pull/1096
[#1097]: https://github.com/hannobraun/Fornjot/pull/1097
[#1098]: https://github.com/hannobraun/Fornjot/pull/1098

[@payload]: https://github.com/payload


## v0.15.0 (2022-09-12)

### End-user improvements

Improvements to Fornjot and its documentation that are visible to end-users.

*None this week; busy improving the kernel!*

### Ecosystem improvements

Improvements to the Fornjot ecosystem that are relevant to developers who are building on top of Fornjot components.

#### `fj-kernel`

- Improve sweep algorithm ([#1038], [#1054], [#1061], [#1063], [#1068])
- Add `SurfaceVertex` ([#1048])
- Produce better approximations, validate their correctness ([#1049], [#1053], [#1056], [#1058])
- Make triangulation more flexible ([#1050])
- Add `Faces` ([#1051])
- Simplify `Edge`; perform cleanups this enables ([#1055], [#1057], [#1059], [#1062])
- Rename `Edge` to `HalfEdge` ([#1064])
- Define face orientation by the winding of its exterior cycle ([#1066])
- Add API for finding faces ([#1067])

#### `fj-math`

- Add `Vector<2>::cross` ([#1065])

#### `fj-operations`

- Make use of `Faces` ([#1052])
- Remove unused parameter of `Shape::compute_brep` ([#1060])

### Internal Improvements

Improvements that are relevant to developers working on Fornjot itself.

- Update release procedure ([#1037])
- Update dependencies ([#1039], [#1040], [#1041], [#1042], [#1043], [#1044], [#1045], [#1047])
- Expand release automation ([#1046])

[#1037]: https://github.com/hannobraun/Fornjot/pull/1037
[#1038]: https://github.com/hannobraun/Fornjot/pull/1038
[#1039]: https://github.com/hannobraun/Fornjot/pull/1039
[#1040]: https://github.com/hannobraun/Fornjot/pull/1040
[#1041]: https://github.com/hannobraun/Fornjot/pull/1041
[#1042]: https://github.com/hannobraun/Fornjot/pull/1042
[#1043]: https://github.com/hannobraun/Fornjot/pull/1043
[#1044]: https://github.com/hannobraun/Fornjot/pull/1044
[#1045]: https://github.com/hannobraun/Fornjot/pull/1045
[#1046]: https://github.com/hannobraun/Fornjot/pull/1046
[#1047]: https://github.com/hannobraun/Fornjot/pull/1047
[#1048]: https://github.com/hannobraun/Fornjot/pull/1048
[#1049]: https://github.com/hannobraun/Fornjot/pull/1049
[#1050]: https://github.com/hannobraun/Fornjot/pull/1050
[#1051]: https://github.com/hannobraun/Fornjot/pull/1051
[#1052]: https://github.com/hannobraun/Fornjot/pull/1052
[#1053]: https://github.com/hannobraun/Fornjot/pull/1053
[#1054]: https://github.com/hannobraun/Fornjot/pull/1054
[#1055]: https://github.com/hannobraun/Fornjot/pull/1055
[#1056]: https://github.com/hannobraun/Fornjot/pull/1056
[#1057]: https://github.com/hannobraun/Fornjot/pull/1057
[#1058]: https://github.com/hannobraun/Fornjot/pull/1058
[#1059]: https://github.com/hannobraun/Fornjot/pull/1059
[#1060]: https://github.com/hannobraun/Fornjot/pull/1060
[#1061]: https://github.com/hannobraun/Fornjot/pull/1061
[#1062]: https://github.com/hannobraun/Fornjot/pull/1062
[#1063]: https://github.com/hannobraun/Fornjot/pull/1063
[#1064]: https://github.com/hannobraun/Fornjot/pull/1064
[#1065]: https://github.com/hannobraun/Fornjot/pull/1065
[#1066]: https://github.com/hannobraun/Fornjot/pull/1066
[#1067]: https://github.com/hannobraun/Fornjot/pull/1067
[#1068]: https://github.com/hannobraun/Fornjot/pull/1068


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
