use crate::ppm::Texture;

// Material properties for objects (color, reflectivity, etc.)
#[derive(Clone)]
pub struct Material {
    pub reflectivity: f64,
    pub refractivity: f64,
    pub shininess: f64,
    pub texture: Option<Texture>,
}

impl Default for Material {
    fn default() -> Self {
        Material {
            reflectivity: 0.0,
            refractivity: 0.0,
            shininess: 0.0,
            texture: None,
        }
    }
}
