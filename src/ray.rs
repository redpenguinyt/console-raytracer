use super::objects::RaySphere;
use gemini_engine::elements3d::Vec3D;

pub fn reflect_ray(ray: Vec3D, normal: Vec3D) -> Vec3D {
    normal * 2.0 * normal.dot(ray) - ray
}

pub fn intersect_ray_sphere(origin: Vec3D, direction: Vec3D, sphere: &RaySphere) -> (f64, f64) {
    let r = sphere.radius;
    let co = origin - sphere.centre;

    let a = direction.dot(direction);
    let b = 2.0 * co.dot(direction);
    let c = co.dot(co) - (r * r);

    let discriminant = (b * b) - (4.0 * a * c);
    if discriminant < 0.0 {
        return (f64::INFINITY, f64::INFINITY);
    }

    let t1 = (-b + discriminant.sqrt()) / (2.0 * a);
    let t2 = (-b - discriminant.sqrt()) / (2.0 * a);
    (t1, t2)
}

pub fn is_intersection(spheres: &[RaySphere], origin: Vec3D, direction: Vec3D, t_min: f64, t_max: f64) -> bool {

    for sphere in spheres {
        let (t1, t2) = intersect_ray_sphere(origin, direction, sphere);

        if (t_min..t_max).contains(&t1) {
            return true;
        }
        if (t_min..t_max).contains(&t2) {
            return true;
        }
    }

    false
}

pub fn closest_intersection(
    spheres: &[RaySphere],
    origin: Vec3D,
    direction: Vec3D,
    t_min: f64,
    t_max: f64,
) -> (Option<&RaySphere>, f64) {
    let mut closest_t = f64::INFINITY;
    let mut closest_sphere = None;

    for sphere in spheres {
        let (t1, t2) = intersect_ray_sphere(origin, direction, sphere);

        if (t_min..t_max).contains(&t1) && t1 < closest_t {
            closest_t = t1;
            closest_sphere = Some(sphere);
        }
        if (t_min..t_max).contains(&t2) && t2 < closest_t {
            closest_t = t2;
            closest_sphere = Some(sphere);
        }
    }

    (closest_sphere, closest_t)
}
