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

## Result

The experiment is still ongoing.
