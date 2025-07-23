// Object traits and implementations for sphere, cube, plane, and cylinder.

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn dot(self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }


    pub fn norm(self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(self) -> Vec3 {
        let n = self.norm();
        Vec3 {
            x: self.x / n,
            y: self.y / n,
            z: self.z / n,
        }
    }

    pub fn reflect(self, normal: Vec3) -> Vec3 {
        self - normal * 2.0 * self.dot(normal)
    }
}

// Implement basic arithmetic for Vec3
use std::ops::{Add, Sub, Mul, Div};

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

use crate::material::Material;

pub struct HitRecord {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
    pub color: [u8; 3],
    pub material: Material,
}

pub trait Object {
    fn intersect(&self, ray_origin: Vec3, ray_dir: Vec3) -> Option<HitRecord>;
}

#[derive(Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub color: [u8; 3],
    pub material: Material,
}

impl Object for Sphere {
    fn intersect(&self, ray_origin: Vec3, ray_dir: Vec3) -> Option<HitRecord> {
        let oc = ray_origin - self.center;
        let a = ray_dir.dot(ray_dir);
        let b = 2.0 * oc.dot(ray_dir);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            None
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            let t = if t1 > 0.0 { t1 } else { t2 };
            if t > 0.0 {
                let point = ray_origin + ray_dir * t;
                let normal = (point - self.center).normalize();

                // Texture mapping
                let mut color = self.color;
                if let Some(ref texture) = self.material.texture {
                    // Spherical UV mapping
                    let p = (point - self.center) / self.radius;
                    let u = 0.5 + (p.z.atan2(p.x)) / (2.0 * std::f64::consts::PI);
                    let v = 0.5 - (p.y.asin()) / std::f64::consts::PI;
                    let tx = (u * (texture.width as f64)) as usize % texture.width;
                    let ty = (v * (texture.height as f64)) as usize % texture.height;
                    color = texture.pixels[ty * texture.width + tx];
                }

                Some(HitRecord {
                    t,
                    point,
                    normal,
                    color,
                    material: self.material.clone(),
                })
            } else {
                None
            }
        }
    }
}

#[derive(Clone)]
pub struct Plane {
    pub point: Vec3,
    pub normal: Vec3,
    pub color: [u8; 3],
    pub material: Material,
}

impl Object for Plane {
    fn intersect(&self, ray_origin: Vec3, ray_dir: Vec3) -> Option<HitRecord> {
        let denom = self.normal.dot(ray_dir);
        if denom.abs() > 1e-6 {
            let t = (self.point - ray_origin).dot(self.normal) / denom;
            if t > 1e-4 {
                let point = ray_origin + ray_dir * t;
                let normal = self.normal.normalize();

                // Texture mapping
                let mut color = self.color;
                if let Some(ref texture) = self.material.texture {
                    // Planar UV mapping (x/z axes)
                    let u = (point.x - self.point.x).fract();
                    let v = (point.z - self.point.z).fract();
                    let tx = (u.abs() * (texture.width as f64)) as usize % texture.width;
                    let ty = (v.abs() * (texture.height as f64)) as usize % texture.height;
                    color = texture.pixels[ty * texture.width + tx];
                }

                return Some(HitRecord {
                    t,
                    point,
                    normal,
                    color,
                    material: self.material.clone(),
                });
            }
        }
        None
    }
}

#[derive(Clone)]
pub struct Cube {
    pub center: Vec3,
    pub size: f64,
    pub color: [u8; 3],
    pub material: Material,
}

