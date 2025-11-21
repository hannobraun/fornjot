# Fornjot - Experiment 2025-11-07

## About

This experiment is packaged as a library. For now, it is not intended to demonstrate any functionality. It just sketches out some architectural ideas.

## Context

It has become clear, that Fornjot's current architecture is at a local maximum. I also think it is too complicated for what it does, and suspect that a simpler architecture would serve us much better going forward.

While it's certainly not impossible to address this piecemeal, through incremental improvements (which is the approach that I usually prefer), I don't think this is the best course of action.

Because while I don't consider the architecture to be very good, it is still consistent and self-reinforcing. Whenever I try to simplify one aspect, I run into the problem that it's there for a reason; that other aspects of the architecture depend on it being the way it is.

And while I haven't figured out yet how to break out of this situation, I do have quite a few unproven ideas on how an improved architecture would look like, redesigned from the ground up using the experience I've gained over the last few years.

This experiment is the fourth in a series meant to prove out those ideas. The results should provide a clearer picture of what is possible, and how the current architecture can be evolved.

## Setup

This experiment is a ground-up re-imagining of [the previous one](../2025-03-18). It may or may not replace its predecessor. For now, the goal is just to sketch out some new architectural ideas, to get a better sense of whether a full-blown effort based on those would be promising.

Specifically, I have the following objectives:

- Clearly distinguish between owned and shared topological objects.
  The previous experiment mostly references all objects through a `Handle`, _as if_ they could be shared. But not all are. Changing this simplifies the object graph, hopefully making it easier to understand and manage.
- Return to representing all geometry in terms of local coordinates.
  The previous experiment's README file has a full writeup on why that may be advantageous.
- Explore how local representation of geometry could simplify approximation.
  The mainline code already uses local geometry, but it does not use approximations as uniform intermediate representations. It is unclear to me how those two concepts are going to interact.

In addition, I want to take a new approach to achieving these objectives: focus on the core data structures and how to process them. Completely ignore the operations that build or modify these core data structures. My example models would create those "manually".

So essentially, get the core and the backend working, and leave the frontend for later. This is not going to be ideal, as supporting the frontend/operations side is a big part of what the core data structures need to do.

But doing everything at once is a huge task, and one I haven't done too well at in the past. I hope that by compartmentalizing, I can do a better job overall, knowing that revisions will be necessary later on, when I'm ready to tackle the frontend.

## Results

The experiment is still ongoing.
