use gemini_engine::elements::Vec2D;
use gemini_engine::elements3d::Vec3D;
mod colour;
pub use colour::Colour;

fn dot_product(value: Vec3D, other: Vec3D) -> f64 {
    value.x * other.x + value.y * other.y + value.z * other.z
}

pub struct RaySphere {
    pub centre: Vec3D,
    pub radius: f64,
    pub colour: Colour,
}

impl RaySphere {
    pub const EMPTY: Self = Self::new(Vec3D::ZERO, 0.0, Colour::WHITE);

    pub const fn new(centre: Vec3D, radius: f64, colour: Colour) -> Self {
        Self {
            centre,
            radius,
            colour,
        }
    }
}

pub struct RayScene {
    pub viewport_width: f64,
    pub viewport_height: f64,
    pub viewport_depth: f64,
    pub origin: Vec3D,
    pub spheres: Vec<RaySphere>,
}

impl RayScene {
    pub const fn new(
        viewport_width: f64,
        viewport_height: f64,
        viewport_depth: f64,
        origin: Vec3D,
        spheres: Vec<RaySphere>,
    ) -> Self {
        Self {
            viewport_width,
            viewport_height,
            viewport_depth,
            origin,
            spheres,
        }
    }

    pub fn canvas_to_viewport(&self, pos: Vec2D, canvas_size: Vec2D) -> Vec3D {
        Vec3D::new(
            pos.x as f64 * self.viewport_width / canvas_size.x as f64,
            pos.y as f64 * self.viewport_height / canvas_size.y as f64,
            self.viewport_depth,
        )
    }

    pub fn trace_ray(&self, view_pos: Vec3D, t_min: f64, t_max: f64) -> Colour {
        let mut closest_t = f64::INFINITY;
        let mut closest_sphere = None;

        for sphere in &self.spheres {
            let (t1, t2) = self.intersect_ray_sphere(view_pos, sphere);

            if (t_min..t_max).contains(&t1) && t1 < closest_t {
                closest_t = t1;
                closest_sphere = Some(sphere);
            }
            if (t_min..t_max).contains(&t2) && t2 < closest_t {
                closest_t = t2;
                closest_sphere = Some(sphere);
            }
        }

        closest_sphere.unwrap_or(&RaySphere::EMPTY).colour
    }

    fn intersect_ray_sphere(&self, view_pos: Vec3D, sphere: &RaySphere) -> (f64, f64) {
        let r = sphere.radius;
        let co = self.origin - sphere.centre;

        let a = dot_product(view_pos, view_pos);
        let b = 2.0 * dot_product(co, view_pos);
        let c = dot_product(co, co) - (r * r);

        let discriminant = (b * b) - (4.0 * a * c);
        if discriminant < 0.0 {
            return (f64::INFINITY, f64::INFINITY);
        }

        let t1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b - discriminant.sqrt()) / (2.0 * a);
        (t1, t2)
    }
}
