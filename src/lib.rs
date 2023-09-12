use gemini_engine::elements::PixelContainer;
use gemini_engine::elements::{view::ColChar, Vec2D};
use gemini_engine::elements3d::Vec3D;
mod objects;
pub use gemini_engine::elements::view::colchar::Colour;
pub use objects::{Light, LightType, RaySphere};
mod ray;

pub struct RayScene {
    pub viewport_width: f64,
    pub viewport_height: f64,
    pub viewport_depth: f64,
    pub spheres: Vec<RaySphere>,
    pub lights: Vec<Light>,
}

impl RayScene {
    pub const fn new(
        viewport_width: f64,
        viewport_height: f64,
        viewport_depth: f64,
        spheres: Vec<RaySphere>,
        lights: Vec<Light>,
    ) -> Self {
        Self {
            viewport_width,
            viewport_height,
            viewport_depth,
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

    pub fn render(&self, canvas_size: Vec2D) -> PixelContainer {
        let mut container = PixelContainer::new();
        for x in 0..canvas_size.x as isize {
            for y in 0..canvas_size.y as isize {
                let canvas_point = Vec2D::new(x, canvas_size.y as isize - y - 1);
                // 2. Determine which square on the viewport corresponds to this pixel
                let view_pos =
                    self.canvas_to_viewport(Vec2D { x, y } - canvas_size / 2, canvas_size);

                // 3. Determine the colour seen through that square
                let colour = self.trace_ray(view_pos, 1.0, f64::INFINITY);

                // 4. Paint the pixel with that clour
                let fill_char = ColChar::SOLID.with_colour(colour);
                container.plot(canvas_point, fill_char);
            }
        }

        container
    }

    pub fn trace_ray(&self, direction: Vec3D, t_min: f64, t_max: f64) -> Colour {
        let (closest_sphere, closest_t) =
            ray::closest_intersection(&self.spheres, Vec3D::ZERO, direction, t_min, t_max);

        match closest_sphere {
            Some(sphere) => {
                let point = Vec3D::ZERO + direction * closest_t;
                let normal = point - sphere.centre;
                let normal = normal / normal.magnitude();

                sphere.colour * self.compute_lighting(point, normal, -direction, sphere.specular)
            }
            None => Colour::WHITE,
        }
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
                    let (light_direction, t_max) = match light.light_type {
                        LightType::Ambient => panic!("Ambience should have already been handled"),
                        LightType::Point { position } => (position - point, 1.0),
                        LightType::Directional { direction } => (direction, f64::INFINITY),
                    };

                    // Shadow check
                    let (shadow_sphere, _shadow_t) = ray::closest_intersection(
                        &self.spheres,
                        point,
                        light_direction,
                        0.001,
                        t_max,
                    );
                    if shadow_sphere.is_some() {
                        continue;
                    }

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
}
