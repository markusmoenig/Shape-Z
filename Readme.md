# Shape-Z: Shape Prozessing Language

![Image](/examples/lighthouse.png)

**Shape-Z** is my take on a programming language to procedurally model shapes within a virtual and configurable voxel grid. Whether for rendering detailed images or exporting to 3D meshes.

It features many language constructs to make modeling and shaping easier, a fully featured **GLSL**-style math system plus support for fully programmable Disney BSDF materials. **Shape-Z** takes no shortcuts — it is a fully powered programming language that prioritizes flexibility without sacrificing capability.

**Shape-Z** has a recursive level of detail by unlimited subdivision of your voxel space.

## Features

**Shapes** define a 2D extrudable volume and its coordinate system (for example cartesian for a `Rect` or polar for a `Disc`). It sets the base coordinate system for a voxel box and can be further subdivided using **Segments**, for example `Left` is a segment of the `Rect` shape and would allow the creation of walls and other constructs relative to the left side of the rectangle.

Segments and shapes, like everything in **Shape-Z** are recursive and define `u`, `v`, and `d` coordinates (`uv` and the depth `d`) which allows for patterns and objects which can automatically adapt to the current subspace and UV layout.

Distance fields for 3D shapes like `Sphere` and `Box`, enabling precise volumetric modeling within segment-local coordinates together with fully programmable modifiers.

**Shape-Z** supports configurable **voxel density**, allowing you to repurpose the same model for pixel art or high-resolution mesh export.

A fully programmable, parallel, recursive and very fast virtual machine allows **Shape-Z** to compute millions of voxels in milli-seconds, allowing for instant previews and fast path-tracing.

## Use Shape-Z to

- Build 3D models and worlds in a resolution-independent way, and render them in full Disney BSDF glory.
- Export models to `.obj` format for integration in 3D engines and pipelines.
- Rapidly iterate and generate complex geometry with minimal code.

Shape-Z is ideal for game developers, artists, and procedural generation enthusiasts seeking a fast and expressive tool for creative modeling.

## Examples

For examples and documentation please visit the [Website](https://shapez-z.com).

## License

Shape-Z is dual-licensed under the same terms as Rust:

- **MIT License**  
- **Apache License (Version 2.0)**

You may choose either license to use Shape-Z according to your needs.
