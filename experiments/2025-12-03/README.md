# Fornjot - Experiment 2025-12-03

## About

This document presents a new architecture that could solve the problems that previous experiments have failed to address. So far, this only exists as a concept. I may or may not turn this into a full experiment.

## Context

Fornjot's mainline code, at the root of this repository, has reached a local maximum. It has become mired in complexity. In response, I started this series of experiments.

Initially, I experimented with quite a few ideas. Over time, I pared that down to just a few core concepts: approximated geometry as a uniform intermediate representation, and a simplified topological objects graph.

As [experiment 2025-03-18](../2025-03-18) has shown, there's a limit though, to how much you can simplify the object graph. The mainline object graph contains redundant local definitions of geometry, a significant source of its complexity. Leaving those out was a big improvement, but also caused problems.

These local definitions where there for a reason. You need them later, to create approximated geometry from the topological object graph, and reconstructing them can be problematic. In response, [experiment 2025-11-07](../2025-11-07) added those redundant local definitions back.

There's a price to pay though. This is how you define a cube using the latest core data structures from that experiment:
https://github.com/hannobraun/fornjot/blob/4ad899c22499f51161032b7d14a4a6993c7a7165/experiments/2025-11-07/src/main.rs#L29-L619

It's a lot! Now don't get me wrong, eventually we'd have convenient APIs to do all of this for us. But that doesn't change the fact that under the hood, there is quite a lot of complexity. And we'll have to pay for that, with every piece of code that interfaces with those core data structures.

This leaves us between a rock and a hard place. Either we maintain a complex object graph, or we throw away relevant information and run into trouble reconstructing it later.

But what if we _didn't_ need that information later? That's the core premise of this idea.

## Concept

### Core Data

So the core idea is this: Instead of maintaining complicated information until we need it later, or throwing that information away and run into trouble reconstructing it, we instead use the complicated information _right away_, so we don't need to maintain nor reconstruct it.

But let's take a step back and explore what I've come up with step by step, from the bottom up. To make this easier, let's use an example: a simple cube.

The core piece that everything else builds on top of, is an append-only list of vertices. Here's what that would look like for a cube:

```
v0 = [0, 0, 0]
v1 = [1, 0, 0]
v2 = [0, 1, 0]
v3 = [1, 1, 0]
v4 = [0, 0, 1]
v5 = [1, 0, 1]
v6 = [0, 1, 1]
v7 = [1, 1, 1]
```

This alone doesn't do much. Yes, since we know it's a cube, we can figure out what those vertices mean. But in the general case, it's just a point cloud. Not very helpful.

So let's layer a second structure on top, an append-only list of triangles. These triangles reference the vertices from above:

```
t0 = [v0, v1, v4]
t1 = [v1, v5, v4]
t2 = [v1, v3, v7]
...
```

Now we're getting somewhere! We can render this triangle mesh to the screen. Or we can export it to a 3MF or STL file for 3D printing.

But this is just raw geometry. It lacks the topological structure that boundary representation provides. We could try to reconstruct that. Figure out which triangles form faces, which vertices form polylines that bound those faces. But that would be hard, pretty much impossible in the general case.

And without that topological structure, we lack the means to manipulate the geometry in certain ways. We can't say, "chamfer the edges along the top face", because none of those concepts are represented in our core data. And yeah, we could figure it out for this simple cube, but it would be hard in other cases.

### Layered Topological Structure

We may not be able to _infer_ the topological structure of arbitrary unstructured geometry. But we can certainly _know_ that structure while we construct the geometry, and then just remember it.

Let's restart our cube example, and consider how we could construct it from the ground up. Let's start with a sketch that describe the bottom of the cube.

```
sketch:
  start: [0, 0]
  line_to: [1, 0]
  line_to: [1, 1]
  line_to: [0, 1]
  line_to: start
```

The syntax I use here is not important. It's just an abstract representation of what we could be doing in whatever language, using an arbitrary API. And that is to define a 2D sketch out of a few lines. We could have used arcs, or splines, or whatever else. But to keep this example simple, we use lines.

This sketch exists outside of the context of the core data structures we saw above. It's just an auxiliary concept, a helper, that we can use to define geometry for those core data structures. Let's do just that.

