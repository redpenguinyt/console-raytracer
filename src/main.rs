use gemini_engine::elements::view::{ColChar, Colour, View, Wrapping};
use gemini_engine::elements3d::{Transform3D, Vec3D};
use gemini_engine::fps_gameloop;
use raytracing::{Light, RayScene, RaySphere};

const VIEW_SIZE: (f64, f64) = (1.0, 1.0);
const VIEW_DEPTH: f64 = 1.0;

fn main() {
    let mut canvas = View::new(500, 170, ColChar::BACKGROUND);

    let mut scene = RayScene::new(
        VIEW_SIZE,
        VIEW_DEPTH,
        Transform3D::new_tr(Vec3D::ZERO, Vec3D::new(-0.2, 0.0, 0.0)),
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
            RaySphere::new(Vec3D::new(0.0, 3.0, 5.0), 2.0, Colour::WHITE, 10.0, 0.7),
        ],
        vec![
            Light::new_ambient(0.2),
            Light::new_point(0.6, Vec3D::new(2.0, 1.0, 0.0)),
            Light::new_directional(0.2, Vec3D::new(1.0, 4.0, 4.0)),
        ],
    );

    fps_gameloop!(
        {
            scene.spheres[1].centre = Transform3D::new_tr(Vec3D::ZERO, Vec3D::new(0.0, 0.02, 0.0))
                .rotate(scene.spheres[1].centre);
        },
        {
            canvas.blit(&scene.render(canvas.size()), Wrapping::Panic);
            canvas.display_render().unwrap();
            break;
        },
        30,
        |elapsed, _| println!("Elapsed: {:.2?}µs", elapsed)
    );
}
