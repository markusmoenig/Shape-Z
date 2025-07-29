# Shape-Z: Shape Prozessing Language

![Image](images/lighthouse.png)

**Shape-Z** is a recursive, voxel-based programming language inspired by **GLSL** and shader-like workflows. It is designed for the procedural generation of 3D worlds using simple, expressive code. Unlike traditional modeling tools, Shape-Z treats models as programmable entities—built from logical operations, shapes, and materials—executed on a virtual voxel grid.

> ⚠️ *Shape-Z is in the early phase of development and its syntax and features may change.*

## Core Features

At its core, **Shape-Z** offers a minimal yet expressive syntax to define:

- **Shapes** like `Rect` and `Disc` that define both the foundational volume within the voxel grid **and the coordinate system** (e.g., cartesian for `Rect`, polar for `Disc`), enabling context-aware modeling and pattern placement.
- Recursive **segments** (e.g., `Floor`, `Left`) that subdivide space and define the UV/extrusion domain for patterns and hierarchical modeling.
- **Patterns** such as `Bricks` and `Modulo` that automatically adapt to the current subspace and UV layout.
- **Distance fields** to 3D shapes like `Sphere` and `Box`, enabling precise volumetric modeling within segment-local coordinates.
- Fully programmable **Disney BSDF materials** that interact with noise, patterns, and lighting.
- **Procedural logic**, including conditionals, mathematical operations, and noise functions, written in a **GLSL**-like style.
- Configurable **model density**, allowing output from pixel-art fidelity to dense, smooth geometry.
- Support for direct **rendering** or **export to .OBJ** for integration into 3D pipelines.

You write code that **precisely defines how space is filled**, using recursive structure, branching logic, and procedural evaluation. This gives you full control to build efficient, high-resolution geometry with minimal, expressive input.

## Performance & Flexibility

The lighthouse model shown above consists of ~11 million voxels and is fully generated in under a second (at a density of 50 per tile).

Shape-Z supports configurable **voxel density**, allowing you to repurpose the same model for pixel art or high-resolution mesh export.

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

![Image](images/bathroom.png)
