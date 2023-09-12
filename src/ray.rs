use gemini_engine::elements3d::Vec3D;
use super::objects::RaySphere;

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