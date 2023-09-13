use std::time::Instant;

use gemini_engine::elements::view::{ColChar, View, Wrapping};
use gemini_engine::elements3d::{Transform3D, Vec3D};
use raytracing::{Colour, Light, RayScene, RaySphere};

const VIEW_SIZE: (f64, f64) = (1.0, 1.0);
const VIEW_DEPTH: f64 = 1.0;

fn main() {
    let mut canvas = View::new(500, 170, ColChar::BACKGROUND);

    let scene = RayScene::new(
        VIEW_SIZE,
        VIEW_DEPTH,
        Transform3D::DEFAULT,
        // Transform3D::new_tr(Vec3D::new(3.0, 0.0, 0.0), Vec3D::new(0.0, 0.7, 0.0)),
        vec![
            RaySphere::new(
                Vec3D::new(0.0, -1.0, 3.0),
                1.0,
                Colour::rgb(255, 0, 0),
                500.0,
                0.2,
            ), // Red
            RaySphere::new(
                Vec3D::new(2.0, 0.0, 4.0),
                1.0,
                Colour::rgb(0, 0, 255),
                500.0,
                0.3,
            ), // Blue
            RaySphere::new(
                Vec3D::new(-2.0, 0.0, 4.0),
                1.0,
                Colour::rgb(0, 255, 0),
                10.0,
                0.4,
            ), // Green
            RaySphere::new(
                Vec3D::new(0.0, -5001.0, 0.0),
                5000.0,
                Colour::rgb(255, 255, 0),
                1000.0,
                0.1,
            ), // Yellow
        ],
        vec![
            Light::new_ambient(0.2),
            Light::new_point(0.6, Vec3D::new(2.0, 1.0, 0.0)),
            Light::new_directional(0.2, Vec3D::new(1.0, 4.0, 4.0)),
        ],
    );

    let now = Instant::now();
    canvas.blit(&scene.render(canvas.size()), Wrapping::Panic);
    let elapsed = now.elapsed();
    canvas.display_render().unwrap();
    println!("Elapsed: {}µs", elapsed.as_micros());
}
