use gemini_engine::elements::view::{ColChar, Colour, View, Wrapping};
use gemini_engine::elements3d::{Transform3D, Vec3D};
use raytracer::{Light, RayScene, RaySphere};

const VIEW_SIZE: (f64, f64) = (1.0, 1.0);
const VIEW_DEPTH: f64 = 1.0;

fn main() {
    let mut canvas = View::new(500, 170, ColChar::BACKGROUND);

    let scene = RayScene::new(
        VIEW_SIZE,
        VIEW_DEPTH,
        Transform3D::new_tr(Vec3D::ZERO, Vec3D::new(-0.2, 0.0, 0.0)),
        // Transform3D::new_tr(Vec3D::new(3.0, 0.0, 0.0), Vec3D::new(0.0, 0.7, 0.0)),
        vec![
            RaySphere::new(
                Vec3D::new(0.0, -1.0, 3.0), // Position
                1.0,                        // Radius
                Colour::rgb(255, 0, 0),     // Colour (red)
                500.0,                      // Specular (in this case, quite shiny)
                0.2,                        // Reflectiveness (a value from 0 to 1)
            ),
            RaySphere::new(
                Vec3D::new(2.0, 0.0, 4.0),
                1.0,
                Colour::rgb(0, 0, 255), // Blue
                500.0,
                0.3,
            ),
            RaySphere::new(
                Vec3D::new(-2.0, 0.0, 4.0),
                1.0,
                Colour::rgb(0, 255, 0), // Green
                10.0,                   // somewhat shiny
                0.4,
            ),
            RaySphere::new(
                Vec3D::new(0.0, -5001.0, 0.0),
                5000.0,
                Colour::rgb(255, 255, 0), // Yellow
                1000.0,                   // very shiny
                0.1,
            ),
            RaySphere::new(Vec3D::new(0.0, 3.0, 5.0), 2.0, Colour::WHITE, 2.0, 0.7),
        ],
        vec![
            Light::new_ambient(0.2),
            Light::new_point(0.6, Vec3D::new(2.0, 1.0, 0.0)),
            Light::new_directional(0.2, Vec3D::new(1.0, 4.0, 4.0)),
        ],
    );

    // Render all the 3D stuff
    canvas.blit(&scene.render(canvas.size()), Wrapping::Panic);

    // Display the result
    canvas.display_render().expect("Failed to render view");
}
