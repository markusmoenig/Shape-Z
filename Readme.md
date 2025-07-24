# Shape-Z: Shape Prozessing Language

![Image](/examples/lighthouse.png)

**Shape-Z** is a recursive, voxel-based modeling language inspired by **GLSL** and shader-like workflows. It is designed for the procedural generation of 3D worlds using simple, expressive code. Unlike traditional modeling tools, Shape-Z treats models as programmable entities—built from logical operations, shapes, and materials—executed on a virtual voxel grid.

> ⚠️ *Shape-Z is in the early phase of development and its syntax and features may change.*

## Core Features

At its core, Shape-Z offers a minimal yet powerful syntax to define:

- **Shapes** like `Rect`, `Disc` that describe distance-based extrusion.
- Recursive **segments** (e.g., `Floor`, `Left`) that subdivide space and define the UV/extrusion domain for patterns and structured modeling.
- **Patterns** such as `Bricks`, `Modulo`, which automatically adapt to the current subdomain.
- Fully programmable **Disney BSDF materials** that can interact with patterns and noise.
- **Procedural logic** using conditionals, noise functions, and math in a GLSL-like syntax.

You write code that **describes how space is filled**, using recursive composition, conditional branching, and procedural evaluation. This enables the creation of anything from detailed architecture and terrain to stylized props and abstract geometry—all in a compact text format.

## Performance & Flexibility

For example, the lighthouse model shown above consists of ~11 million voxels and is fully generated in under a second (at a density of 50 per tile).

Shape-Z supports configurable **voxel density**, allowing you to repurpose the same model for pixel art or high-resolution mesh export.

## Use Shape-Z to

- Build 3D models and worlds in a resolution-independent way, and render them in full Disney BSDF glory.
- Export models to `.obj` format for integration in 3D engines and pipelines.
- Rapidly iterate and generate complex geometry with minimal code.

Shape-Z is ideal for game developers, artists, and procedural generation enthusiasts seeking a fast and expressive tool for creative modeling.

## License

Shape-Z is dual-licensed under the same terms as Rust:

- **MIT License**  
- **Apache License (Version 2.0)**

You may choose either license to use Shape-Z according to your needs.

# Example

![Bathroom](/examples/bathroom.png)

To create the left wall of the example bathroom scene, we create a `Left` space inside a rectangle and extrude the contents from the space based on the `d` (depth) parameter and patterns which adjust to the space.

A code segment to show the syntax is below. For more info and detailed documentation please visit the [Shape-Z Homepage](https://shape-z.com).

```glsl
voxel Box size = vec3(5, 3, 5) {
    shape Rect {
        segment Left depth = 1.0 {
            if d <= 0.05 {
                // Back Wall Color
                White;
            } else
            if d < 0.2 {
                // Bricks in the middle of the wall
                pattern Bricks {
                    brick {
                        Bricks;
                    }
                    cement {
                        White;
                    }
                } 
            }

            // Cabinet
            shape Rect at = vec2(0.5, 0.0), size = vec2(3.5, 1.2) {
                if d > 0.2 {
                    White;
                }

                if d > 0.9 {
                    // Relief
                    let relief = fract(u / 0.3);            
                    if v < 1.0 && relief > 0.1 && relief < 0.8 {
                        Clear;
                    }
                }
            }
        }
    }
    ```
