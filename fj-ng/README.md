# Fornjot NG

## About

This is the experimental next generation prototyping for the already experimental Fornjot CAD toolkit. It tries to address a few shortcomings that the previous Fornjot effort has:

- Function representation (or implicit functions, or signed distance functions/fields; whatever you want to call it). Has turned out to be not such a great approach, in my opinion. I saw two main advantages, and both have been kinda negated for me, as I gained more experience:
  - The math is not as neat as you might think, at first. See the [discussion of "bound" SDFs here](https://iquilezles.org/www/articles/distfunctions/distfunctions.htm), or [this article about interior distance](https://iquilezles.org/www/articles/interiordistance/interiordistance.htm).
  - Aside from the SDFs themselves, you only need one algorithm to export and render them (see isosurface extraction), but all available algorithms are limited in some ways, and the better ones are very complicated to implement.
- Modeling productivity is completely killed by the compile times. (And the fact that the required features that would enable productive modeling aren't implemented, but that could be overcome.)
- The implementation has become complex, with both f-rep and mesh-based features.

This is going to be a partial rewrite. I plan to extract some parts of the old codebase, but will otherwise liberally reinvent the wheel here, for maximum prototyping effectiveness.

There are two focus areas to this new approach:
1. Don't get bogged down in complexity, by solving problems that enable specific models instead of trying to build generic CAD algorithms.
2. Make modeling productive by solving the compile-time problem.

As for that second point, I hope to avoid including a scripting language, because of the complexity that would involve, and because I don't know any that I like as much as Rust.


## Status

Vaporware. Right now, only this README exists.