impl Object for Cube {
    fn intersect(&self, ray_origin: Vec3, ray_dir: Vec3) -> Option<HitRecord> {
        let half = self.size / 2.0;
        let min = self.center - Vec3 { x: half, y: half, z: half };
        let max = self.center + Vec3 { x: half, y: half, z: half };

        let mut tmin = (min.x - ray_origin.x) / ray_dir.x;
        let mut tmax = (max.x - ray_origin.x) / ray_dir.x;
        if tmin > tmax { std::mem::swap(&mut tmin, &mut tmax); }

        let mut tymin = (min.y - ray_origin.y) / ray_dir.y;
        let mut tymax = (max.y - ray_origin.y) / ray_dir.y;
        if tymin > tymax { std::mem::swap(&mut tymin, &mut tymax); }

        if (tmin > tymax) || (tymin > tmax) {
            return None;
        }
        if tymin > tmin { tmin = tymin; }
        if tymax < tmax { tmax = tymax; }

        let mut tzmin = (min.z - ray_origin.z) / ray_dir.z;
        let mut tzmax = (max.z - ray_origin.z) / ray_dir.z;
        if tzmin > tzmax { std::mem::swap(&mut tzmin, &mut tzmax); }

        if (tmin > tzmax) || (tzmin > tmax) {
            return None;
        }
        if tzmin > tmin { tmin = tzmin; }

        let t = if tmin > 1e-4 { tmin } else { tmax };
        if t < 1e-4 {
            return None;
        }

        let point = ray_origin + ray_dir * t;
        // Compute normal based on which slab was hit
        let mut normal = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
        let eps = 1e-4;
        if (point.x - min.x).abs() < eps { normal = Vec3 { x: -1.0, y: 0.0, z: 0.0 }; }
        else if (point.x - max.x).abs() < eps { normal = Vec3 { x: 1.0, y: 0.0, z: 0.0 }; }
        else if (point.y - min.y).abs() < eps { normal = Vec3 { x: 0.0, y: -1.0, z: 0.0 }; }
        else if (point.y - max.y).abs() < eps { normal = Vec3 { x: 0.0, y: 1.0, z: 0.0 }; }
        else if (point.z - min.z).abs() < eps { normal = Vec3 { x: 0.0, y: 0.0, z: -1.0 }; }
        else if (point.z - max.z).abs() < eps { normal = Vec3 { x: 0.0, y: 0.0, z: 1.0 }; }

        Some(HitRecord {
            t,
            point,
            normal,
            color: self.color,
            material: self.material.clone(),
        })
    }
}

#[derive(Clone)]
pub struct Cylinder {
    pub base: Vec3,
    pub axis: Vec3,
    pub radius: f64,
    pub height: f64,
    pub color: [u8; 3],
    pub material: Material,
}

impl Object for Cylinder {
    fn intersect(&self, ray_origin: Vec3, ray_dir: Vec3) -> Option<HitRecord> {
        let axis = self.axis.normalize();
        let oc = ray_origin - self.base;
        let d_dot_a = ray_dir.dot(axis);
        let oc_dot_a = oc.dot(axis);

        let d_perp = ray_dir - axis * d_dot_a;
        let oc_perp = oc - axis * oc_dot_a;

        let a = d_perp.dot(d_perp);
        let b = 2.0 * d_perp.dot(oc_perp);
        let c = oc_perp.dot(oc_perp) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_disc = discriminant.sqrt();
        let t1 = (-b - sqrt_disc) / (2.0 * a);
        let t2 = (-b + sqrt_disc) / (2.0 * a);

        let mut t = None;
        for &ti in &[t1, t2] {
            if ti > 1e-4 {
                let hit = ray_origin + ray_dir * ti;
                let proj = (hit - self.base).dot(axis);
                if proj >= 0.0 && proj <= self.height {
                    t = Some(ti);
                    break;
                }
            }
        }

        if let Some(t_hit) = t {
            let point = ray_origin + ray_dir * t_hit;
            let proj = (point - self.base).dot(axis);
            let center = self.base + axis * proj;
            let normal = (point - center).normalize();
            return Some(HitRecord {
                t: t_hit,
                point,
                normal,
                color: self.color,
                material: self.material.clone(),
            });
        }
        None
    }
}
