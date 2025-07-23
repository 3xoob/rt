// Entry point: sets up scene, objects, and renders to PPM.

mod scene;
mod object;
mod raytracer;
mod ppm;
mod material;

use std::fs::File;
use std::io::BufWriter;
use scene::{Camera, Light, Scene};
use object::{Sphere, Plane, Cube, Cylinder};
use raytracer::RayTracer;

// Add clap for CLI argument parsing
use clap::{Arg, Command};

fn main() {
    use object::Vec3;

    // Parse CLI arguments
    let matches = Command::new("Raytracer")
        .arg(Arg::new("textures")
            .long("textures")
            .help("Enable textures")
            .action(clap::ArgAction::SetTrue)
            .default_value("true"))
        .arg(Arg::new("reflection")
            .long("reflection")
            .help("Enable reflection")
            .action(clap::ArgAction::SetTrue)
            .default_value("true"))
        .arg(Arg::new("refraction")
            .long("refraction")
            .help("Enable refraction")
            .action(clap::ArgAction::SetTrue)
            .default_value("true"))
        .arg(Arg::new("width")
            .long("width")
            .help("Image width")
            .default_value("800"))
        .arg(Arg::new("height")
            .long("height")
            .help("Image height")
            .default_value("600"))
        .arg(Arg::new("scene")
            .long("scene")
            .help("Scene to render: all, plane_sphere, plane_cube, plane_cylinder, all_particles, particles, reflection, refraction")
            .default_value("all"))
        .arg(Arg::new("camera")
            .long("camera")
            .help("Camera position as x,y,z (e.g. --camera 0.0,0.0,-5.0)")
            .default_value("0.0,0.0,-5.0"))
        .arg(Arg::new("particles")
            .long("particles")
            .help("Number of particles")
            .default_value("100"))
        .arg(Arg::new("output")
            .long("output")
            .help("Output filename (default depends on scene)")
            .default_value(""))
        .get_matches();

    let enable_textures = matches.get_one::<bool>("textures").copied().unwrap_or(true);
    let enable_reflection = matches.get_one::<bool>("reflection").copied().unwrap_or(true);
    let enable_refraction = matches.get_one::<bool>("refraction").copied().unwrap_or(true);
    let width = matches.get_one::<String>("width").unwrap().parse::<usize>().unwrap_or(800);
    let height = matches.get_one::<String>("height").unwrap().parse::<usize>().unwrap_or(600);
    let scene_name = matches.get_one::<String>("scene").unwrap().as_str();
    let camera_str = matches.get_one::<String>("camera").unwrap();
    let camera_vals: Vec<f64> = camera_str.split(',').filter_map(|s| s.parse().ok()).collect();
    let camera_pos = if camera_vals.len() == 3 {
        Vec3 { x: camera_vals[0], y: camera_vals[1], z: camera_vals[2] }
    } else {
        Vec3 { x: 0.0, y: 0.0, z: -5.0 }
    };
    let particles_count = matches.get_one::<String>("particles").unwrap().parse::<usize>().unwrap_or(100);
    let output_filename = matches.get_one::<String>("output").unwrap();

    let camera = Camera {
        position: camera_pos,
        fov: 60.0,
    };
    let light = Light {
        position: Vec3 { x: 5.0, y: 5.0, z: -10.0 },
        intensity: 1.0,
    };
    let scene = Scene {
        camera,
        lights: vec![light.clone()],
    };

    let texture = if enable_textures {
        ppm::read_ppm("Output/solid.ppm").ok()
    } else {
        None
    };

    let sphere = Sphere {
        center: Vec3 { x: 0.0, y: 0.0, z: 0.0 },
        radius: 1.0,
        color: [255, 0, 0],
        material: material::Material {
            reflectivity: if enable_reflection { 0.7 } else { 0.0 },
            refractivity: if enable_refraction { 0.0 } else { 0.0 },
            shininess: 64.0,
            texture: texture.clone(),
        },
    };

    let plane = Plane {
        point: Vec3 { x: 0.0, y: -1.0, z: 0.0 },
        normal: Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        color: [200, 200, 200],
        material: material::Material {
            reflectivity: if enable_reflection { 0.0 } else { 0.0 },
            refractivity: if enable_refraction { 0.8 } else { 0.0 },
            shininess: 16.0,
            texture: if enable_textures { None } else { None },
        },
    };

    let cube = Cube {
        center: Vec3 { x: 2.0, y: 0.0, z: 0.0 },
        size: 1.0,
        color: [0, 255, 0],
        material: material::Material::default(),
    };

    let cylinder = Cylinder {
        base: Vec3 { x: -2.0, y: -1.0, z: 0.0 },
        axis: Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        radius: 0.5,
        height: 2.0,
        color: [0, 0, 255],
        material: material::Material::default(),
    };

    // Remove Output directory if it exists, then create it
    if std::path::Path::new("Output").exists() {
        std::fs::remove_dir_all("Output").expect("Failed to remove existing Output directory");
    }
    std::fs::create_dir_all("Output").expect("Failed to create Output directory");

    // Helper to get output filename
    fn get_output_filename(scene: &str, output_flag: &str) -> String {
        if !output_flag.is_empty() {
            output_flag.to_string()
        } else {
            match scene {
                "all" => "Output/output_all.ppm".to_string(),
                "plane_sphere" => "Output/output_plane_sphere.ppm".to_string(),
                "plane_cube" => "Output/output_plane_cube.ppm".to_string(),
                "plane_cylinder" => "Output/output_plane_cylinder.ppm".to_string(),
                "all_particles" => "Output/output_all_particles.ppm".to_string(),
                "particles" => "Output/output_particles.ppm".to_string(),
                "reflection" => "Output/reflection.ppm".to_string(),
                "refraction" => "Output/refraction.ppm".to_string(),
                _ => "Output/output_all.ppm".to_string(),
            }
        }
    }

    match scene_name {
        "all" => {
            let raytracer_all = RayTracer { scene: &scene, objects: vec![
                Box::new(sphere.clone()),
                Box::new(plane.clone()),
                Box::new(cube.clone()),
                Box::new(cylinder.clone()),
            ]};
            let pixels_all = raytracer_all.render(width, height);
            let file_all = File::create(get_output_filename("all", output_filename)).expect("Failed to create file");
            let mut writer_all = BufWriter::new(file_all);
            ppm::write_ppm(&mut writer_all, width, height, &pixels_all).expect("Failed to write PPM");
        }
        "plane_sphere" => {
            let raytracer_plane_sphere = RayTracer { scene: &scene, objects: vec![
                Box::new(plane.clone()),
                Box::new(sphere.clone()),
            ]};
            let pixels_plane_sphere = raytracer_plane_sphere.render(width, height);
            let file_plane_sphere = File::create(get_output_filename("plane_sphere", output_filename)).expect("Failed to create file");
            let mut writer_plane_sphere = BufWriter::new(file_plane_sphere);
            ppm::write_ppm(&mut writer_plane_sphere, width, height, &pixels_plane_sphere).expect("Failed to write PPM");
        }
        "plane_cube" => {
            let raytracer_plane_cube = RayTracer { scene: &scene, objects: vec![
                Box::new(plane.clone()),
                Box::new(cube.clone()),
            ]};
            let pixels_plane_cube = raytracer_plane_cube.render(width, height);
            let file_plane_cube = File::create(get_output_filename("plane_cube", output_filename)).expect("Failed to create file");
            let mut writer_plane_cube = BufWriter::new(file_plane_cube);
            ppm::write_ppm(&mut writer_plane_cube, width, height, &pixels_plane_cube).expect("Failed to write PPM");
        }
        "plane_cylinder" => {
            let raytracer_plane_cylinder = RayTracer { scene: &scene, objects: vec![
                Box::new(plane.clone()),
                Box::new(cylinder.clone()),
            ]};
            let pixels_plane_cylinder = raytracer_plane_cylinder.render(width, height);
            let file_plane_cylinder = File::create(get_output_filename("plane_cylinder", output_filename)).expect("Failed to create file");
            let mut writer_plane_cylinder = BufWriter::new(file_plane_cylinder);
            ppm::write_ppm(&mut writer_plane_cylinder, width, height, &pixels_plane_cylinder).expect("Failed to write PPM");
        }
        "all_particles" => {
            let particle_objects = generate_particles(particles_count);
            let mut all_objects: Vec<Box<dyn object::Object>> = vec![
                Box::new(sphere.clone()),
                Box::new(plane.clone()),
                Box::new(cube.clone()),
                Box::new(cylinder.clone()),
            ];
            all_objects.extend(particle_objects);
            let raytracer_all_particles = RayTracer { scene: &scene, objects: all_objects };
            let pixels_all_particles = raytracer_all_particles.render(width, height);
            let file_all_particles = File::create(get_output_filename("all_particles", output_filename)).expect("Failed to create file");
            let mut writer_all_particles = BufWriter::new(file_all_particles);
            ppm::write_ppm(&mut writer_all_particles, width, height, &pixels_all_particles).expect("Failed to write PPM");
        }
        "particles" => {
            let particle_objects = generate_particles(particles_count);
            let raytracer_particles = RayTracer { scene: &scene, objects: particle_objects };
            let pixels_particles = raytracer_particles.render(width, height);
            let file_particles = File::create(get_output_filename("particles", output_filename)).expect("Failed to create file");
            let mut writer_particles = BufWriter::new(file_particles);
            ppm::write_ppm(&mut writer_particles, width, height, &pixels_particles).expect("Failed to write PPM");
        }
        "reflection" => {
            let reflect_sphere = Sphere {
                center: sphere.center,
                radius: sphere.radius,
                color: sphere.color,
                material: material::Material {
                    reflectivity: 0.8,
                    refractivity: 0.0,
                    shininess: 64.0,
                    texture: None,
                },
            };
            let raytracer_reflection = RayTracer { scene: &scene, objects: vec![
                Box::new(reflect_sphere.clone()),
                Box::new(plane.clone()),
                Box::new(cube.clone()),
                Box::new(cylinder.clone()),
            ]};
            let pixels_reflection = raytracer_reflection.render(width, height);
            let file_reflection = File::create(get_output_filename("reflection", output_filename)).expect("Failed to create file");
            let mut writer_reflection = BufWriter::new(file_reflection);
            ppm::write_ppm(&mut writer_reflection, width, height, &pixels_reflection).expect("Failed to write PPM");
        }
        "refraction" => {
            let refract_plane = Plane {
                point: plane.point,
                normal: plane.normal,
                color: plane.color,
                material: material::Material {
                    reflectivity: 0.0,
                    refractivity: 0.9,
                    shininess: 16.0,
                    texture: None,
                },
            };
            let raytracer_refraction = RayTracer { scene: &scene, objects: vec![
                Box::new(sphere.clone()),
                Box::new(refract_plane.clone()),
                Box::new(cube.clone()),
                Box::new(cylinder.clone()),
            ]};
            let pixels_refraction = raytracer_refraction.render(width, height);
            let file_refraction = File::create(get_output_filename("refraction", output_filename)).expect("Failed to create file");
            let mut writer_refraction = BufWriter::new(file_refraction);
            ppm::write_ppm(&mut writer_refraction, width, height, &pixels_refraction).expect("Failed to write PPM");
        }
        _ => {
            // Default: render all
            let raytracer_all = RayTracer { scene: &scene, objects: vec![
                Box::new(sphere.clone()),
                Box::new(plane.clone()),
                Box::new(cube.clone()),
                Box::new(cylinder.clone()),
            ]};
            let pixels_all = raytracer_all.render(width, height);
            let file_all = File::create(get_output_filename("all", output_filename)).expect("Failed to create file");
            let mut writer_all = BufWriter::new(file_all);
            ppm::write_ppm(&mut writer_all, width, height, &pixels_all).expect("Failed to write PPM");
        }
    }
}

// Particle system: generate random spheres
fn generate_particles(n: usize) -> Vec<Box<dyn object::Object>> {
    use rand::Rng;
    let mut rng = rand::rng();
    let mut particles: Vec<Box<dyn object::Object>> = Vec::new();
    for _ in 0..n {
        let radius = rng.random_range(0.05..0.2);
        let center = object::Vec3 {
            x: rng.random_range(-3.0..3.0),
            y: -1.0 + radius, // Move sphere fully above the plane
            z: rng.random_range(-3.0..3.0),
        };
        let color = [
            rng.random_range(0..=255),
            rng.random_range(0..=255),
            rng.random_range(0..=255),
        ];
        let sphere = object::Sphere {
            center,
            radius,
            color,
            material: material::Material {
                reflectivity: 0.0,
                refractivity: 0.0,
                shininess: 8.0,
                texture: None,
            },
        };
        particles.push(Box::new(sphere) as Box<dyn object::Object>);
    }
    particles
}
