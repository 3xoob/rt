// Core ray tracing logic: casting rays, intersection, lighting, pixel color calculation.

use crate::scene::Scene;
use crate::object::{Object, Vec3};

pub struct RayTracer<'a> {
    pub scene: &'a Scene,
    pub objects: Vec<Box<dyn Object>>,
}

impl<'a> RayTracer<'a> {
    pub fn trace_ray(&self, ray_origin: Vec3, ray_dir: Vec3, depth: u32) -> [u8; 3] {
        if depth == 0 {
            return [0, 0, 0];
        }
        let cam = &self.scene.camera;
        let mut closest_t = f64::INFINITY;
        let mut hit_record: Option<crate::object::HitRecord> = None;

        for obj in &self.objects {
            if let Some(rec) = obj.intersect(ray_origin, ray_dir) {
                if rec.t < closest_t {
                    closest_t = rec.t;
                    hit_record = Some(rec);
                }
            }
        }
        match hit_record {
            Some(rec) => {
                // Ambient + Lambertian diffuse + Phong specular shading
                let ambient = 0.1;
                let shininess = rec.material.shininess.max(32.0);
                let mut color = [
                    (rec.color[0] as f64 * ambient) as f64,
                    (rec.color[1] as f64 * ambient) as f64,
                    (rec.color[2] as f64 * ambient) as f64,
                ];
                for light in &self.scene.lights {
                    let light_dir = (light.position - rec.point).normalize();

                    // Shadow check
                    let shadow_origin = rec.point + rec.normal * 1e-4;
                    let mut in_shadow = false;
                    for obj in &self.objects {
                        if let Some(shadow_hit) = obj.intersect(shadow_origin, light_dir) {
                            let light_dist = (light.position - rec.point).norm();
                            if shadow_hit.t > 1e-4 && shadow_hit.t < light_dist {
                                in_shadow = true;
                                break;
                            }
                        }
                    }

                    if !in_shadow {
                        let diffuse = rec.normal.dot(light_dir).max(0.0) * light.intensity;
                        let view_dir = (cam.position - rec.point).normalize();
                        let reflect_dir = light_dir.reflect(rec.normal).normalize();
                        let specular = view_dir.dot(reflect_dir).max(0.0).powf(shininess);

                        color[0] += rec.color[0] as f64 * diffuse + 255.0 * specular;
                        color[1] += rec.color[1] as f64 * diffuse + 255.0 * specular;
                        color[2] += rec.color[2] as f64 * diffuse + 255.0 * specular;
                    }
                }

                // Reflection
                if rec.material.reflectivity > 0.0 {
                    let reflect_dir = ray_dir.reflect(rec.normal).normalize();
                    let reflect_origin = rec.point + rec.normal * 1e-4;
                    let reflect_color = self.trace_ray(reflect_origin, reflect_dir, depth - 1);
                    color[0] = (1.0 - rec.material.reflectivity) * color[0] + rec.material.reflectivity * reflect_color[0] as f64;
                    color[1] = (1.0 - rec.material.reflectivity) * color[1] + rec.material.reflectivity * reflect_color[1] as f64;
                    color[2] = (1.0 - rec.material.reflectivity) * color[2] + rec.material.reflectivity * reflect_color[2] as f64;
                }

                // Refraction (basic, no Fresnel)
                if rec.material.refractivity > 0.0 {
                    let n = rec.normal;
                    let eta = 1.0 / 1.5; // Assume fixed IOR
                    let cosi = -n.dot(ray_dir).max(-1.0).min(1.0);
                    let k = 1.0 - eta * eta * (1.0 - cosi * cosi);
                    if k >= 0.0 {
                        let refract_dir = ray_dir * eta + n * (eta * cosi - k.sqrt());
                        let refract_origin = rec.point - rec.normal * 1e-4;
                        let refract_color = self.trace_ray(refract_origin, refract_dir.normalize(), depth - 1);
                        color[0] = (1.0 - rec.material.refractivity) * color[0] + rec.material.refractivity * refract_color[0] as f64;
                        color[1] = (1.0 - rec.material.refractivity) * color[1] + rec.material.refractivity * refract_color[1] as f64;
                        color[2] = (1.0 - rec.material.refractivity) * color[2] + rec.material.refractivity * refract_color[2] as f64;
                    }
                }

                [
                    color[0].min(255.0) as u8,
                    color[1].min(255.0) as u8,
                    color[2].min(255.0) as u8,
                ]
            }
            None => [0, 0, 0],
        }
    }

    pub fn render(&self, width: usize, height: usize) -> Vec<[u8; 3]> {
        let mut pixels = Vec::with_capacity(width * height);
        let cam = &self.scene.camera;
        let aspect_ratio = width as f64 / height as f64;
        let fov_rad = (cam.fov.to_radians() / 2.0).tan();

        for y in 0..height {
            for x in 0..width {
                let px = ((2.0 * ((x as f64 + 0.5) / width as f64)) - 1.0) * aspect_ratio * fov_rad;
                let py = (1.0 - 2.0 * ((y as f64 + 0.5) / height as f64)) * fov_rad;
                let ray_dir = Vec3 { x: px, y: py, z: 1.0 }.normalize();
                let ray_origin = cam.position;
                let color = self.trace_ray(ray_origin, ray_dir, 3);
                pixels.push(color);
            }
        }
        pixels
    }
}
