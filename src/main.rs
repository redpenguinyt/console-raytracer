use gemini_engine::elements::view::{ColChar, View, Wrapping};
use gemini_engine::elements3d::Vec3D;
use raytracing::{Colour, Light, RayScene, RaySphere};

const VIEW_WIDTH: f64 = 1.0;
const VIEW_HEIGHT: f64 = 1.0;
const VIEW_DEPTH: f64 = 1.0;

fn main() {
    let mut canvas = View::new(500, 170, ColChar::BACKGROUND);

    let scene = RayScene::new(
        VIEW_WIDTH,
        VIEW_HEIGHT,
        VIEW_DEPTH,
        Vec3D::ZERO,
        vec![
            RaySphere::new(
                Vec3D::new(0.0, -1.0, 3.0),
                1.0,
                Colour::rgb(255, 0, 0),
                500.0,
            ), // Red
            RaySphere::new(
                Vec3D::new(2.0, 0.0, 4.0),
                1.0,
                Colour::rgb(0, 0, 255),
                500.0,
            ), // Blue
            RaySphere::new(
                Vec3D::new(-2.0, 0.0, 4.0),
                1.0,
                Colour::rgb(0, 255, 0),
                10.0,
            ), // Green
            RaySphere::new(
                Vec3D::new(0.0, -5001.0, 0.0),
                5000.0,
                Colour::rgb(255, 255, 0),
                1000.0,
            ), // Yellow
        ],
        vec![
            Light::new_ambient(0.2),
            Light::new_point(0.6, Vec3D::new(2.0, 1.0, 0.0)),
            Light::new_directional(0.2, Vec3D::new(1.0, 4.0, 4.0)),
        ],
    );

    canvas.blit(&scene.render(canvas.size()), Wrapping::Panic);

    canvas.display_render().unwrap();
}
