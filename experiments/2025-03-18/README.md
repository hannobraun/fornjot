# Fornjot - Experiment 2025-03-18

## About

This experiment is packaged as a single application. Run it with `cargo run`.
This should open a window and also create a 3MF file in this directory.

## Context

It has become clear, that Fornjot's current architecture is at a local maximum.
I also think it is too complicated for what it does, and suspect that a simpler
architecture would serve us much better going forward.

While it's certainly not impossible to address this piecemeal, through
incremental improvements (which is the approach that I usually prefer), I don't
think this is the best course of action.

Because while I don't consider the architecture to be very good, it is still
consistent and self-reinforcing. Whenever I try to simplify one aspect, I run
into the problem that it's there for a reason; that other aspects of the
architecture depend on it being the way it is.

And while I haven't figured out yet how to break out of this situation, I do
have quite a few unproven ideas on how an improved architecture would look like,
redesigned from the ground up using the experience I've gained over the last few
years.

This experiment is the third in a series meant to prove out those ideas. The
results should provide a clearer picture of what is possible, and how the
current architecture can be evolved.

## Setup

This experiment builds on [the second one](../2024-12-09/). There are two
objectives:

- Simplify the `Object` trait by removing as much functionality as possible.
  Compensate for the loss of insight into an object's structure by experimenting
  with other means of providing required debug information.
- Expand the existing b-rep primitives, adding support for curved surfaces.

## Results

The experiment is still ongoing, but I already have some ideas that a possible next experiment might or might not pick up.

### Possible Return to Locally Defined Geometry

#### Locally Defined Geometry

The current mainline code defines geometry locally. For example, a vertex is defined as a 1-dimensional object on a curve. That curve is defined as a 2-dimensional object on a surface.

Since vertices are shared between edges and edges are shared between faces, multiple redundant definitions might exist for any single object. This causes complexity in the object graph, hence complexity in the code constructing it, and requires intelligent caching when approximating those shared objects.

Caching probably makes sense anyway, for performance reasons alone. And I believe that, aside from its effect on construction, the drawbacks of the additional graph complexity are negligible. This leaves the complexity of graph construction as the only argument against this approach.

#### Globally Defined Geometry

This experiment went with the ostensibly simpler approach of storing all geometry in 3D, making object graph constructions _much_ simpler, at least within the scope of what this experiment has explored so far. However, this came at a cost.

These local definitions are still needed in some situations, and getting them has turned out to be a problem. It requires additional infrastructure, namely the capability of projecting into a curve/surface. I don't want to overstate this though, as it's exceedingly likely this capability is needed anyway.

The bigger problem is conceptual. By constructing all geometry in 3D, we're throwing away information about local coordinates that would be available at the time of construction. This information can't be reconstructed reliably, as there are degenerate cases where a 3D coordinate maps to multiple 2D ones.

It is possible to work around this. But I am concerned that this is just one example of a whole class of problem, and that the need for more workarounds awaits in the future.

#### The Case for Going Local

Neither approach is perfect. So the question is how to weigh their respective advantages and disadvantages. I think that giving locally defined geometry another try might be worth it, for two reasons.

First, I'd like to revisit the problem of constructing the more complicated object graph. I haven't worked with the mainline code in over a year, and I'd be curious to see how I'd approach this with a fresh perspective, and benefiting from the improvements made in these experiments.

Maybe I'm simply managing to fool myself into believing I could do better today, because the memories of the pain are no longer that fresh. But it might be worth to give it a try and actually confirm that this is the case, instead of giving up prematurely.

Second, I think that throwing away information that you can't reliably reconstruct later is a very foundational problem. My instincts tell me that this will keep causing problems down the line, that might be hard or impossible to work around. The problem of redundant, local geometry seems more manageable.
