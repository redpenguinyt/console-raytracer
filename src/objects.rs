use super::Colour;
use gemini_engine::elements3d::Vec3D;

pub struct RaySphere {
    pub centre: Vec3D,
    pub radius: f64,
    pub colour: Colour,
    pub specular: f64,
    pub reflective: f64,
}

impl RaySphere {
    pub const fn new(
        centre: Vec3D,
        radius: f64,
        colour: Colour,
        specular: f64,
        reflective: f64,
    ) -> Self {
        Self {
            centre,
            radius,
            colour,
            specular,
            reflective,
        }
    }
}

pub enum LightType {
    Ambient,
    Point { position: Vec3D },
    Directional { direction: Vec3D },
}

pub struct Light {
    pub light_type: LightType,
    pub intensity: f64,
}

impl Light {
    pub fn new_ambient(intensity: f64) -> Self {
        Self {
            light_type: LightType::Ambient,
            intensity,
        }
    }

    pub fn new_point(intensity: f64, position: Vec3D) -> Self {
        Self {
            light_type: LightType::Point { position },
            intensity,
        }
    }

    pub fn new_directional(intensity: f64, direction: Vec3D) -> Self {
        Self {
            light_type: LightType::Directional { direction },
            intensity,
        }
    }
}