```
face:
  from_sketch: ... # insert the sketch we defined above!
  on_surface:
    plane:
      origin: [0, 0, 0]
      u_axis: [1, 0, 0]
      v_axis: [0, 1, 0]
```

Here we define a face, to connect our 2D sketch with a 3D surface. Again, the syntax is not important, just the abstract concept is. What this allows us to do, is take the 2D points from the sketch and add them to our list of vertices.

```
v0 = [0, 0, 0] # from [0, 0]
v1 = [1, 0, 0] # from [1, 0]
v2 = [1, 1, 0] # from [1, 1]
v3 = [0, 1, 0] # from [0, 1]
```

It's also quite trivial to get triangles from that sketch above. Fornjot is using Delaunay triangulation (to support curved surfaces) with a filtering step afterwards (to support holes in faces). Regardless of how specifically we do it, this gives us triangles for our list.

```
t0 = [v0, v1, v2]
t1 = [v0, v2, v3]
```

(Perceptive readers might notice that due to how I set up the plane, those triangles now face upwards, according to common practice, which makes them unsuitable for the bottom of our cube. I'm sorry. I did it to keep the example simple. I promise I'll do it the right way once I implement this in code.)

But wait, that's not all! We can get more information out of the sketch and the face we used to construct this, and we can store this information in additional layers. We might store a list of edges.

```
e0 = [v0, v1]
e1 = [v1, v2]
e2 = [v2, v3]
e3 = [v3, v0]
```

And of course we could store how all of the above relates to the face.

```
f0 = {
  triangles: [t0, t1]
  boundary: [e0, e1, e2, e3]
}
```

Now we have pretty much all of the usual topological structure that b-rep would give us, and we can use that for further operations. For example, we could do a "sweep" (or "extrude", whatever you wanna call it), from our face `f0` and the vector `[0, 0, 1]`, creating a cube.

That would add more vertices, triangles, edges, and faces to the lists we already have. In addition, it could start a new list of solid object, adding a single solid to represent the cube, that refers to all of the faces.

From there, we might perform typical b-rep operations like "chamfer all edges of solid `s0`", or "bore a hole into `s0` from the center of `f0`". We have all of the topological information we need for those things, along with the raw geometry.

### Extrapolating to More Complex Cases

As I noted earlier, our sketch could have consisted of arcs or other curves, not just lines. We could also have created a more complex solid by sweeping the original face along a spline instead of a line.

This would require approximation. Each operation would take a tolerance value, to determine how closely its output would need to approximate the arcs or splines. Edges would then refer to all the vertices that approximate them. Faces to all the triangles.

### Potential Drawbacks

"But what about perfect mathematical accuracy?", I hear you say. Well, we wouldn't have that. And I don't think it would actually be a problem.

First, you wouldn't throw away the operations (like sketch, sweep) that generated your approximations. If you defined them in code, that code is still there. In a graphical CAD application, you'd store them in some other way. You could always go back and get more accuracy, should you need to.

Second, approximations are going to happen anyway at some point, down the line. If you're 3D printing, you're working with mesh-based file formats. According to my understanding, CAM systems typically also approximate geometry as a triangle mesh, before doing their thing.

Every manufacturing technique is going to have some tolerance. Nothing is going to be perfect. As long as the CAD system generates approximations that are good enough for a given case, that should be fine.

Third, even classical b-rep kernels aren't purely mathematical. You need approximations to represent the intersection of two NURBS surfaces, for example. I don't think you can get away from that.

There are some unanswered questions that could turn into problems later. Export to STEP files is one of those. Another is that at a given complexity and required accuracy, you might have to store too more vertices, triangles, and whatever else, than your computer has room for.

I don't know how much of a problem this would actually be, especially compared to other CAD software. But what I can say for sure, is that Fornjot's first useful version (which is what this project is still working towards) definitely won't be perfect. There will be drawbacks.

### Conclusion

Maybe the core problem of the old approach, is that it builds up a full topological b-rep structure first, then creates an approximation from that afterwards. If that is true, we may be able to address that by building up the topological structure and its approximation together, step by step.

This new approach would do that, and it would do that in a way that seems to preserve the advantages of b-rep. Maybe it also has hidden problems that would make it unworkable. But so far, I haven't seen any.

Currently, I'm still undecided. But maybe this is worth a try.
