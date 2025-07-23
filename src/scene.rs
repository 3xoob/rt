// Scene, camera, and light definitions will go here.

use crate::object::Vec3;

#[derive(Clone)]
pub struct Camera {
    pub position: Vec3,
    pub fov: f64,
}

#[derive(Clone)]
pub struct Light {
    pub position: Vec3,
    pub intensity: f64,
}

pub struct Scene {
    pub camera: Camera,
    pub lights: Vec<Light>,
}
