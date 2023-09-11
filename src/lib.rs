use gemini_engine::elements::Vec2D;
use gemini_engine::elements3d::Vec3D;
mod colour;
mod objects;
pub use colour::Colour;
pub use objects::{Light, LightType, RaySphere};

pub struct RayScene {
    pub viewport_width: f64,
    pub viewport_height: f64,
    pub viewport_depth: f64,
    pub origin: Vec3D,
    pub spheres: Vec<RaySphere>,
    pub lights: Vec<Light>,
}

impl RayScene {
    pub const fn new(
        viewport_width: f64,
        viewport_height: f64,
        viewport_depth: f64,
        origin: Vec3D,
        spheres: Vec<RaySphere>,
        lights: Vec<Light>,
    ) -> Self {
        Self {
            viewport_width,
            viewport_height,
            viewport_depth,
            origin,
            spheres,
            lights,
        }
    }

    pub fn canvas_to_viewport(&self, pos: Vec2D, canvas_size: Vec2D) -> Vec3D {
        Vec3D::new(
            pos.x as f64 * self.viewport_width / canvas_size.x as f64,
            pos.y as f64 * self.viewport_height / canvas_size.y as f64,
            self.viewport_depth,
        )
    }

    pub fn compute_lighting(
        &self,
        point: Vec3D,
        normal: Vec3D,
        towards_view: Vec3D,
        specular: f64,
    ) -> f64 {
        let mut i = 0.0;

        for light in &self.lights {
            match light.light_type {
                LightType::Ambient => i += light.intensity,
                _ => {
                    let light_direction = match light.light_type {
                        LightType::Ambient => panic!("Ambience should have already been handled"),
                        LightType::Point { position } => position - point,
                        LightType::Directional { direction } => direction,
                    };

                    // Diffuse
                    let n_dot_l = normal.dot(light_direction);
                    if n_dot_l > 0.0 {
                        i += light.intensity * n_dot_l
                            / (normal.magnitude() * light_direction.magnitude());
                    }

                    // Specular
                    if specular != -1.0 {
                        let reflected_ray =
                            normal * 2.0 * normal.dot(light_direction) - light_direction;

                        let r_dot_v = reflected_ray.dot(towards_view);

                        if r_dot_v > 0.0 {
                            i += light.intensity
                                * (r_dot_v
                                    / (reflected_ray.magnitude() * towards_view.magnitude()))
                                .powf(specular);
                        }
                    }
                }
            }
        }

        i
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

        match closest_sphere {
            Some(sphere) => {
                let point = self.origin + view_pos * closest_t;
                let normal = point - sphere.centre;
                let normal = normal / normal.magnitude();

                sphere.colour * self.compute_lighting(point, normal, -view_pos, sphere.specular)
            }
            None => Colour::WHITE,
        }
    }

    fn intersect_ray_sphere(&self, view_pos: Vec3D, sphere: &RaySphere) -> (f64, f64) {
        let r = sphere.radius;
        let co = self.origin - sphere.centre;

        let a = view_pos.dot(view_pos);
        let b = 2.0 * co.dot(view_pos);
        let c = co.dot(co) - (r * r);

        let discriminant = (b * b) - (4.0 * a * c);
        if discriminant < 0.0 {
            return (f64::INFINITY, f64::INFINITY);
        }

        let t1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b - discriminant.sqrt()) / (2.0 * a);
        (t1, t2)
    }
}
