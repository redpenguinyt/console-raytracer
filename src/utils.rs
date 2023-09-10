use gemini_engine::elements3d::Vec3D;

pub fn dot(value: Vec3D, other: Vec3D) -> f64 {
    value.x * other.x + value.y * other.y + value.z * other.z
}
