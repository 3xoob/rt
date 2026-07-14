# rt

A CPU-based ray tracer written in Rust that renders simple 3D scenes (spheres, planes, cubes, and cylinders) to PPM image files, driven entirely from the command line.

## Overview

`rt` casts rays from a camera through each pixel of an image, tests them against a small set of geometric primitives, and shades hits using an ambient + Lambertian diffuse + Phong specular lighting model with shadow rays. It supports recursive reflection and basic refraction, optional texture mapping from PPM images, and a particle system that scatters small random spheres into a scene. The renderer writes its output as ASCII PPM (`P3`) files under an `Output/` directory. The Cargo package itself is named `raytracer`.

## Features

- Ray/object intersection for four primitive types: sphere, plane, axis-aligned cube (slab test), and finite cylinder (`src/object.rs`)
- Phong-style shading: ambient term, Lambertian diffuse, specular highlight, and shadow rays (`src/raytracer.rs`)
- Recursive reflection (`Material.reflectivity`), bounded by a fixed recursion depth of 3
- Basic refraction (`Material.refractivity`) using a fixed index of refraction (no Fresnel term)
- Texture mapping onto spheres (spherical UV) and planes (planar UV) from a PPM source image (`src/object.rs`, `src/ppm.rs`)
- Particle system that generates randomly placed/colored small spheres via the `rand` crate (`generate_particles` in `src/main.rs`)
- Selectable built-in scenes and CLI-configurable image size, camera position, particle count, and output filename (`clap`-based CLI in `src/main.rs`)
- Custom PPM (P3) reader and writer (`src/ppm.rs`)

## Technologies

- Rust (edition 2024, per `Cargo.toml`)
- [`clap`](https://crates.io/crates/clap) 4.5 — command-line argument parsing
- [`rand`](https://crates.io/crates/rand) 0.9.1 — random particle generation

## Project Structure

```
src/
├── main.rs        # Entry point: CLI parsing, scene/object setup, scene dispatch, particle generation
├── object.rs       # Vec3 math, Object trait, and Sphere/Plane/Cube/Cylinder intersection logic
├── material.rs     # Material struct (reflectivity, refractivity, shininess, texture)
├── raytracer.rs    # RayTracer: ray casting, shading, shadows, reflection, refraction
├── scene.rs        # Camera, Light, and Scene structs
└── ppm.rs          # PPM (P3) image reader/writer and Texture struct
audit.sh            # Runs all scenes/feature combinations in release mode and archives outputs
markdown.md          # Feature/CLI reference documentation
```

## Requirements

- A Rust toolchain supporting the 2024 edition (Rust 1.85 or newer) with Cargo

## Installation

```bash
git clone https://github.com/3xoob/rt.git
cd rt
cargo build --release
```

## Usage

Run the renderer with Cargo, optionally passing CLI flags (parsed with `clap` in `src/main.rs`):

```bash
cargo run -- --scene all --width 800 --height 600
```

Available flags:

- `--textures` — enable texture mapping
- `--reflection` — enable reflection
- `--refraction` — enable refraction
- `--width <pixels>` — image width (default `800`)
- `--height <pixels>` — image height (default `600`)
- `--scene <name>` — scene to render: `all`, `plane_sphere`, `plane_cube`, `plane_cylinder`, `all_particles`, `particles`, `reflection`, `refraction` (default `all`)
- `--camera <x,y,z>` — camera position, e.g. `--camera 0.0,0.0,-5.0` (default `0.0,0.0,-5.0`)
- `--particles <n>` — number of particles for particle scenes (default `100`)
- `--output <filename>` — custom output file path (defaults to a scene-specific name under `Output/`)

Rendered images are written as `.ppm` files into an `Output/` directory, which is deleted and recreated on every run.

## Configuration

There are no environment variables. Behavior is configured entirely through the CLI flags listed above. When textures are enabled, the renderer looks for a texture image at the fixed path `Output/solid.ppm` (see Limitations below).

## Example

```bash
cargo run -- --scene plane_cube --width 1024 --height 768 --camera 0.0,2.0,-10.0 --output my_cube.ppm
```

This renders the plane+cube scene at 1024x768 with the camera at `(0.0, 2.0, -10.0)` and saves the result to `my_cube.ppm`.

There is also `audit.sh`, a shell script that builds the project in release mode and runs through every scene/feature combination, copying each resulting `.ppm` into a `Result/` directory for manual inspection.

## Limitations

- Fluid simulation is explicitly documented as **not implemented** in `markdown.md` — only the particle (random spheres) feature exists.
- Refraction uses a fixed index of refraction and does not account for the Fresnel effect.
- Reflection/refraction recursion depth is hardcoded to 3 (`render` calls `trace_ray(..., 3)` in `src/raytracer.rs`).
- Scenes, objects, and lights are hardcoded in `src/main.rs`; there is no external scene description file format.
- Texture loading reads from the fixed path `Output/solid.ppm`, but the `Output/` directory is deleted and recreated on every run, so a texture file must be present at that path before running (it is not shipped in the repository and will not persist between runs unless replaced each time).
- There are no automated tests in the repository (`audit.sh` is a manual smoke-test/audit script, not a test suite).

## License

This project is distributed under a custom, restrictive license — see [`LICENSE`](./LICENSE) and [`COPYRIGHT.md`](./COPYRIGHT.md). The source is published for portfolio and viewing purposes only; copying, modifying, redistributing, or commercially using it requires prior written permission from the copyright holder.
