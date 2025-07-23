# Raytracer Documentation

## Feature Summary

| Feature         | How to Use                        | Example CLI Flag(s)                | Code Example Location      |
|-----------------|-----------------------------------|------------------------------------|---------------------------|
| Textures        | Enable with CLI flag, assign PPM  | `--textures`                       | `src/main.rs`, `src/object.rs` |
| Reflection      | Enable with CLI flag              | `--reflection`                     | `src/main.rs`             |
| Refraction      | Enable with CLI flag              | `--refraction`                     | `src/main.rs`             |
| Image Size      | Set width/height with CLI flags   | `--width`, `--height`              | `src/main.rs`             |
| Particles       | Auto-generated spheres            | (no flag, always enabled)          | `src/main.rs`             |

---

## 1. Using Textures

**Step-by-step:**
1. Enable textures by running with the `--textures` flag:
   ```bash
   cargo run -- --textures
   ```
2. The program loads a PPM image (e.g., `Output/solid.ppm`) and applies it to objects.
3. Texture mapping is supported for spheres and planes.

**Code Example:**
```rust
let texture = if enable_textures {
    ppm::read_ppm("Output/solid.ppm").ok()
} else {
    None
};

let sphere = Sphere {
    // ...
    material: material::Material {
        texture: texture.clone(),
        ..Default::default()
    },
};
```
*See `src/object.rs` for mapping details.*

---

## 2. CLI Flags

**Available Flags:**
- `--textures` : Enable textures
- `--reflection` : Enable reflection
- `--refraction` : Enable refraction
- `--width <pixels>` : Set image width
- `--height <pixels>` : Set image height
- `--scene <name>` : Select which scene to render (`all`, `plane_sphere`, `plane_cube`, `plane_cylinder`, `all_particles`, `particles`, `reflection`, `refraction`)
- `--camera <x,y,z>` : Set camera position (e.g., `--camera 0.0,0.0,-5.0`)
- `--particles <n>` : Set number of particles (for scenes with particles)
- `--output <filename>` : Set custom output filename

**How to Use:**
```bash
cargo run -- --scene plane_cube --width 1024 --height 768 --camera 0.0,2.0,-10.0 --output my_cube.ppm
```
*Flags are parsed using `clap` in `src/main.rs`.*

**Scene Examples:**
- `--scene all` : Render all objects
- `--scene plane_sphere` : Render plane and sphere
- `--scene plane_cube` : Render plane and cube
- `--scene plane_cylinder` : Render plane and cylinder
- `--scene all_particles` : Render all objects and particles
- `--scene particles` : Render only particles
- `--scene reflection` : Render scene with reflection
- `--scene refraction` : Render scene with refraction

**Camera Example:**
- `--camera 0.0,2.0,-10.0` : Sets camera position to (0.0, 2.0, -10.0)

**Particles Example:**
- `--scene all_particles --particles 200` : Renders 200 particles with all objects

**Output Example:**
- `--output custom_output.ppm` : Saves the result as `custom_output.ppm`

---

## 3. Particle Features

**Description:**
- The raytracer automatically generates random spheres as particles.
- These are added to the scene and rendered in outputs like `Output/output_all_particles.ppm`.

**Code Example:**
```rust
let particle_objects = generate_particles(100);
all_objects.extend(particle_objects);

let raytracer_all_particles = RayTracer { scene: &scene, objects: all_objects };
let pixels_all_particles = raytracer_all_particles.render(width, height);
```
*See the `generate_particles` function in `src/main.rs` for details.*

---

## 4. Fluid Simulation

**Note:**  
Fluid simulation is **not implemented**. Only particle features are available.

---
